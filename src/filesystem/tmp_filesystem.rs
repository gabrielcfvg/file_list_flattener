
use super::template;
use super::builder::Builder;


pub struct TmpFilesystem {

    path: std::path::PathBuf
}

impl<'a> TmpFilesystem {

    pub fn new(dir_template: &template::Dir) -> Self {

        let name = Self::build_unique_name();
        let path = std::env::temp_dir().as_path().join(name);

        std::fs::create_dir(&path).unwrap_or_else(|err| panic!("TmpFilesystem directory creation failed, error: {}", err));
    
        if let anyhow::Result::Err(err) = Builder::build_dir(&path, dir_template) {

            panic!("template building failed during TmpFilesystem creation, error: {}", err);
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
        let since_unix_epoch = now.duration_since(std::time::SystemTime::UNIX_EPOCH).expect("invalid system date");

        return since_unix_epoch.as_nanos().to_string();
    }
}

impl Drop for TmpFilesystem {
    
    fn drop(&mut self) {
        
        assert!(self.path.exists());
        assert!(self.path.is_dir());

        std::fs::remove_dir_all(&self.path).unwrap_or_else(|_| panic!("TmpFilesystem mount directory deletion failed, path: {}", self.path.display()));
    }
}

#[test]
fn test_tmp_filesystem() {

    use super::checker::Checker;

    let dir_template = template::Dir::new("dir");
    let filesystem = TmpFilesystem::new(&dir_template);

    // check filesystem
    assert!(filesystem.path().exists());
    assert!(filesystem.path().is_dir());
    
    // check filesystem structure
    assert!(Checker::check_dir(filesystem.path(), &dir_template));

    let path = filesystem.path().to_owned();
    std::mem::drop(filesystem);

    assert!(path.exists() == false);
}
