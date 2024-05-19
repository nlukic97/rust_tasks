/* //! Some hello world program
fn main() {
    let integer = 10;

    let floating_point = 3.14;

    let character = 'a';

    let string = "Hello, Rust!";

    let boolean = true;

    let num = 42;

    println!("{num}");

    let x = "5";

    println!("x is: {}", x); // prints "x is: 5"

    let x: i32 = x.parse().expect("Not a number");

    println!("x is: {}", x); // prints "x is: 5"
}
 */

/* fn main() {
    let x = 5;

    let y: i32 = {
        let temp = x * 2;
        temp + 1
    };

    println!("Y is {}", y);
} */

/* fn main() {
    let x = 5;

    let y: i32 = 'bad: {
        if x < 10 {
            break 'bad x * 2;
        }
        x + 1
    };

    println!("Y is {}", y);
}
 */

/* fn main() {
    let mut y = 5;
}
 */

/* fn main() {
    let tuple = (111, 2.0, "Rust");

    println!("{}", tuple.0);
    // println!("{:x}", tuple.0); // as a hex
    // println!("{:X}", tuple.0); // as a hex capital
    println!("{:b}", tuple.0); // as a hex

    let (integer, floating_point, string) = tuple;
    // ↑ destructuring a tuple
}
 */

/* fn main() {
    let tuple = (1, "hello", 4.5);

    let (x, y, z) = tuple;

    println!("x: {}, y: {}, z: {}", x, y, z);

    // Accessing elements of a tuple

    let first_element = tuple.0;

    let second_element = tuple.1;

    let third_element = tuple.2;

    println!("First element: {}", first_element);

    // ↑ prints "First element: 1"

    println!("Second element: {}", second_element);

    // ↑ prints "Second element: hello"

    println!("Third element: {}", third_element);

    // ↑ prints
} */

/* fn main() {
    let mut array = [1, 2, 3, 4, 5]; // type is [i32; 5]
    let mut array2 = [3, 4, 5, 6, 7]; // type is [i32; 5]

    array = array2; // copies array2 to array1
                    // let array3 = [5; 2000]; // 2000 elements, al
    let first = array[0];
    println!("{:?}", array);
} */

/* use std::mem;

fn main() {
    let my_string = String::from("Hello, world!");

    let string_slice: &str = &my_string;

    // due to Unicode, we can’t just index Strings

    for c in my_string.chars() {
        println!("{}", c);
    }

    let mut my_string2 = String::new();
    my_string2.push_str("Whats up world?");

    let mut my_string3 = "Hello world!".to_string();

    let stringslice = &my_string;

    // string indexation, char must be used
    // for c in my_string.chars() {
    //     println!("{c}");
    // }

    // initial capacity
    // let mut string4 = String::with_capacity(2048);
    let mut string4 = String::from("Stuff");
    string4.clear(); // after this, the same amount of bytes is allocated in memory
    string4.shrink_to_fit(); // this will shrink it since there are not buytes

    // reference a string
    // let another_str = String::from("test");
    // let string_ref = &another_str;
    // mem::drop(another_str);
    // println!("{}", string_ref); // error because I am referencing an owned value dropped from memory

    // copy (clone) a string to a new value
    let my_string = String::from("test");
    let string_ref = &my_string.clone(); // without this clone, I move the string to string_ref
    mem::drop(my_string);
    println!("{}", string_ref);

    // not sure about this tbh
    // let another_str3 = String::from("test");
    // let string_ref3 = &another_str3; // without this clone, I move the string to string_ref2

    // println!("{}", string_ref3);
} */

/* use std::mem;

fn main() {
    let string = String::from("Hello world.");
    let string_ref = &string.clone();

    mem::drop(string);

    str(string_ref);
    println!("{}", string_ref);
}

fn str(str: &String) -> &String {
    println!("{}", str);
    str
} */

/* fn main() {
    let arr: [i32; 4294967295] = [0; u32::MAX as usize]; // overflow will happen
    println!("{}", arr[1000]);
} */
