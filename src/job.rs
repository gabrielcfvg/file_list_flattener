
use std::sync::Arc;
use crate::absolute_ignore;
use crate::ignore_node::IgnoreNode;



struct Job {

    path: std::path::PathBuf,
    ignore_context: Option<Arc<IgnoreNode>>
}


fn process_job(job: Job, push_job: &mut dyn FnMut(Job), ignore_file_name: &str) -> Option<Vec<String>> {

    let Job{ref path, mut ignore_context} = job;

    let mut local_patterns = None;

    let local_gitignore_path = path.join(ignore_file_name);
    if local_gitignore_path.is_file() {

        ignore_context = Some(IgnoreNode::new(&local_gitignore_path, ignore_context));
        local_patterns = Some(absolute_ignore::read_patterns_from_file(&local_gitignore_path, path));
    }

    let dir_walker = walkdir::WalkDir::new(path).min_depth(1).max_depth(1);
    for dir in dir_walker.into_iter().map(|entry| entry.unwrap()).filter(|entry| entry.file_type().is_dir()) {

        if let Some(ref ignore_matcher) = ignore_context {

            if ignore_matcher.matches(dir.path()) == true {

                continue;
            }
        }

        push_job(Job{path: dir.path().to_owned(), ignore_context: ignore_context.clone()});
    }

    return local_patterns;
}

#[test]
fn test_job_processing() {

    use crate::filesystem::tmp_filesystem::TmpFilesystem;
    use crate::filesystem::template::{Dir, File};


    let fs_template = Dir::new("dir")
        .add_file(File::new_gitignore(&["foo/"]))
        .add_dir(Dir::new("foo"))
        .add_dir(Dir::new("bar")
            .add_dir(Dir::new("foo")));

    let fs = TmpFilesystem::new(&fs_template);

    
    // root dir
    let mut subdir_job = None;
    let mut push_job = |job| subdir_job = Some(job);
    
    let job = Job{path: fs.path().join("dir"), ignore_context: None};
    let patterns = process_job(job, &mut push_job, ".gitignore");
    
    assert_eq!(patterns, Some(vec![fs.path().join("dir/**/foo/").to_str().unwrap().to_owned()]));
    assert!(subdir_job.is_some());
    assert!(subdir_job.as_ref().unwrap().path == fs.path().join("dir/bar"));
    
    // "bar" subdir
    let mut push_job = |_| panic!("unexpected subdir");
    let patterns = process_job(subdir_job.unwrap(), &mut push_job, ".gitignore");

    assert!(patterns.is_none());
}
