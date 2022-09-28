
use super::template;
use super::template::Visitor;



pub struct Builder;

impl template::Visitor for Builder {

    type DirParameter<'a> = &'a std::path::Path;
    type DirReturnType = anyhow::Result<()>;

    fn visit_dir<'a>(&self, dir: &template::Dir, path: Self::DirParameter<'a>) -> Self::DirReturnType {
        
        // create directory
        let path = path.join(&dir.name);
        std::fs::create_dir(&path)?;

        // build files
        dir.child_files.iter().try_for_each(|file| file.visit(self, &path))?;

        // build directories
        dir.child_dirs.iter().try_for_each(|dir| dir.visit(self, &path))?;

        return anyhow::Result::Ok(());
    }


    type FileParameter<'a> = &'a std::path::Path;
    type FileReturnType = anyhow::Result<()>;

    fn visit_file<'a>(&self, file: &template::File, path: Self::FileParameter<'a>) -> Self::FileReturnType {
        
        use std::io::Write;

        // create file
        let path = path.join(&file.name);
        let mut _file = std::fs::File::create(path)?;
    
        // write content if needed
        if let Some(ref content) = file.content {
            
            _file.write_all(content)?;
        }
        
        return anyhow::Result::Ok(());
    }
}

impl Builder {

    fn new() -> Self {

        return Self{};
    }

    pub fn build_dir(path: &std::path::Path, dir: &template::Dir) -> <Self as template::Visitor>::DirReturnType {

        return Self::new().visit_dir(dir, path);
    }

    pub fn build_file(path: &std::path::Path, file: &template::File) -> <Self as template::Visitor>::FileReturnType {

        return Self::new().visit_file(file, path);
    }
}

#[test]
fn test_empty_dir_builder() {

    use template::Dir;
    use super::tmp_filesystem::TmpFilesystem;

    const DIR_NAME: &'static str = "dir";

    let dir_template = Dir::new(DIR_NAME);
    let filesystem = TmpFilesystem::new(&dir_template);

    let dir_path = filesystem.path().join(DIR_NAME);
    
    assert!(dir_path.exists());
    assert!(dir_path.is_dir());

    // asserts that the directory is empty
    assert!(std::fs::read_dir(dir_path).unwrap_or_else(|err| panic!("unexpected walk error, error: {}", err)).into_iter().next().is_none());
}

#[test]
fn test_dir_builder() {

    use template::{Dir, File};
    use super::tmp_filesystem::TmpFilesystem;
    use super::checker::Checker;

    let dir_template = Dir::new("dir")
        .add_dir(Dir::new("subdir1"))
        .add_dir(Dir::new("subdir2"))
        .add_file(File::new_empty("file1"))
        .add_file(File::new_empty("file2"));

    let filesystem = TmpFilesystem::new(&dir_template);

    assert!(Checker::check_dir(filesystem.path(), &dir_template));
}

#[test]
fn test_file_content() {

    use template::{Dir, File};
    use super::tmp_filesystem::TmpFilesystem;
    use super::checker::Checker;

    let dir_template = Dir::new("dir")
        .add_file(File::new_empty("file1"))
        .add_file(File::new("file2", "binary_content".to_owned().into_bytes()))
        .add_file(File::new_gitignore(&["ignore_pattern1", "ignore_pattern2"]));

    let filesystem = TmpFilesystem::new(&dir_template);

    assert!(Checker::check_dir(filesystem.path(), &dir_template));
}
