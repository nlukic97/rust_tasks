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

    /* assert_ne!(1, 1);
    assert_eq!(1, 2);
    assert!(false); */
}

/* impl Rectangle {
    fn area(&self) -> u32 {
        panic!();
        unimplemented!();
        unreachable!();
        todo!();
    }
} */
