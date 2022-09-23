


fn add_parent_to_ignore_pattern(path: &str, pattern: IgnorePattern) -> String {

    assert!(pattern.pattern_body.is_empty() == false);
    assert!(pattern.pattern_body.starts_with('/') == false);


    let mut result = pattern.pattern_body.to_string();

    if pattern.absolute == false {

        result = format!("**/{}", result);
    }

    // add parent path
    result = format!("{}/{}", path, result);

    if pattern.negated == true {

        result = format!("!{}", result);
    }

    return result;
}

#[test]
fn test_add_parent_to_ignore_pattern() {

    #[track_caller]
    fn assert_pattern(base_path: &str, input: &str, expected_output: &str) {

        assert_eq!(&add_parent_to_ignore_pattern(base_path, parse_ignore_pattern(input)), expected_output);
    }

    let path = "foo/bar";

    assert_pattern(path, "/foobar", "foo/bar/foobar");
    assert_pattern(path, "/foobar/", "foo/bar/foobar/");
    assert_pattern(path, "foobar", "foo/bar/**/foobar");
    assert_pattern(path, "!/foobar", "!foo/bar/foobar");
    assert_pattern(path, "!foobar", "!foo/bar/**/foobar");

    
    assert_pattern("/foo/bar", "/foobar", "/foo/bar/foobar");
    assert_pattern("./foo/bar", "/foobar", "./foo/bar/foobar");
}


#[derive(PartialEq, Debug)]
struct IgnorePattern<'a> {

    negated: bool,
    absolute: bool,
    pattern_body: &'a str
}

fn parse_ignore_pattern(mut pattern: &str) -> IgnorePattern {

    assert!(pattern.is_empty() == false);
    assert!(pattern.starts_with('#') == false);
    debug_assert_eq!(pattern, strip_trailing_whitespaces(pattern));

    let mut negated = false;
    let mut absolute = false;

    if pattern.starts_with('!') {

        negated = true;
        pattern = &pattern[1..];
    }
    
    if let Some((idx, _)) = pattern.char_indices().find(|(idx, ch)| (*ch == '/') && (*idx != (pattern.len() - 1))) {

        absolute = true;

        // remove the slash if it appears at the beginning
        if idx == 0 {

            pattern = &pattern[1..];
        }
    }
    
    return IgnorePattern{negated, absolute, pattern_body: pattern};
}

#[test]
fn test_parse_ignore_pattern() {

    #[track_caller]
    fn assert_parse(pattern: &str, negated: bool, absolute: bool, body: &str) {

        assert_eq!(parse_ignore_pattern(pattern), IgnorePattern{negated, absolute, pattern_body: body});
    }

    assert_parse("foo", false, false, "foo");
    assert_parse("!foo", true, false, "foo");
    assert_parse("!/foo", true, true, "foo");
    assert_parse("/foo", false, true, "foo");
    assert_parse("foo/bar", false, true, "foo/bar");
    assert_parse("foo/", false, false, "foo/");
}


fn filter_ignore_line(line: &str) -> bool {

    if line.is_empty() {

        return false;
    }
    
    // filter comments
    if line.starts_with('#') {

        return false;
    }

    // filter blank lines
    if line.chars().any(|ch| ch != ' ') == false {

        return false
    }

    return true;
}

#[test]
fn test_filter_ignore_line() {

    #[track_caller]
    fn assert_filter(line: &str, result: bool) {

        assert_eq!(filter_ignore_line(line), result);
    }

    assert_filter("", false);
    assert_filter("# pattern", false);
    assert_filter("\\#pattern", true);
    assert_filter(" ", false);
    assert_filter(" foo", true);
    assert_filter(" #foo", true);
}


// reference: https://github.com/git/git/blob/4b79ee4b0cd1130ba8907029cdc5f6a1632aca26/dir.c#L936
fn strip_trailing_whitespaces(line: &str) -> &str {
    
    let stripped = line.trim_end_matches(' ');

    if stripped.len() == line.len() {

        return line;
    }

    if stripped.is_empty() {

        return "";
    }


    let strip_position = stripped.len();
    let get_line_char = |idx| line.chars().nth(idx).expect("unexpected string size");

    if stripped.ends_with('\\') && get_line_char(stripped.len()) == ' ' {

        return &line[..strip_position + 1];
    }
    else {
        
        return stripped;
    }
}

#[test]
fn test_strip_trailing_whitespaces() {

    #[track_caller]
    fn assert_strip(line: &str, result: &str) {

        assert_eq!(strip_trailing_whitespaces(line), result);
    }

    assert_strip("", "");
    assert_strip(" ", "");
    assert_strip("  ", "");
    assert_strip("\\ ", "\\ ");
    assert_strip("\\ \\ ", "\\ \\ ");
    assert_strip("\\  ", "\\ ");
    assert_strip("\\", "\\");
    assert_strip("foo bar", "foo bar");
    assert_strip(" foo bar", " foo bar");
    assert_strip("foo bar ", "foo bar");
}
