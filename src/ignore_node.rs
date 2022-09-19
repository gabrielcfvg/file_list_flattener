
// std
use std::sync::Arc;

// extern
use ignore::gitignore::{Gitignore, GitignoreBuilder};



pub struct IgnoreNode {

    matcher: Gitignore,
    parent: Option<Arc<IgnoreNode>>
}

impl IgnoreNode {

    pub fn new(path: &std::path::Path, parent: Option<Arc<IgnoreNode>>) -> Arc<Self> {

        assert!(path.is_file());

        let mut builder = GitignoreBuilder::new(path.parent().expect("invalid path"));
        
        if let Some(error) = builder.add(path) {

            eprintln!("invalid file list, error: {:?}", error);
            std::process::exit(1);
        }
        
        let matcher = builder.build().expect("matcher build unexpected error");

        return Arc::new(IgnoreNode{matcher, parent});
    }

    pub fn matches(self: &Arc<Self>, path: &std::path::Path) -> bool {

        assert!(self.matcher.path().join(path).is_dir());

        let mut node = self;

        loop {

            match node.matcher.matched(path, true) {
                
                ignore::Match::None => {},
                ignore::Match::Ignore(_) => { return true; },
                ignore::Match::Whitelist(_) => { return false; },
            }

            match node.parent {

                Some(ref parent) => node = parent,
                None => { return false; }
            }
        }
    }
}

#[test]
fn test_ignore_node() {

    use std::path::Path;
    use crate::filesystem::tmp_filesystem::TmpFilesystem;
    use crate::filesystem::template::{File, Dir};
    
    let dir_template = Dir::new("dir")
        .add_file(File::new_gitignore(&["/ignore_dir*", "!/ignore_dir_white", "foo*", "bar*"]))
        .add_dir(Dir::new("ignore_dir1"))
        .add_dir(Dir::new("ignore_dir2"))
        .add_dir(Dir::new("ignore_dir_white"))
        .add_dir(Dir::new("sub_dir")
            .add_file(File::new_gitignore(&["!bar*"]))
            .add_dir(Dir::new("foo_dir"))
            .add_dir(Dir::new("bar_dir")));

    let filesystem = TmpFilesystem::new(&dir_template);

    let matcher_dir = IgnoreNode::new(&filesystem.path().join("dir/.gitignore"), None);
    let matcher_sub_dir = IgnoreNode::new(&filesystem.path().join("dir/sub_dir/.gitignore"), Some(matcher_dir.clone()));

    assert_eq!(matcher_dir.matches(Path::new("ignore_dir1")), true);
    assert_eq!(matcher_dir.matches(Path::new("ignore_dir2")), true);
    assert_eq!(matcher_dir.matches(Path::new("ignore_dir_white")), false);
    assert_eq!(matcher_dir.matches(Path::new("sub_dir")), false);

    assert_eq!(matcher_sub_dir.matches(Path::new("foo_dir")), true);
    assert_eq!(matcher_sub_dir.matches(Path::new("bar_dir")), false);
}
