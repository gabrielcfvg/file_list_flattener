
use std::ffi::OsString;



pub trait Visitor {

    type DirParameter<'a>;
    type DirReturnType;
    fn visit_dir<'a>(&self, dir: &Dir, param: Self::DirParameter<'a>) -> Self::DirReturnType;
    
    type FileParameter<'a>;
    type FileReturnType;
    fn visit_file<'a>(&self, file: &File, param: Self::FileParameter<'a>) -> Self::FileReturnType;
}



#[derive(Clone)]
pub struct Dir {

    pub(super) name: OsString,
    pub(super) child_dirs: Vec<Dir>,
    pub(super) child_files: Vec<File>
}

impl Dir {
    
    pub fn new(name: impl std::convert::Into<OsString>) -> Self {

        return Self{name: name.into(), child_dirs: vec![], child_files: vec![]};
    }


    pub fn add_dir(mut self, new_dir: Dir) -> Self {

        self.child_dirs.push(new_dir);
        return self;
    }

    pub fn add_file(mut self, new_file: File) -> Self {

        self.child_files.push(new_file);
        return self;
    }


    pub fn visit<'a, R, P>(&self, visitor: &impl Visitor<DirReturnType=R, DirParameter<'a>=P>, param: P) -> R {

        return visitor.visit_dir(self, param);
    }
}



#[derive(Clone)]
pub struct File {

    pub(super) name: OsString,
    pub(super) content: Option<Vec<u8>>
}

impl File {

    pub fn new_empty(name: impl std::convert::Into<OsString>) -> Self {

        return Self{name: name.into(), content: None};
    }

    pub fn new(name: impl std::convert::Into<OsString>, content: Vec<u8>) -> Self {

        return Self{name: name.into(), content: Some(content)};
    }

    pub fn new_gitignore(content: &[&str]) -> Self {

        let content_line_list = content.iter().map(|line| String::from(*line));
        let content_string = content_line_list.reduce(|l1, l2| format!("{}\n{}", l1, l2)).unwrap_or_else(|| String::from(""));
        let content_bytes = content_string.into_bytes();

        return Self::new(".gitignore", content_bytes);
    }


    pub fn visit<'a, R, P>(&self, visitor: &impl Visitor<FileReturnType=R, FileParameter<'a>=P>, param: P) -> R {

        return visitor.visit_file(self, param);
    }
}
