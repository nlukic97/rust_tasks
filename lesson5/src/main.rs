/* impl Rectangle {
    fn area(&self) -> u32 {
        panic!();
        unimplemented!();
        unreachable!();
        todo!();
    }
} */

use std::panic;

fn main() {
    fn division(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
        if divisor == 0.0 {
            Err("Cannot divide by zero!")
        } else {
            Ok(dividend / divisor)
        }
    }

    //  ----------------------------------------
    // pattern match

    /* match division(2.0, 0.0) {
        Ok(content) => {
            println!("{}", content)
        }
        Err(error) => {
            println!("{}", error);
        }
    } */
    //  ----------------------------------------

    // will panic
    /* let _ans = division(2.0, 0.0).unwrap(); // panic */

    //  ----------------------------------------
    /* let ans = division(10.0, 2.0).unwrap(); // 5
    println!("{ans}"); */

    //  ----------------------------------------
    /* let ans = division(2.0, 0.0).expect("Error: unable to divide by 0."); // will throw error and show this message
    println!("{ans}");
    println!("This will not print."); */

    //  ----------------------------------------
    // is ok example
    /* let ans = division(2.0, 1.0);
    if ans.is_ok() {
        println!("is ok. {}", ans.unwrap());
    } else {
        println!("is not ok.");
    } */

    // is error example
    /* let bad_ans = division(2.0, 0.0);
    if bad_ans.is_ok() {
        println!("is ok. {}", bad_ans.unwrap());
    } else {
        println!("is not ok.");
    }

    if bad_ans.is_err() {
        println!("is not ok.");
    } else {
        println!("is ok. {}", bad_ans.unwrap());
    } */

    /*  let ok_value: Result<&str, &str> = Result::Ok("This is ok");
    let err_value: Result<i32, &str> = Result::Err("An error occurred"); */

    //  ----------------------------------------

    // testing on my own
    /* let arr: [&str; 3] = ["Nikola", "Danilo", "Petar"];

    fn get_name(arr: [&str; 3], index: usize) -> Result<&str, &str> {
        if arr[index] == "Nikola" {
            return Ok(arr[index]);
        } else {
            return Err("This is an error");
        }
    }

    let ans = get_name(arr, 1);
    if ans.is_ok() {
        println!("{}", ans.unwrap());
    } else {
        println!("{}", ans.unwrap());
    } */

    //  ----------------------------------------

    /* fn find_name(id: usize) -> Option<&'static str> {
        let array = ["Alex", "Peter", "John"];

        let res: &str = array[id];
        if (res == "Alex") {
            return Some(res);
        } else {
            return None;
        }
    }

    let check_right_name = find_name(0);
    if check_right_name.is_some() {
        println!("{}", check_right_name.unwrap());
    }

    let check_wrong_name = find_name(1);
    if check_wrong_name.is_none() {
        println!("The value is none. Sorry.");
    } */

    //  ----------------------------------------
    // watching video

    /* let arr = [1, 2, 3];
    let ans = arr.get(4);

    println!("Value if array at index 4: {ans:?}") */

    //  ----------------------------------------
    // video from about 19:00 mnutes

    /* let some_option: Option<i32> = Some(32);
    let number = some_option.unwrap(); // will be fine
    println!("{number:?}");

    let some_option2: Option<i32> = None;
    let number2 = some_option2.expect("some_option2 was none"); // option .unwrap on none value, will panick but with msg
    println!("{number2:?}");

    let number2 = some_option2.unwrap(); // option .unwrap on none value, will panic
    println!("{number2:?}"); */

    //  ----------------------------------------

    /* let another_option: Result<(), &'static str> = Err("hi");
    let number: () = another_option.expect("Expected some value"); // --will panic with: Expected some value: "hi". So prints expect value and err */

    //  ----------------------------------------

    /* let a: u8 = 250;
    let b: u8 = 10;
    // let sum: u8 = a + b; // will panic in debug mode due to arythmetic overflow
    let sum: u8 = a.wrapping_add(b); // will allow overflow and wrap around it, so this will be 4
    let sum: u8 = a.saturating_add(b); // keep biggest value possible

    println!("{sum}"); */
    //  ----------------------------------------

    // explicitly clling panick!

    /* if true == true {
        panic!("It's time to {}", "panick!");
    } */

    //  ----------------------------------------

    // continue from 32:00
    // https://www.dropbox.com/scl/fi/7tlhvg0835y86zi54i9lm/Lecture-5.MP4?authuser=0&rlkey=kkzoq6xq9246meb5316zi33kq&e=1&st=84ijzbfy&dl=0
    /* let input = 31;

    let x = match input {
        1 | 5 => true,
        2 => false,
        _ => unreachable!("this will never happend"),
    };

    println!("{x}"); */
    //  ----------------------------------------

    /* // assert!(1 == 2, "the assert! did not work, 1 is not the same as 2");
    //  assert_ne!(1, 1);
    // assert_eq!(1, 2);
    // assert!(false); */

    //  ----------------------------------------
    // black magic on how to catch panics - one case where this was necessary. Written by someone who didn't know how to return a result so this was good for that
    /* let result = panic::catch_unwind(|| {
        println!("About to panic...");
        panic!("The panic happens here inside the catch_unwind block");
        println!("This line will not be executed");
    });

    match result {
        Ok(_) => println!("Successfully executed without panic"),
        Err(_) => println!("Caught a panick"),
    } */

    //  ----------------------------------------

    /* let ok_value: Result<_, &str> = Ok(42); // _ means - hey rust, i think you can figure this out
    let err_value: Result<i32, &str> = Err("An error occured");

    match err_value {
        Ok(value) => println!("is ok. {value}"),
        Err(error) => println!("is an error. {error}"),
    }

    if let Ok(value) = ok_value {
        println!("The value is ok: here it is {}", value);
    } else {
        println!("An error occurred.");
    } */

    //  ----------------------------------------

    // notice the boxed state object. Error types
    /* fn wrapper_function() -> Result<i32, Box<dyn std::error::Error>> {
        let value = error_prone_function()?; // this is kind of like matching. if its good return value, if not return error

        Ok(value)
    }

    fn error_prone_function() -> Result<i32, &'static str> {
        Err("This function always errors out!")
    }

    match wrapper_function() {
        Ok(value) => println!("Received value: {}", value),
        Err(e) => println!("An error occurred: {:?}", e),
    } */

    //  ----------------------------------------

    // trying on my own
    /* fn error_prone_function() -> Result<i32, &'static str> {
        // Ok(32) // this will mean its okay
        Err("Error caused in error prone function")
    }

    fn wrapper_function() -> Result<i32, Box<dyn std::error::Error>> {
        let value = error_prone_function()?;
        Ok(value)
    }

    let value = wrapper_function();
    match value {
        Ok(value) => println!("The value is ok. Here it is: {}", value),
        Err(error) => println!("An error has been caught: {}", error),
    } */

    //  ----------------------------------------

    /* // mapping and chaining
    let x: Result<i32, &str> = Ok(2);

    // option 1
    let y = x.map(|v| (v * 2) as f32);

    // or like underneath
    // let y = x.map(|v| double(v));
    // fn double(x: i32) -> i32 {
    //     x * 2
    // }

    assert_eq!(y, Ok(4.0), "is good"); */

    //  ----------------------------------------

    /* // use unwrap_or if the default value allocation is cheap...
    let err: Result<i32, &str> = Err("This is an error");
    println!("This might be Ok or Err: {}", err.unwrap_or(0));

    let ok: Result<i32, &str> = Ok(32);
    println!("This might be Ok or Err: {}", ok.unwrap_or(0));

    // otherwise use unwrap_or_else. good for side effects, and if you don't want to waste cpu cycles on smt
    let err2: Result<i32, &str> = Err("This is an error in err2");
    let value = err2.unwrap_or_else(|err| {
        println!("Error encountered:{}", err);
        0
    });

    assert_eq!(value, 0); */

    //  ----------------------------------------
    // didn't get this part 100%

    /* let x: Result<i32, &str> = Err("This is an error");
    let value: Result<_, &str> = x.or(Ok(0)); // difference between .unwrap_or and .or is that we can change the error typed. or returns Reuslt, it doesn't unwrap the value
    assert_eq!(value, Ok(0)); */

    //  ----------------------------------------

    /* let x: Result<i32, &str> = Ok(2);

    let y: Result<i32, &str> = Ok(100);

    assert_eq!(x.and(y), Ok(100)); // replaces the OK type, it will be Ok(100)

    let x: Result<i32, &str> = Ok(2);
    let res = x.and_then(|v| {
        // changes the error type, we can do something with the value of the Ok's value
        if v > 1 {
            Ok(v + 1)
        } else {
            Err("Value too small")
        }
    });

    assert_eq!(res, Ok(3)); */

    //  ----------------------------------------

    // continue from 1:07:00
    // https://www.dropbox.com/scl/fi/7tlhvg0835y86zi54i9lm/Lecture-5.MP4?rlkey=kkzoq6xq9246meb5316zi33kq&e=1&st=84ijzbfy&dl=0

    // let x: Result<Option<i32>, &str> = Ok(Some(5));
    let x: Result<Option<i32>, &str> = Err("this is an error");

    let y: Option<Result<i32, &str>> = x.transpose();

    assert_eq!(y, Some(Ok(5)));
}
