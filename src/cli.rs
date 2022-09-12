


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
pub mod cli_parser_tests {

    fn expect_parsing_error<'a>(parser: clap::Command<'static>, arg_list: impl std::iter::IntoIterator<Item=&'a str>) {
    
        assert!(matches!(parser.try_get_matches_from(arg_list), Err(_)));
    }

    fn expect_parsing_success<'a>(parser: clap::Command<'static>, arg_list: impl std::iter::IntoIterator<Item=&'a str>) -> clap::ArgMatches {

        return parser.try_get_matches_from(arg_list).expect("invalid argument list");
    }

    fn expect_arg(matches: &clap::ArgMatches, arg_id: &str, arg_value: &str) {

        assert_eq!(matches.get_one::<String>(arg_id), Some(&arg_value.to_owned()));
    }

    fn expect_arg_err(matches: &clap::ArgMatches, arg_id: &str) {

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

        let matches = expect_parsing_success(parser.clone(), ["flf", "-r", "foo", "."]);
        expect_arg(&matches, "regex_matcher", "foo");
        expect_arg_err(&matches, "glob_matcher");
        expect_arg(&matches, "path", ".");
        
        let matches = expect_parsing_success(parser.clone(), ["flf", "-g", "foo", "."]);
        expect_arg(&matches, "glob_matcher", "foo");
        expect_arg_err(&matches, "regex_matcher");
        expect_arg(&matches, "path", ".");
    }
}
