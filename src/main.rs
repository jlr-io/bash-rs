use clap::Parser;
use std::{
    error, fmt, fs,
    path::{self, PathBuf},
};

#[derive(Parser)]
struct Cli {
    cmd: String,
    #[clap(short, long)]
    all: bool,
    #[clap(short, long)]
    time: bool,
    #[clap(parse(from_os_str))]
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

// -a -> hidden files
// -t -> ordered by last-modified

// implementation of the bash `ls` command.
fn ls(path: path::PathBuf, all: bool, time: bool) -> Result<String, Box<dyn error::Error>> {
    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(_) => match path.to_str() {
            Some(path) => return Err(format!("No such file or directory: {}", path).into()),
            None => return Err("Invalid path".into()),
        },
    };

    // if the path is a file, return the file name
    if metadata.is_file() {
        return match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => Ok(file_name.to_owned()),
                None => return Err("Invalid file name".into()),
            },
            // this should never happen
            None => return Err("Something went wrong..".into()),
        };
    }

    let dir = fs::read_dir(&path)?;

    // get files
    let mut files = dir
        .filter_map(|file| file.ok())
        .collect::<Vec<std::fs::DirEntry>>();

    // sort by last modified if time flag is set
    if time {
        // order by last modified first, so the vector is reversed
        // TODO: is this the best way to do this?
        files.sort_by_key(|file| {
            let timestamp = if let Ok(metadata) = file.metadata().unwrap().modified() {
                metadata
            } else {
                std::time::SystemTime::now()
            };
            std::cmp::Reverse(timestamp)
        });
    }

    // get file names and filter out hidden files if all flag is not set
    let file_names = files
        .iter()
        .map(|file| match file.file_name().to_str() {
            Some(file_name) => file_name.to_owned(),
            None => String::new(),
        })
        .filter(|file_name| !file_name.starts_with(".") || all)
        .collect::<Vec<String>>()
        .join(" ");

    Ok(file_names)
}
