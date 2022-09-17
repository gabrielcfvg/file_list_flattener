
use std::ffi::OsString;
use std::io::Write;



pub struct TmpFilesystem {

    path: std::path::PathBuf
}

impl<'a> TmpFilesystem {

    pub fn new(dir_builder: DirBuilder) -> Self {

        let name = Self::build_unique_name();
        let path = std::env::temp_dir().as_path().join(name);

        std::fs::create_dir(&path).expect("TmpFilesystem creation failed");
    
        if let anyhow::Result::Err(err) = dir_builder.build(&path) {

            panic!("DirBuilder building failed during TmpFilesystem creation, error: {}", err);
        }

        return Self{path};
    }

    pub fn path(&'a self) -> &'a std::path::Path {

        return &self.path;
    }


    fn build_unique_name() -> String {

        let process_id = std::process::id();
        let date = Self::get_date();
        let internal_id = Self::get_internal_id();

        return format!("tmpfilesystem_{}_{}_{}", process_id, date, internal_id);
    }

    fn get_internal_id() -> u64 {

        use std::sync::atomic::AtomicU64;
        use std::sync::atomic::Ordering;

        static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

        // get the current value and then increment it to make sure it will not get taken again
        return ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    fn get_date() -> String {

        let now = std::time::SystemTime::now();
        let since_unix_epoch = now.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();

        return since_unix_epoch.as_nanos().to_string();
    }
}

impl Drop for TmpFilesystem {
    
    fn drop(&mut self) {
        
        assert!(self.path.exists());
        assert!(self.path.is_dir());

        std::fs::remove_dir_all(&self.path).expect(&format!("TmpFilesystem mount directory deletion failed, path: {}", self.path.display()));
    }
}



#[derive(Clone)]
pub struct DirBuilder {

    name: OsString,
    child_dirs: Vec<DirBuilder>,
    child_files: Vec<FileBuilder>
}

impl DirBuilder {
    
    pub fn new(name: impl std::convert::Into<OsString>) -> Self {

        return Self{name: name.into(), child_dirs: vec![], child_files: vec![]};
    }

    pub fn add_dir(mut self, new_dir: DirBuilder) -> Self {

        self.child_dirs.push(new_dir);
        return self;
    }

    pub fn add_file(mut self, new_file: FileBuilder) -> Self {

        self.child_files.push(new_file);
        return self;
    }

    pub fn build(self, path: &std::path::Path) -> anyhow::Result<()> {

        // create directory
        let path = path.join(self.name);
        std::fs::create_dir(&path)?;

        // build files
        self.child_files.into_iter().map(|file| file.build(&path)).collect::<anyhow::Result<()>>()?;

        // build directories
        self.child_dirs.into_iter().map(|dir| dir.build(&path)).collect::<anyhow::Result<()>>()?;

        return anyhow::Result::Ok(());
    }
}



#[derive(Clone)]
pub struct FileBuilder {

    name: OsString,
    content: Option<Vec<u8>>
}

impl FileBuilder {

    pub fn new_empty(name: impl std::convert::Into<OsString>) -> Self {

        return Self{name: name.into(), content: None};
    }

    pub fn new(name: impl std::convert::Into<OsString>, content: Vec<u8>) -> Self {

        return Self{name: name.into(), content: Some(content)};
    }

    pub fn new_gitignore(content: &[&str]) -> Self {

        let content_line_list = content.iter().map(|line| String::from(*line));
        let content_string = content_line_list.reduce(|l1, l2| format!("{}\n{}", l1, l2)).unwrap_or(String::new());
        let content_bytes = content_string.into_bytes();

        return Self::new(".gitignore", content_bytes);
    }

    pub fn build(self, path: &std::path::Path) -> anyhow::Result<()> {
    
        // create file
        let path = path.join(self.name);
        let mut file = std::fs::File::create(path)?;

        // write content if needed
        if let Some(content) = self.content {

            file.write(&content)?;
        }

        return anyhow::Result::Ok(());
    }
}



#[cfg(test)]
mod test_filesystem_builder {

    use super::*;


    impl DirBuilder {

        fn check(&self, path: &std::path::Path) -> bool {

            let path = path.join(self.name.clone());
            
            if (path.exists() && path.is_dir()) == false {

                return false;
            }

            let files_check = self.child_files.iter().map(|file| file.check(&path)).reduce(|c1, c2| c1 && c2).unwrap_or(true);
            let dirs_check = self.child_dirs.iter().map(|dir| dir.check(&path)).reduce(|c1, c2| c1 && c2).unwrap_or(true);

            return files_check && dirs_check;
        }
    }

    impl FileBuilder {

        fn check(&self, path: &std::path::Path) -> bool {
            
            let path = path.join(self.name.clone());

            if (path.exists() && path.is_file()) == false {

                return false;
            }

            if let Some(ref content) = self.content {

                let file_content = std::fs::read_to_string(&path).unwrap().into_bytes();

                if file_content != *content {

                    return false;
                }
            }

            return true;
        }
    }


    #[test]
    fn test_tmpfilesystem() {

        let dir_builder = DirBuilder::new("dir");
        let filesystem = TmpFilesystem::new(dir_builder.clone());
    
        // check filesystem
        assert!(filesystem.path().exists());
        assert!(filesystem.path().is_dir());
        
        // check filesystem structure
        assert!(dir_builder.check(filesystem.path()));

        let path = filesystem.path().to_owned();
        std::mem::drop(filesystem);

        assert!(path.exists() == false);
    }

    #[test]
    fn test_empty_dir_builder() {
    
        const DIR_NAME: &'static str = "dir";

        let dir_builder = DirBuilder::new(DIR_NAME);
        let filesystem = TmpFilesystem::new(dir_builder);

        let dir_path = filesystem.path().join(DIR_NAME);
        
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        // asserts that the directory is empty
        assert!(walkdir::WalkDir::new(dir_path).min_depth(1).into_iter().next().is_none());
    }

    #[test]
    fn test_dir_builder() {

        let dir_builder = DirBuilder::new("dir")
            .add_dir(DirBuilder::new("subdir1"))
            .add_dir(DirBuilder::new("subdir2"))
            .add_file(FileBuilder::new_empty("file1"))
            .add_file(FileBuilder::new_empty("file2"));

        let filesystem = TmpFilesystem::new(dir_builder.clone());

        assert!(dir_builder.check(filesystem.path()));
    }

    #[test]
    fn test_file_content() {

        let dir_builder = DirBuilder::new("dir")
            .add_file(FileBuilder::new_empty("file1"))
            .add_file(FileBuilder::new("file2", "binary_content".to_owned().into_bytes()))
            .add_file(FileBuilder::new_gitignore(&["ignore_pattern1", "ignore_pattern2"]));

        let filesystem = TmpFilesystem::new(dir_builder.clone());

        assert!(dir_builder.check(filesystem.path()));
    }
}
