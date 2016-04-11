extern crate regex;

pub fn remove_comments(lines: Vec<String>) -> Vec<String>
{
    let rgx = regex::Regex::new(r"(#.*)$|(^#.*$)").unwrap();
    lines.iter().map(|x| rgx.replace(x, "")).filter(|x| *x != "").collect()
}
