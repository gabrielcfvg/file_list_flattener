


pub fn build_cli_parser() -> clap::Command<'static> {

    use clap::{Command, Arg};
    
    // program info
    let command = Command::new("file list flattener")
        .version("0.1.0")
        .about("flatten file lists with gitignore syntax")
        .author("gabrielcfvg <gabrielcfvg@gmail.com>");
    
    // file list name selector
    let command = command
        .arg(Arg::new("file list name")
            .short('n')
            .takes_value(true)
            .default_value(".gitignore")
            .id("file_list_name"));

    // search path
    let command = command
        .arg(Arg::new("path")
            .takes_value(true)
            .default_value(".")
            .id("path"));
        
    return command;
}

#[test]
fn test_cli_parser() {
    
    let parser = build_cli_parser();

    let expect_parsing_success = |arg_list: &str| assert!(matches!(parser.clone().try_get_matches_from(arg_list.split_ascii_whitespace()), Ok(_)));
    let expect_parsing_error = |arg_list: &str| assert!(matches!(parser.clone().try_get_matches_from(arg_list.split_ascii_whitespace()), Err(_)));

    expect_parsing_success("flf");
    expect_parsing_success("flf .");
    expect_parsing_success("flf -n foo");
    expect_parsing_success("flf -n foo .");
    expect_parsing_success("flf . -n foo");

    expect_parsing_error("flf -n");
    expect_parsing_error("flf . -n");
}


#[derive(Debug, PartialEq, Eq)]
pub struct Arguments {

    pub path: std::path::PathBuf,
    pub ignore_file_name: String
}

pub fn parse_cli_matches(matches: &clap::ArgMatches) -> Arguments {

    let get_value = |id: &str| matches.get_one::<String>(id).expect("invalid matches").to_owned();

    let path = std::path::PathBuf::from(get_value("path"));
    let ignore_file_name = get_value("file_list_name");

    return Arguments{path, ignore_file_name};
}

#[test]
fn test_cli_matches_parser() {

    let parser = build_cli_parser();

    let expect_result = |args: &str, path: &str, ignore_file_name: &str| {
        
        let expected_arguments = Arguments{path: std::path::PathBuf::from(path), ignore_file_name: ignore_file_name.to_owned()};
        let matches = parser.clone().try_get_matches_from(args.split_ascii_whitespace()).expect("invalid arguments");

        assert_eq!(parse_cli_matches(&matches), expected_arguments)
    };

    expect_result("flf", ".", ".gitignore");
    expect_result("flf .", ".", ".gitignore");
    expect_result("flf foo", "foo", ".gitignore");
    expect_result("flf -n bar", ".", "bar");
    expect_result("flf -n bar foo", "foo", "bar");
    expect_result("flf foo -n bar", "foo", "bar");
}
