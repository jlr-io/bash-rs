use clap::{Args, Subcommand};
use either::Either;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List files in a directory.
    Ls(LsArgs),
    /// Print the first x number lines in the file.
    Head(HeadArgs),
    /// Prints the last number of lines in the file.
    Tail(TailArgs),
    /// Prints the contents in the file.
    Cat(CatArgs),
}

#[derive(Debug)]
pub enum CommandError {
    IoError(std::io::Error),
    InvalidFileName,
}

impl From<std::io::Error> for CommandError {
    fn from(error: std::io::Error) -> Self {
        CommandError::IoError(error)
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}", self)
    }
}

#[derive(Debug, Args, Default)]
pub struct LsArgs {
    /// Include directory entries whose names begin with a dot (‘.’) except for . and ..
    #[arg(short = 'A', long = "almost-all")]
    almost_all: bool,

    /// Sort by descending time modified  
    #[arg(short, long)]
    time: bool,

    /// The path of the directory to list files from. Defaults to the current directory.
    #[arg(default_value = ".", value_name = "PATH")]
    path: std::path::PathBuf,
}

pub fn ls(args: LsArgs) -> Result<Either<Vec<String>, String>, CommandError> {
    let metadata = std::fs::metadata(&args.path)?;

    // if the path is a file, return the file name
    if metadata.is_file() {
        return args
            .path
            .file_name()
            .ok_or(CommandError::InvalidFileName)
            .and_then(|file_name| {
                file_name
                    .to_str()
                    .ok_or(CommandError::InvalidFileName)
                    .map(|file_name| Either::Right(file_name.to_owned()))
            });
    }

    let mut files = std::fs::read_dir(&args.path)?
        .filter_map(|file| file.ok())
        .collect::<Vec<std::fs::DirEntry>>();

    // sort by last modified if time flag is set
    match args.time {
        true => files.sort_by_key(|file| {
            let timestamp = match file.metadata() {
                Ok(metadata) => metadata.modified().unwrap_or(std::time::SystemTime::now()),
                Err(_) => std::time::SystemTime::now(),
            };
            std::cmp::Reverse(timestamp)
        }),
        false => files.sort_by_key(|file| file.file_name()),
    };

    // get file names and filter out hidden files if all flag is not set
    let file_names = files
        .iter()
        .filter_map(|file| {
            file.file_name()
                .to_str()
                .map(|file_name| file_name.to_owned())
        })
        .filter(|file_name| match file_name {
            name => !name.starts_with(".") || args.almost_all,
        })
        .collect::<Vec<String>>();

    Ok(Either::Left(file_names))
}

#[derive(Debug, Args)]
pub struct HeadArgs {
    /// The path of the file to read from.
    #[arg(value_name = "PATH")]
    path: std::path::PathBuf,

    /// The number of lines to print. Defaults to 10.
    #[arg(short, long, value_name = "NUM", default_value = "10")]
    number: usize,
}

pub fn head(args: HeadArgs) -> Result<Vec<String>, CommandError> {
    let file = std::fs::File::open(&args.path)?;
    let content = std::io::read_to_string(file)?;
    let lines = content
        .lines()
        .take(args.number)
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    Ok(lines)
}

#[derive(Debug, Args)]
pub struct TailArgs {
    /// The path of the file to read from.
    #[arg(value_name = "PATH")]
    path: std::path::PathBuf,

    /// The number of lines to print. Defaults to 10.
    #[arg(short, long, value_name = "NUM", default_value = "10")]
    number: usize,
}

pub fn tail(args: TailArgs) -> Result<Vec<String>, CommandError> {
    let file = std::fs::File::open(args.path)?;
    let content = std::io::read_to_string(file)?;
    let mut lines = content.lines();
    let mut tail_lines: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.number {
        if let Some(line) = lines.next_back() {
            tail_lines.push(line.to_owned())
        }
        i += 1;
    }
    tail_lines.reverse();
    Ok(tail_lines)
}

#[derive(Debug, Args)]
pub struct CatArgs {
    /// The path of the file to read from.
    #[arg(value_name = "PATH")]
    path: std::path::PathBuf,
}

pub fn cat(args: CatArgs) -> Result<String, CommandError> {
    let file = std::fs::File::open(args.path)?;
    let contents = std::io::read_to_string(file)?;
    Ok(contents)
}
