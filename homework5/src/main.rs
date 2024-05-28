use slug::slugify;
use std::env;
use std::io;

fn main() {
    println!("Please submit your string input:");

    let mut input = String::new();
    read_input(&mut input);

    let mut arg = String::new();

    match get_argument() {
        Ok(value) => arg = value,
        Err(error) => println!("Error: {}", error),
    }

    update_input(&mut input, arg);

    /* if arg == "lowercase" {
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
    } */

    println!("{input}");
}

fn read_input(mut input: &mut String) {
    io::stdin().read_line(&mut input).unwrap();
}

fn get_argument() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Not enough args.");
    }
    let arg = &args[1];
    return Ok(arg.clone());
}

fn update_input(&mut input: &mut String, arg: String) {
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
}
