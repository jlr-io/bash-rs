mod bash;
use clap::Parser;
use either::Either;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: bash::Commands,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        bash::Commands::Ls(args) => match bash::ls(args) {
            Ok(val) => match val {
                Either::Left(file_names) => println!("{}", file_names.join(" ")),
                Either::Right(file_name) => println!("{}", file_name),
            },
            Err(error) => println!("{}", error),
        },
        bash::Commands::Head(args) => match bash::head(args) {
            Ok(lines) => lines.iter().for_each(|line| println!("{}", line)),
            Err(error) => println!("{}", error),
        },
        bash::Commands::Tail(args) => match bash::tail(args) {
            Ok(lines) => lines.iter().for_each(|line| println!("{}", line)),
            Err(error) => println!("{}", error),
        },
        bash::Commands::Cat(args) => match bash::cat(args) {
            Ok(content) => println!("{}", content),
            Err(error) => println!("{}", error),
        },
    }
}
