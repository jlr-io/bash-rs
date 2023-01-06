use clap::Parser;
use std::{
    error, fmt, fs,
    path::{self, PathBuf},
};

#[derive(Parser)]
struct Cli {
    // #[clap]
    cmd: String,
    #[clap(short, long)]
    all: bool,
		#[clap(short, long)]
		time: bool,
    // #[clap(parse(from_os_str))]
    path: Option<path::PathBuf>,
}

#[derive(Debug)]
struct CliError(String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Command: {}", self.0)
    }
}

impl error::Error for CliError {}

fn invalid_cmd(cmd: String) -> Result<String, Box<dyn error::Error>> {
    Err(Box::new(CliError(cmd)))
}

fn main() {
    let args = Cli::parse();

    let path = match args.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };

    let result = match args.cmd.as_str() {
        "ls" => ls(path, args.all, args.time),
        _ => invalid_cmd(args.cmd),
    };

    match result {
        Ok(val) => println!("{}", val),
        Err(error) => println!("{}", error),
    };
}

// -t -> ordered by last-modified
// -a -> hidden files

// implementation of the linux `ls` command.
fn ls(path: path::PathBuf, all: bool, time: bool) -> Result<String, Box<dyn error::Error>> {
    let metadata = fs::metadata(&path)?;

    if metadata.is_file() {
        return Ok(path.file_name().unwrap().to_str().unwrap().to_owned());
    }

    let dir = fs::read_dir(&path)?;

    let mut file_names: Vec<String> = Vec::new();

    for file in dir {
        let file_name = file.unwrap().file_name().into_string().unwrap();

        if all || !file_name.starts_with(".") {
            file_names.push(file_name);
        }
    }

    // sort by date modified
		// if time {
		// 	file_names.sort_by()
		// }
    file_names.sort();

    // create a String to return
    let mut ls_result: String = "".to_owned();
    for file_name in file_names {
        ls_result = [ls_result, file_name].join(" ");
    }

    Ok(ls_result)
}