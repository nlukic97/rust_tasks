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

    let arr = [1, 2, 3];
    let ans = arr.get(4);

    println!("Value if array at index 4: {ans:?}")

    // https://www.dropbox.com/scl/fi/7tlhvg0835y86zi54i9lm/Lecture-5.MP4?rlkey=kkzoq6xq9246meb5316zi33kq&e=1&st=84ijzbfy&dl=0 - from: 19:13
    // https://robot-dreams-rust.mag.wiki/5-error-handling/index.html#3-the--operator-and-the-option-type notes.
}
