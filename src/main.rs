use std::{env, path::PathBuf, process::exit};

fn pwd_logical() {
    let cwd = env::var_os("PWD")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_default());
    if cwd == std::path::Path::new("") {
        println!("error: Unable to read working directory path!");
        exit(1);
    }
    println!("{}", cwd.display());
}

fn pwd_physical() {
    match std::fs::canonicalize(".") {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(_) => {
            println!("error: Unable to read working directory path!");
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        pwd_logical();
    } else if args.len() == 1 {
        let arg = &args[0];
        if arg == "-version" || arg == "-v" {
            println!("{}", VERSION);
        } else if arg == "-help" || arg == "-h" {
            println!("{}", VERSION);
            println!("{}", HELP);
        } else if arg == "-logical" || arg == "-l" {
            pwd_logical();
        } else if arg == "-physical" || arg == "-p" {
            pwd_physical();
        } else {
            println!("{}", UNKNOWN_COMMAND_ERROR);
            exit(1);
        }
    } else {
        println!("{}", UNKNOWN_COMMAND_ERROR);
        exit(1);
    }
}

static VERSION: &str = "pwd 1.0.0";
static HELP: &str = r#"A reimplementation of the Linux command pwd (Print Working Directory) in Rust.
- author: csm
- license: MIT
- about: 
    pwd displays the full path of the directory you are currently in.
- commands:
    pwd <no args>        -> Print working directory logical path
    pwd -version,  -v    -> Print pwd version
    pwd -help,     -h    -> Print pwd commands list
    pwd -logical,  -l    -> Print working directory logical path
    pwd -physical, -p    -> Print working directory physical path (resolving sym-links)
"#;
static UNKNOWN_COMMAND_ERROR: &str = r#"pwd error: Unknown command!

try help for usage:
  pwd -help
"#;
