extern crate regex;

use node::Node;

pub fn remove_comments(lines: Vec<String>) -> Vec<String>
{
    let rgx = regex::Regex::new(r"(#.*)$|(^#.*$)").unwrap();
    lines.iter()
        .map(|x| rgx.replace(x, ""))
        .filter(|x| *x != "")
        .collect()
}

pub fn to_node(lines: Vec<String>) -> Node
{
    let size: usize = match lines[0].parse::<usize>() {
        Ok(n) => match n {
            0...2 => panic!("The array size must be equal or greater than 3."),
            _ => n,
        },
        Err(_) => panic!("The array size is invalid."),
    };
    let rgx = regex::Regex::new(r"[\s]+").unwrap();
    Node { state: rgx.split(&lines.join(" "))
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>()
            .expect("Wrong file format"))
            .collect::<Vec<usize>>(),
            len: size }
}
