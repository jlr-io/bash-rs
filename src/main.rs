use clap::{Args, Parser, Subcommand};
use std::{
    error, fs,
    path
};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List files in a directory
    // #[command(arg_required_else_help = true)]
    Ls(LsArgs),
}

#[derive(Debug, Args)]
struct LsArgs {
    #[arg(short, long)]
    all: bool,
    #[arg(short='A', long="almost-all")]
    almost_all: bool,
    #[arg(short, long)]
    time: bool,
    /// The path to the directory to list files from. Defaults to the current directory.
    #[arg(default_value = ".", value_name = "PATH")]
    path: path::PathBuf,
}

// #[derive(Debug)]
// struct CliError(String);

// impl fmt::Display for CliError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Invalid Command: {}", self.0)
//     }
// }

// impl error::Error for CliError {}

// fn invalid_cmd(cmd: String) -> Result<String, Box<dyn error::Error>> {
//     Err(Box::new(CliError(cmd)))
// }

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Ls(args) => match ls(args) {
            Ok(val) => println!("{}", val),
            Err(error) => println!("{}", error),
        },
    }
}

// -a -> hidden files
// -t -> ordered by last-modified

// implementation of the bash `ls` command.
fn ls(args: LsArgs) -> Result<String, Box<dyn error::Error>> {
    let metadata = match fs::metadata(&args.path) {
        Ok(metadata) => metadata,
        Err(_) => match args.path.to_str() {
            Some(path) => return Err(format!("No such file or directory: {}", path).into()),
            None => return Err("Invalid path".into()),
        },
    };

    // if the path is a file, return the file name
    if metadata.is_file() {
        return match args.path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => Ok(file_name.to_owned()),
                None => return Err("Invalid file name".into()),
            },
            // this should never happen
            None => return Err("Something went wrong..".into()),
        };
    }

    let dir = fs::read_dir(&args.path)?;

    // get files
    let mut files = dir
        .filter_map(|file| file.ok())
        .collect::<Vec<std::fs::DirEntry>>();

    // sort by last modified if time flag is set
    if args.time {
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
        .filter(|file_name| !file_name.starts_with(".") || args.all)
        .collect::<Vec<String>>()
        .join(" ");

    Ok(file_names)
}
