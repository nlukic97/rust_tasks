use slug::slugify;
use std::env;
use std::io;

fn main() {
    println!("Please submit your string input:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided");
        return;
    }

    let arg = &args[1];

    if arg == "lowercase" {
        input = input.to_lowercase();
    } else if arg == "uppercase" {
        input = input.to_uppercase();
    } else if arg == "no-spaces" {
        input = input.replace(" ", "");
    } else if arg == "slugify" {
        input = slugify(input);
    } else if arg == "mixup" {
        let mut new_string = String::new();

        for mut char in input.chars() {
            if char.is_lowercase() {
                char.make_ascii_uppercase();
            } else if char.is_uppercase() {
                char.make_ascii_lowercase();
            }
            new_string.push(char);
        }

        input = new_string;
    } else if arg == "urgent-message" {
        input = format!("Urgent message: {} !!!", &input.to_uppercase().trim());
    }

    println!("{input}");
}
