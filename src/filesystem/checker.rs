
use super::template;
use super::template::Visitor;



pub struct Checker;

impl template::Visitor for Checker {
    
    type DirParameter<'a> = &'a std::path::Path;
    type DirReturnType = bool;
    
    fn visit_dir<'a>(&self, dir: &template::Dir, path: Self::DirParameter<'a>) -> Self::DirReturnType {
        
        let path = path.join(&dir.name);
        
        // check if the directory exists 
        if (path.exists() && path.is_dir()) == false {

            return false;
        }

        // checks for unwanted entries
        let walker = walkdir::WalkDir::new(&path).min_depth(1).max_depth(1);
        for entry in walker.into_iter().map(|entry| entry.unwrap_or_else(|err| panic!("unexpected walk error, error: {}", err))) {

            let file_type = entry.file_type();
            let entry_name = entry.file_name();

            let found;
            
            if file_type.is_dir() {

                found = dir.child_dirs.iter().any(|dir| dir.name == entry_name);
            }
            else if file_type.is_file() {
                
                found = dir.child_files.iter().any(|file| file.name == entry_name);
            }
            else { // symlink
                
                assert!(file_type.is_symlink());
                found = false;
            }
            
            if found == false {
                
                return false;
            }
        }
        
        // check children nodes
        let files_check = dir.child_files.iter().map(|file| file.visit(self, &path)).reduce(|c1, c2| c1 && c2).unwrap_or(true);
        let dirs_check = dir.child_dirs.iter().map(|dir| dir.visit(self, &path)).reduce(|c1, c2| c1 && c2).unwrap_or(true);
        
        return files_check && dirs_check;
    }

    
    type FileParameter<'a> = &'a std::path::Path;
    type FileReturnType = bool;
    
    fn visit_file<'a>(&self, file: &template::File, path: Self::FileParameter<'a>) -> Self::FileReturnType {
        
        let path = path.join(&file.name);
        
        // check if the file exists
        if (path.exists() && path.is_file()) == false {
            
            return false;
        }

        // check file content
        if let Some(ref content) = file.content {
            
            let file_content = std::fs::read_to_string(&path).unwrap().into_bytes();
            
            if file_content != *content {
                
                return false;
            }
        }
        else { // empty file

            if std::fs::metadata(&path).expect("file metadata reading error").len() != 0 {

                return false;
            }
        }
        
        return true;
    }
}

impl Checker {
    
    fn new() -> Self {
        
        return Self{};
    }
    
    pub fn check_dir(path: &std::path::Path, dir: &template::Dir) -> <Self as template::Visitor>::DirReturnType {
        
        return Self::new().visit_dir(dir, path);
    }
    
    pub fn check_file(path: &std::path::Path, file: &template::File) -> <Self as template::Visitor>::FileReturnType {
        
        return Self::new().visit_file(file, path);
    }
}

#[test]
fn test_check_template_structure() {
    
    use template::{Dir, File};
    use super::tmp_filesystem::TmpFilesystem;
    
    
    let dir_template = Dir::new("dir")
    .add_dir(Dir::new("subdir"))
    .add_file(File::new_empty("file"));

    
    // unchanged structure
    let filesystem = TmpFilesystem::new(&dir_template);
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), true);
    
    // missing directory
    let filesystem = TmpFilesystem::new(&dir_template);
    std::fs::remove_dir(filesystem.path().join("dir/subdir")).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false);
    
    // missing file
    let filesystem = TmpFilesystem::new(&dir_template);
    std::fs::remove_file(filesystem.path().join("dir/file")).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false);
    
    // unwanted directory
    let filesystem = TmpFilesystem::new(&dir_template);
    std::fs::create_dir(filesystem.path().join("dir/unwanted_dir")).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false);
    
    // unwanted file
    let filesystem = TmpFilesystem::new(&dir_template);
    std::fs::File::create(filesystem.path().join("dir/unwanted_file")).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false)
}

#[test]
fn test_check_file_content() {
    
    use std::io::Write;
    use template::{Dir, File};
    use super::tmp_filesystem::TmpFilesystem;
    

    let open_file = |path: &std::path::Path| std::fs::File::options().write(true).open(path);


    let dir_template = Dir::new("dir")
    .add_file(File::new_empty("empty_file"))
    .add_file(File::new("file", "binary_content".to_owned().into_bytes()));
    
    
    // unchanged content
    let filesystem = TmpFilesystem::new(&dir_template);
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), true);
    
    // non-empty empty file
    let filesystem = TmpFilesystem::new(&dir_template);
    open_file(&filesystem.path().join("dir/empty_file")).unwrap().write_all("non-empty file".to_owned().as_bytes()).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false);
    
    // modified file content
    let filesystem = TmpFilesystem::new(&dir_template);
    open_file(&filesystem.path().join("dir/file")).unwrap().write_all("new binary content".to_owned().as_bytes()).unwrap();
    assert_eq!(Checker::check_dir(filesystem.path(), &dir_template), false);
}
