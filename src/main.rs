use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let arguments: Arguments = parse_args(env::args());
    let template = read_template(arguments.input_template.value.clone());
    let replaced = update_template(template, &arguments.template_name.value);
    let output_dir = Path::new(&arguments.output_dir.value);
    ensure_path(&output_dir);
    let output_path = create_path(&output_dir, &arguments.input_template.value, &".tsx");
    save_output(output_path, replaced);
}

fn save_output(path: PathBuf, content: String) {
    let mut of = File::create(path).expect("Unable to create output file");
    of.write(content.as_bytes()).expect("Unable to write to output file");
}

fn create_path(output_dir: &Path, input_filename: &String, file_ext: &str) -> PathBuf {
    let mut in_file = input_filename.clone();
    in_file.push_str(&file_ext.clone());
    let file_path = Path::new(&in_file);
    let dir_path = Path::new(&output_dir);
    Path::join(dir_path, file_path)
}

fn ensure_path(path: &Path) {
    if !path.exists() {
        fs::create_dir(path.as_os_str())
            .expect("Unable to create output directory")
    }
}

fn update_template(template: String, template_name: &String) -> String {
    template.replace("{}", template_name)
}

fn read_template(path: String) -> String {
    let mut f = File::open(path)
            .expect("Unable to find the input template");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Unable to read the contents of the input template");
    contents
}

fn parse_args(args: env::Args) -> Arguments {
    let mut arguments: Arguments = Arguments::new();
    let mut current_flag: Flag = Flag::TemplateName;
    for arg in args {
        if arg.contains("/templates") {
            continue;
        }
        let current_first = arg.get(0..1);
        if current_first == Some("-") {
            current_flag = match arg.as_ref() {
                "-o" => Flag::OutputDir,
                "-t" => Flag::InputTemplate,
                _ => Flag::Unknown,
            }
        } else {
            let template_name = Argument::new(current_flag.clone(), arg.clone());
            arguments.append(template_name)
        }
    }
    arguments
}

#[derive(Debug)]
struct Argument {
    flag: Flag,
    value: String,
}

impl Argument {
    fn new(flag: Flag, value: String) -> Argument {
        Argument {
            flag: flag,
            value: value
        }
    }

    fn unknown() -> Argument {
        Argument::new(Flag::Unknown, String::from(""))
    }
}

struct Arguments {
    template_name: Argument,
    output_dir: Argument,
    input_template: Argument
}

impl Arguments {
    fn append(&mut self, arg: Argument) {
        match arg.flag {
            Flag::InputTemplate => self.input_template = arg,
            Flag::OutputDir => self.output_dir = arg,
            Flag::TemplateName => self.template_name = arg,
            Flag::Unknown => println!("Cannot append an unknown flag")
        };
    }

    fn new() -> Arguments {
        Arguments {
            template_name: Argument::unknown(),
            output_dir: Argument::unknown(),
            input_template: Argument::unknown(),
        }
    }
}

#[derive(Debug)]
enum Flag {
    Unknown,
    TemplateName,
    OutputDir,
    InputTemplate
}

impl Flag {
    fn clone(&self) -> Flag {
        match *self {
            Flag::Unknown => Flag::Unknown,
            Flag::InputTemplate => Flag::InputTemplate,
            Flag::OutputDir => Flag::OutputDir,
            Flag::TemplateName => Flag::TemplateName
        }
    }
}