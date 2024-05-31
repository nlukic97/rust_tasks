use slug::slugify;
use std::env;
use std::io;
use std::io::Read;

fn main() {
    eprintln!("Please submit your string input:");

    let mut input = String::new();
    /* read_multi_line(input); // testing
    return; */
    read_input(&mut input);

    // TODO - format this
    if input.trim() == "" {
        return eprintln!("Error: No input entered");
    }

    let arg_wrapped = get_argument();

    if arg_wrapped.is_err() {
        return eprintln!("Error: {}", arg_wrapped.unwrap_err());
    }

    match format_input(input, arg_wrapped.unwrap()) {
        Ok(value) => eprintln!("{value}"),
        Err(error) => eprintln!("Error: {}", error),
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
    return match arg.as_str() {
        "lowercase" => Ok(input.to_lowercase()),
        "uppercase" => Ok(input.to_uppercase()),
        "no-spaces" => Ok(input.replace(" ", "")),
        "slugify" => Ok(slugify(input)),
        "urgent-message" => Ok(format!("Urgent message: {} !!!", &input.to_uppercase().trim())),

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
        &_ => Err("Argument not found. Valid arguments: |lowercase|uppercase|no-spaces|slugify|urgent-message|mixup|"),
    };
}

// testing
fn read_multi_line(mut input: String) {
    /* Data like this could work: */
    /*
    name,age
    John,24
    Alice,26
    Markus,28
    ;

    */
    // the last clean line as well must be there
    println!("Say something: ");

    let mut input_finished = false;
    while input_finished == false {
        let mut string = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("failed to read line");

        input = input + &string;

        if string.trim().ends_with(';') {
            input_finished = true;
            input = input.replace(';', "");
        }

        println!("You input:");
        println!("{}", input);
        println!("--------------------------------");
    }
}
