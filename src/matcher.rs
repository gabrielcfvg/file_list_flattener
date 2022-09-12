


trait Matcher: Clone {

    fn matches(&self, value: &str) -> bool;
}


impl Matcher for glob::Pattern {
    
    fn matches(&self, value: &str) -> bool {
        
        return self.matches(value);
    }
}


impl Matcher for regex::Regex {
    
    fn matches(&self, value: &str) -> bool {
        
        return self.is_match(value);
    }
}
