use core::panic;

use clap::{Args, Parser, Subcommand, ValueEnum};


// todo: 
// 2. specify input and output type.
// loooots more


#[derive(Debug)]
enum Item {
    Num(i32),
    Char(char),
}

#[derive(Debug)]
struct Data {
    data: Vec<Item>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DataType {
    Hex,
    Bin,
    Str,
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

impl Item {
    fn expect_num(self) -> i32 {
        match self {
            Item::Num(n) => n,
            Item::Char(char) => panic!("expected a number got {}", char),
        }
    }
    fn expect_char(self) -> char {
        match self {
            Item::Num(n) => panic!("expected a char got {}", n),
            Item::Char(char) => char,
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Num(n) => n.to_string(),
            Item::Char(c) => c.to_string(),
        }
    }
}

trait ToInputType {
    fn to_input_type(&self) -> DataType;
}

impl ToInputType for Hex {
    fn to_input_type(&self) -> DataType {
        DataType::Hex
    }
}
impl ToInputType for Bin {
    fn to_input_type(&self) -> DataType {
        DataType::Bin
    }
}

fn main() {
    
    let cli = Cli::parse();
    println!("{:?}", cli);

    let parsed = match cli.command.unwrap_or_else(|| Commands::Hex(Hex { values: cli.values.unwrap() })) {
        Commands::Hex(scmd) => {
            parse(&scmd.values, scmd.to_input_type())
        }
        Commands::Bin(scmd) => {
            parse(&scmd.values, scmd.to_input_type())
        }
    };
    println!("parsed: {:?}", parsed);
    println!("output: {}", display(parsed, DataType::Str));
}

fn parse(input: &Vec<String>, input_type: DataType) -> Data {
    match input_type {
        DataType::Hex => {
            let mut input: String = input.join("");
            input = input.trim_start_matches("0x").to_string();
            let data = group(2, input.chars()).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            })
                .map(|x| {
                    i32::from_str_radix(&x, 16).expect("expected a valid hex number")
                })
                .map(|x| {
                    Item::Num(x)
                })
                .collect();
            Data {
                data,
            }
        }
        DataType::Bin => {
            let mut input: String = input.join("");
            input = input.trim_start_matches("0b").to_string();
            let data = group(8, input.chars()).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            })
                .map(|x| {
                    i32::from_str_radix(&x, 2).expect("expected a valid hex number")
                })
                .map(|x| {
                    Item::Num(x)
                })
                .collect();
            Data {
                data,
            }
        }
        DataType::Str  => {
            Data {
                data: input
                    .into_iter()
                    .map(|s| s .chars().nth(0).expect("should be at least one char per element"))
                    .map(|c| Item::Char(c))
                    .collect()
            }
        }
    }
}

fn display(parsed: Data, output_type: DataType) -> String {
    match output_type {
        DataType::Hex => {
            parsed
                .data
                .into_iter()
                .map(|x| format!("{:#01x}", x.expect_num()))
                .collect::<Vec<String>>()
                .join(" ")
        }
        DataType::Bin => todo!(),
        DataType::Str => {
            parsed
                .data
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
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
