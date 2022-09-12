


fn build_cli_parser() -> clap::Command<'static> {

    use clap::{Command, Arg};

    // program info
    let command = Command::new("file list flattener")
        .version("0.1.0")
        .about("flatten file lists")
        .author("gabrielcfvg <gabrielcfvg@gmail.com>");

    // file list name matcher
    let command = command
        .arg(Arg::new("glob matcher")
            .short('g')
            .long("glob")
            .takes_value(true)
            .id("glob_matcher")
            .conflicts_with("regex_matcher")
            .required_unless_present("regex_matcher")
            .help("match file lists with Unix glob patterns"))
        .arg(Arg::new("regex matcher")
            .short('r')
            .long("regex")
            .takes_value(true)
            .id("regex_matcher")
            .conflicts_with("glob_matcher")
            .required_unless_present("glob_matcher")
            .help("match file lists with rust regex crate <crates.io/crates/regex>"));

    // search path
    let command = command
        .arg(Arg::new("path")
            .takes_value(true)
            .required(true)
            .id("path"));

    return command;
}

#[cfg(test)]
mod cli_parser_tests {

    pub fn expect_parsing_error<'a>(parser: clap::Command<'static>, arg_list: impl std::iter::IntoIterator<Item=&'a str>) {
    
        assert!(matches!(parser.try_get_matches_from(arg_list), Err(_)));
    }

    pub fn expect_parsing_success<'a>(parser: clap::Command<'static>, arg_list: impl std::iter::IntoIterator<Item=&'a str>) -> clap::ArgMatches {

        return parser.try_get_matches_from(arg_list).expect("invalid argument list");
    }

    pub fn expect_arg(matches: &clap::ArgMatches, arg_id: &str, arg_value: &str) {

        assert_eq!(matches.get_one::<String>(arg_id), Some(&arg_value.to_owned()));
    }

    pub fn expect_arg_err(matches: &clap::ArgMatches, arg_id: &str) {

        assert_eq!(matches.get_one::<String>(arg_id), None);
    }

    #[test]
    fn test_invalid_arg_list() {
    
        let parser = super::build_cli_parser();
    
        expect_parsing_error(parser.clone(), ["flf"]);
        expect_parsing_error(parser.clone(), ["flf", "."]);
        expect_parsing_error(parser.clone(), ["flf", "-r", "foo"]);
        expect_parsing_error(parser.clone(), ["flf", "-g", "foo"]);
        expect_parsing_error(parser.clone(), ["flf", ".", "-r"]);
        expect_parsing_error(parser.clone(), ["flf", ".", "-g"]);
        expect_parsing_error(parser.clone(), ["flf", "-r", "foo", "-g", "foo", "."]);
    }

    #[test]
    fn test_matcher_selection() {

        let parser = super::build_cli_parser();

        // regex
        let matches = expect_parsing_success(parser.clone(), ["flf", "-r", "foo", "."]);
        expect_arg(&matches, "regex_matcher", "foo");
        expect_arg_err(&matches, "glob_matcher");
        expect_arg(&matches, "path", ".");
        
        // glob
        let matches = expect_parsing_success(parser.clone(), ["flf", "-g", "foo", "."]);
        expect_arg(&matches, "glob_matcher", "foo");
        expect_arg_err(&matches, "regex_matcher");
        expect_arg(&matches, "path", ".");
    }
}


#[derive(PartialEq, Eq, Debug)]
enum MatcherOption {

    Regex(String),
    Glob(String)
}

struct Arguments {

    matcher: MatcherOption,
    path: String
}

fn parse_cli_matches(matches: clap::ArgMatches) -> Arguments {

    let get_value = |id: &str| matches.get_one::<String>(id).expect("invalid matches").to_owned();


    let path = get_value("path");


    let matcher: MatcherOption;

    if matches.contains_id("regex_matcher") {

        matcher = MatcherOption::Regex(get_value("regex_matcher"));
    }
    else if matches.contains_id("glob_matcher") {
        
        matcher = MatcherOption::Glob(get_value("glob_matcher"));
    }
    else {

        panic!("invalid matches");
    }

    return Arguments{matcher, path};
}

#[test]
fn test_cli_matches_parser() {

    use cli_parser_tests::*;

    let parser = build_cli_parser();
    
    // regex
    let matches = expect_parsing_success(parser.clone(), ["flf", "-r", "regex_pattern", "."]);
    let arguments = parse_cli_matches(matches);
    assert_eq!(arguments.path, ".");
    assert_eq!(arguments.matcher, MatcherOption::Regex("regex_pattern".to_owned()));
    
    // glob
    let matches = expect_parsing_success(parser.clone(), ["flf", "-g", "glob_pattern", "."]);
    let arguments = parse_cli_matches(matches);
    assert_eq!(arguments.path, ".");
    assert_eq!(arguments.matcher, MatcherOption::Glob("glob_pattern".to_owned()));
}
