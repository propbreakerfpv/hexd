fn main() {
    let mut args = std::env::args().into_iter();
    let _prog = args.next().unwrap();
    let mut input: String = args.fold(String::new(), |mut str, x| {
        str.push_str(&x);
        str
    });

    input = input.trim_start_matches("0x").to_string();
    println!("{}", input);
    let mut hex_chars = Vec::new();
    for idx in 0..input.len() / 2 {
        hex_chars.push(input[idx..idx + 2].to_string());
    }
    println!("{:?}", hex_chars);
    let output = hex_chars
        .iter()
        .map(|x| {
            i16::from_str_radix(x, 16).unwrap()
        })
        .map(|x| x as u8 as char)
        .fold(String::new(), |mut str, x| {
            str.push(x);
            str
        });

    println!("{}", output);
}
