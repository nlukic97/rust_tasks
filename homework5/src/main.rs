use slug::slugify;
use std::env;
use std::io;

fn main() {
    println!("Please submit your string input:");

    let mut input = String::new();
    read_input(&mut input);

    let arg_wrapped = get_argument();

    if arg_wrapped.is_err() {
        return println!("Error: {}", arg_wrapped.unwrap_err());
    }

    match format_input(input, arg_wrapped.unwrap()) {
        Ok(value) => println!("{value}"),
        Err(error) => println!("Error: {}", error),
    }
}

/// Used to read the user input, will assign it to the mutable variable input
fn read_input(mut input: &mut String) {
    io::stdin().read_line(&mut input).unwrap();
}

/// get argument - meant to fetch the first argument submitted by the user. Returns an error if the first argument is not there
fn get_argument() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Not enough args.");
    }
    let arg = &args[1];
    return Ok(arg.clone());
}

/// takes an input string, the argument supplied, and returns the string formatted in the desired way. Or returns an error if argument is not found.
fn format_input(input: String, arg: String) -> Result<String, &'static str> {
    match arg.as_str() {
        "lowercase" => return Ok(input.to_lowercase()),
        "uppercase" => return Ok(input.to_uppercase()),
        "no-spaces" => return Ok(input.replace(" ", "")),
        "slugify" => return Ok(slugify(input)),
        "urgent-message" => return Ok(format!("Urgent message: {} !!!", &input.to_uppercase().trim())),

        "mixup" => {
            let mut new_string = String::new();

            for mut char in input.chars() {
                if char.is_lowercase() {
                    char.make_ascii_uppercase();
                } else if char.is_uppercase() {
                    char.make_ascii_lowercase();
                }
                new_string.push(char);
            }

            return Ok(new_string);
        }
        &_ => return Err("Argument not found. Valid arguments: |lowercase|uppercase|no-spaces|slugify|urgent-message|mixup|"),
    };
}
