use std::env;

mod file_reader;
mod utils;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        eprintln!("The code requires a day number as an argument.");
        return;
    }

    let day_number = &args[1];
    let part = &args[2].parse::<u8>().unwrap();

    let output = match day_number.as_str() {
        "1" => day_01::run(*part),
        "2" => day_02::run(*part),
        "3" => day_03::run(*part),
        "4" => day_04::run(*part),
        "5" => day_05::run(*part),
        "6" => day_06::run(*part),
        "7" => day_07::run(*part),
        // "8" => day_08::run(*part),
        // "9" => day_09::run(*part),
        // "10" => day_10::run(*part),
        // "11" => day_11::run(*part),
        // "12" => day_12::run(*part),
        _ => "Invalid day number".to_string(),
    };

    println!("Output: {:?}", output);

} 