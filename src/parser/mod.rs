extern crate regex;

pub fn remove_comments(lines: Vec<String>) -> Vec<String>
{
    let rgx = regex::Regex::new(r"(#.*)$|(^#.*$)").unwrap();
    lines.iter().map(|x| rgx.replace(x, "")).filter(|x| *x != "").collect()
}

pub fn to_array(lines: Vec<String>) -> u32
{
    let size: u32 = match lines[0].parse::<u32>() {
        Ok(n) => match n {
            0...2 => panic!("The array size must be equal or greater than 3."),
            _ => n,
        },
        Err(_) => panic!("The array size is invalid."),
    };
    let s = (1..size*size).map(|i| if i % size == 0 { r"( +\d)\\n" } else { r"( +\d)"}).collect::<String>();
    println!("{}", s);
    size
}

pub fn to_array_test(lines: Vec<String>) -> [String]
{
    let size: u32 = match lines[0].parse::<u32>() {
        Ok(n) => match n {
            0...2 => panic!("The array size must be equal or greater than 3."),
            _ => n,
        },
        Err(_) => panic!("The array size is invalid."),
    };
    let mut array: [String; size] = [""; size];
    for x in &array {

    }
}
