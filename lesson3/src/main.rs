use std::vec;

mod foo;
fn main() {
    // loop
    /* let mut count = 0;
    loop {
        if count > 5 {
            break;
        }
        println!("{count}");
        count += 1;
    } */
    // ------------------------------------------------------

    // while loop
    /* let mut count = 0;
    while count < 5 {
        println!("{count}");
        count += 1;
    } */
    // ------------------------------------------------------

    // range based loops
    /* for i in 1..4 {
        println!("{i}");
    } */
    // ------------------------------------------------------

    // same with a vector based loop
    /* let fruits = vec!["apple", "bannana", "cherry"]; // vec! is something that can grow?
    for fruit in fruits {
        println!("{fruit}");
    } */
    // ------------------------------------------------------

    // iterators
    /* let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("{:?}", squared);

    let sum_of_squares: i32 = numbers.iter().map(|x| x * x).sum();
    println!("{:?}", sum_of_squares); */
    // ------------------------------------------------------

    //defining functions. types must be defined
    /* fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("{}", add(2, 3)); */
    // ------------------------------------------------------

    // passing data by value
    /* fn consume(data: String) {
        println!("Data received: {}", data);
    }
    let my_data = String::from("Hello");
    consume(my_data);
    // println!("{}", my_data); // this will have an error because ownership was transferred
    // my_data is no longer usable here as its ownership was transferred. */
    // ------------------------------------------------------

    // passing data by value
    /* fn read(data: &String) {
        println!("Data received: {}", data);
    }
    let my_data = String::from("Hello");
    read(&my_data);
    println!("Data is still: {}", my_data); // this will work */
    // ------------------------------------------------------

    // this method accepts a mutable reference, so it will change whatever the argument is referencing
    /* fn modify(data: &mut String) {
        data.push_str(" world!");
    }

    let mut greeting = String::from("Hello");
    modify(&mut greeting);
    println!("{greeting}"); */

    // Rules:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references to a particular piece of data, but not both.
    // 2. Mutable references must always be unique.

    // AKA:
    //You can't have mutable and immutable references to the same data in the same scope.
    // Also, there can only be one mutable reference to a piece of data in a particular scope.
    // ------------------------------------------------------

    // revesit this
    /* fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    } */
    // ------------------------------------------------------

    /* mod greetings {
        pub fn hello() {
            println!("Hello");
        }
    }
    greetings::hello(); */
    // ------------------------------------------------------

    /* mod outer {
        pub mod inner {
            pub fn inner_function() {
                println!("Inside the inner module!");
            }
        }
    }

    fn main() {
        outer::inner::inner_function();
    } */
    // ------------------------------------------------------

    /* mod my_module {
        pub fn public_function() {
            println!("Public");
            // private_function(); / we can call it from here
        }

        fn private_function() {
            println!("private");
        }
    }

    my_module::public_function(); */
    // ------------------------------------------------------

    // foo::some_function();
    // ------------------------------------------------------

    /* So,
    - the rules of mutable and immutable references, a variable can either mutable references,immutable  references in the same scope for a variable - but not both.
    - Vec makes a collection
    - what is Vec<_> used somewhere here?
     */
}
