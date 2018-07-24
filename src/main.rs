extern crate templates;
use templates::*;
fn main() {
    let args = parse_args();
    if !args.arg_template.exists() {
        eprintln!("Template argument path was not found");
        ::std::process::exit(1);
    }
    let template = read_template(&args.arg_template);
    let data = get_data(&template);
    let rendered = render(&template, data).expect("Error rendering template");
    save_output(args.arg_output, rendered);
}