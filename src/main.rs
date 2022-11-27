#![feature(is_some_and)]

#![allow(clippy::needless_return)]
#![allow(clippy::bool_comparison)]

mod cli;
mod filesystem;
mod ignore_node;
mod batch;
mod absolute_ignore;
mod job;



fn main() {

    let matches = cli::build_cli_parser().get_matches();
    let args = cli::parse_cli_matches(&matches);

    let mut patterns = Vec::new();
    let mut jobs = vec![job::Job{path: args.path, ignore_context: None}];

    while jobs.is_empty() == false {

        
        let job = jobs.pop().expect("invalid job stack size");
        let push_job = &mut |job| jobs.push(job);
        
        let new_patterns = job::process_job(job, push_job, &args.ignore_file_name);
        
        if let Some(new_patterns) = new_patterns {

            patterns.extend(new_patterns);
        }
    }

    for pattern in patterns {

        println!("{}", pattern);
    }
}
