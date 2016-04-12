extern crate regex;

pub fn remove_comments(lines: Vec<String>) -> Vec<String>
{
    let rgx = regex::Regex::new(r"(#.*)$|(^#.*$)").unwrap();
    lines.iter()
        .map(|x| rgx.replace(x, ""))
        .filter(|x| *x != "")
        .collect()
}

pub fn to_array(lines: Vec<String>) -> Vec<Vec<usize>>
{
    let size: usize = match lines[0].parse::<usize>() {
        Ok(n) => match n {
            0...2 => panic!("The array size must be equal or greater than 3."),
            _ => n,
        },
        Err(_) => panic!("The array size is invalid."),
    };
    let rgx = regex::Regex::new(r"[\s]+").unwrap();
    (0..size).map(|i| rgx.split(&lines[i + 1])
                  .filter(|s| *s != "")
                  .map(|s| s.parse::<usize>()
                       .expect("Wrong file format"))
                  .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>()
}
