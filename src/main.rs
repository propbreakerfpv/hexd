use core::panic;

use clap::{Args, Parser, Subcommand, ValueEnum};


// todo:
//

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
    #[clap(name = "hex")]
    Hex,
    #[clap(name = "dec")]
    Dec,
    #[clap(name = "bin")]
    Bin,
    #[clap(name = "string")]
    Str,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(short, long, value_enum, default_value_t = DataType::Hex)]
    input_type: DataType,
    #[clap(short, long, value_enum, default_value_t = DataType::Str)]
    output_type: DataType,
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
            Item::Num(n) => ((*n as u8) as char).to_string(),
            Item::Char(c) => c.to_string(),
        }
    }
}

fn main() {
    
    let cli = Cli::parse();
    println!("{:?}", cli);

    let parsed = parse(&cli.values, cli.input_type);
    println!("parsed: {:?}", parsed);
    println!("output: {}", display(parsed, cli.output_type));
}

fn parse(input: &Vec<String>, input_type: DataType) -> Data {
    match input_type {
        DataType::Hex => {
            let mut input: String = input.join("");
            input = input.trim_start_matches("0x").to_string();
            let data = input.chars().bunch(2).map(|x| {
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
            let data = input.chars().bunch(8).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            })
                .map(|x| {
                    i32::from_str_radix(&x, 2).expect("expected a valid binary number")
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
        DataType::Dec => {
            let mut input: String = input.join("");
            input = input.trim_start_matches("0b").to_string();
            let data = input.chars().bunch(3).map(|x| {
                String::from(x.into_iter().fold(String::new(), |mut acc, x| {
                    acc.push(x);
                    acc
                }))
            })
                .map(|x| {
                    i32::from_str_radix(&x, 10).expect("expected a valid decimal number")
                })
                .map(|x| {
                    Item::Num(x)
                })
                .collect();
            Data {
                data,
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
        DataType::Bin => {
            parsed
                .data
                .into_iter()
                .map(|x| format!("{:#010b}", x.expect_num()))
                .collect::<Vec<String>>()
                .join(" ")
        }
        DataType::Str => {
            parsed
                .data
                .into_iter()
                .map(|i| i.to_string())
                .collect::<String>()
        }
        DataType::Dec => {
            parsed
                .data
                .into_iter()
                .map(|x| format!("{}", x.expect_num()))
                .collect::<Vec<String>>()
                .join(" ")
        }
    }
}

#[derive(Debug)]
struct BunchIter<I> {
    iter: I,
    size: u32,
}

impl<I> Iterator for BunchIter<I> where I: Iterator {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = Vec::new();
        for _ in 1..=self.size {
            out.push(self.iter.next()?);
        }
        Some(out)
    }
}

trait Bunch {
    fn bunch(self, size: u32) -> BunchIter<Self> where Self: Sized;
}

impl<T: Iterator> Bunch for T {
    fn bunch(self, size: u32) -> BunchIter<Self> {
        BunchIter {
            iter: self,
            size,
        }
    }
}
