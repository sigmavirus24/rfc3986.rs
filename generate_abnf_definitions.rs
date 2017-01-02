//! Definitions collected from https://tools.ietf.org/html/rfc3986#appendix-a
//! and https://tools.ietf.org/html/rfc2234#section-6.1. 
//!
//! RFC 3986 relies on standardized definitions from RFC 2234 such as `ALPHA`,
//! `DIGIT`, etc. Collecting both into one place is useful.
use std::char;


fn make_alpha() -> Vec<String> {
    let exclude_from_alphabet: Vec<u32> = (91u32..97u32).collect::<Vec<u32>>();
    (65u32..123u32)
        .filter(|x| !exclude_from_alphabet.contains(x))
        .map(|x| char::from_u32(x).unwrap().to_string())
        .collect::<Vec<String>>()
}

fn make_digit() -> Vec<String> {
    (48u32..58u32)
        .map(|x| char::from_u32(x).unwrap().to_string())
        .collect::<Vec<String>>()
}

fn make_unreserved(alpha: &Vec<String>, digit: &Vec<String>) -> Vec<String> {
    let mut unreserved: Vec<String> = Vec::new();
    unreserved.extend(alpha.iter().cloned());
    unreserved.extend(digit.iter().cloned());
    unreserved.extend([
        "-".to_string(),
        ".".to_string(),
        "_".to_string(),
        "~".to_string(),
    ].iter().cloned());
    unreserved
}

fn print_vec(var_name: &str, vec: &Vec<String>) {
    println!("static {}: Vec<char> = vec![\n    '{}',\n];",
             var_name, vec.join("',\n    '"));
}


fn main() {
    let alpha: Vec<String> = make_alpha();
    let digit: Vec<String> = make_digit();
    let unreserved: Vec<String> = make_unreserved(&alpha, &digit);
    println!("//! This file is autogenerated by a tool in the root of the");
    println!("//! project.");
    print_vec("ALPHA", &alpha);
    print_vec("DIGIT", &digit);
    print_vec("UNRESERVED", &unreserved);
}
