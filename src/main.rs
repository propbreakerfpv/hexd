use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputType {
    Hex,
    Bin
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    values: Option<Vec<String>>
}

#[derive(Debug, Subcommand)]
enum Commands {
    Hex(Hex),
    Bin(Bin),
}

#[derive(Debug, Args)]
struct Hex {
    values: Vec<String>
}

#[derive(Debug, Args)]
struct Bin {
    values: Vec<String>
}

fn main() {
    
    let cli = Cli::parse();
    println!("{:?}", cli);

    match cli.command.unwrap_or_else(|| Commands::Hex(Hex { values: cli.values.unwrap() })) {
        Commands::Hex(scmd) => {
            let mut input: String = scmd.values.join("");
            input = input.trim_start_matches("0x").to_string();
            let output = group(2, input.chars()).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            }).map(|x| {
                    i16::from_str_radix(&x, 16).unwrap()
                })
                .map(|x| x as u8 as char)
                .fold(String::new(), |mut str, x| {
                    str.push(x);
                    str
                });

            println!("{}", output);
        }
        Commands::Bin(scmd) => {
            let mut input: String = scmd.values.join("");
            input = input.trim_start_matches("0x").to_string();
            let output = group(8, input.chars()).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            })
                .map(|x| {
                    i16::from_str_radix(&x, 2).unwrap()
                })
                .map(|x| x as u8 as char)
                .fold(String::new(), |mut str, x| {
                    str.push(x);
                    str
                });

            println!("{}", output);
        }
    }
}

#[derive(Debug)]
struct GroupIter<I> {
    iter: I,
    size: u32,
}

impl<I> Iterator for GroupIter<I> where I: Iterator {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = Vec::new();
        for _ in 1..=self.size {
            out.push(self.iter.next()?);
        }
        Some(out)
    }
}

fn group<I>(size: u32, iter: I) -> GroupIter<I> where I: Iterator {
    GroupIter {
        iter,
        size,
    }
}
