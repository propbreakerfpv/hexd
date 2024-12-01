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
    Hex(Hex)
}

#[derive(Debug, Args)]
struct Hex {
    values: Vec<String>
}

fn main() {
    
    let cli = Cli::parse();
    println!("{:?}", cli);

    match cli.command.unwrap_or_else(|| Commands::Hex(Hex { values: cli.values.unwrap() })) {
        Commands::Hex(scmd) => {
            let mut input: String = scmd.values.join("");
            input = input.trim_start_matches("0x").to_string();
            let output = group(input.chars()).map(|x| {
                let mut s = String::from(x.0);
                s.push(x.1);
                s
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
    }
}

#[derive(Debug)]
struct GroupIter<I> {
    iter: I,
}

impl<I> Iterator for GroupIter<I> where I: Iterator {
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.iter.next()?, self.iter.next()?))
    }
}

fn group<I>(iter: I) -> GroupIter<I> where I: Iterator {
    GroupIter { iter }
}
