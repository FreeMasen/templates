extern crate docopt;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use docopt::Docopt;
use handlebars::Handlebars;

use std::{
    collections::{BTreeMap, HashSet},
    fs::{OpenOptions, File},
    io::{Read, Write},
    path::PathBuf,
};

const HELP: &str = "
templates
----
A command line utility for using Handlebars templates

Usage:
template <template> <output>
template (-h | --help)

Arguments:
template  The path to your template
output    Where you want the new file to exist
";
#[derive(Debug, Deserialize)]
pub struct Arguments {
    pub arg_template: PathBuf,
    pub arg_output: PathBuf,
    pub flag_data: Option<Vec<String>>,
}

pub fn parse_args() -> Arguments {
    Docopt::new(HELP)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

pub fn read_template(path: &PathBuf) -> String {
    let mut f = File::open(path)
            .expect("Unable to find the input template");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Unable to read the contents of the input template");
    contents
}

pub fn get_data(s: &str) -> BTreeMap<String, String> {
    let mut ret: BTreeMap<String, String> = BTreeMap::new();
    for key in extract_replacements(s) {
        let value = request_data(&key);
        ret.insert(key, value);
    }
    ret
}

pub fn extract_replacements(s: &str) -> HashSet<String> {
    let mut s = s.to_string();
    let mut set: HashSet<String> = HashSet::new();
    while s.len() > 0 {
        if let Some(start) = s.find("{{") {
            if let Some(end) = s.find("}}") {
                let new = s[start+2..end].to_string();
                if new.contains(" ") || new.starts_with("#") || new.starts_with("~#") {
                    panic!("template replacements can only be simple {{value}} types");
                }
                set.insert(new);
                s = s[end+2..].to_string();
            } else {
                panic!("{{ w/o }}")
            }
        } else {
            break;
        }
    }
    set
}

pub fn request_data(arg: &str) -> String {
    let mut ret = String::new();
    if let Some(first) = request_input(&format!("Please provide the value for {{{{{}}}}}", arg)) {
        ret = first
    } 
    while ret == "" {
        if let Some(subsequent) = request_input(&format!("invalid input, provide value for {{{{{}}}}}", arg)) {
            ret = subsequent
        }
    }
    ret
}

pub fn request_input(msg: &str) -> Option<String> {
    println!("{}", msg);
    let mut buf = String::new();
    let _ = ::std::io::stdin().read_line(&mut buf);
    let ret = buf.trim().to_string();
    if ret == "" {
        None
    } else {
        Some(ret)
    }
}

pub fn render(template: &str, replacements: BTreeMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
    let hb = Handlebars::new();
    let ret = hb.render_template(&template, &replacements)?;
    Ok(ret)
}

pub fn save_output(path: PathBuf, content: String) {
    let mut of = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .expect("Unable to open output file");
    of.write(content.as_bytes()).expect("Unable to write to output file");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn extract() {
        let r = extract_replacements("things and {{stuff}} and people and places {{and}} haha");
        assert_eq!(r, vec!["stuff".to_string(), "and".to_string()]);
    }
}