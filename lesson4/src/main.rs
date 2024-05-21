fn main() {
    /* // named structs
    struct Point3D {
        x: f32,
        y: f32,
        z: f32,
    }

    // named structs which we can mutate
    let mut point = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    point.x = 4.0;

    println!("point is:{}, {}, {}", point.x, point.y, point.z); */
    // ------------------------------------------------------------------

    /* // tuple structs
    struct Color(u8, u8, u8);

    let red = Color(255, 0, 0);
    println!("red is: {}, {}, {}", red.0, red.1, red.2); */
    // ------------------------------------------------------------------

    /* // tuple
    let tuple_example = (1, "Hello", 4.5);
    println!("First value: {}", tuple_example.0);
    println!("Second value: {}", tuple_example.1);

    // destructure a tuple
    let (x, y, z) = tuple_example;
    println!("Tule values --->  x: {}, y: {}, z: {}", x, y, z); */
    // ------------------------------------------------------------------

    /* //enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    // using enum
    let msg = Message::Write(String::from("Hello"));

    // matching - run different code for different variants of an enum

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    } */
    // ------------------------------------------------------------------

    /* // try my own match
    enum Greet {
        Morning(String),
        MorningMultiple { him: String, her: String },
        Birthday(String, i8),
    }

    let morning = Greet::Morning(String::from("Mike"));

    let morningMultiple = Greet::MorningMultiple {
        him: String::from("Marcus"),
        her: String::from("Valeria"),
    }; // some error here

    let birthday = Greet::Birthday(String::from("John"), 12);

    match morningMultiple {
        Greet::Morning(name) => {
            println!("Good morning {}", name);
        }
        Greet::MorningMultiple { him, her } => {
            println!("Good morning to {} and {}", him, her);
        }
        Greet::Birthday(name, age) => {
            println!("Happy birthday {}, you are {}!", name, age);
        }
    } */
    // ------------------------------------------------------------------

    /* // defining associated methods on structs
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn square(&mut self) {
            self.width = self.height;
        }

        fn set_width(&mut self, width: u32) -> &mut Self {
            self.width = width;
            self
        }

        fn set_height(&mut self, height: u32) -> &mut Self {
            self.height = height;
            self
        }
    }

    // new instance of rect
    let mut rect = Rectangle {
        width: 20,
        height: 5,
    };

    // getting current area
    println!("Area: {}", rect.area()); // 100

    // making it into a square
    rect.square();
    println!("Square: {}", rect.area()); // 25

    // set height and width
    rect.set_height(1000).set_width(2);
    println!("Square: {}", rect.area()); // 25 */
    // ------------------------------------------------------------------

    /* // defining functions on structs
    // they don't take self as a parameter. associated with the type, but not a particular instance of a type
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(20);
    println!("width: {}, height: {}", sq.width, sq.height); */
    // ------------------------------------------------------------------

    // continue from here:
    // https://robot-dreams-rust.mag.wiki/4-structs-enums-patterns/index.html#4-advanced-control-flow

    // if  - don't really get this
    /* let some_option: Option<i32> = Some(5);
       if let Some(x) = some_option {
           println!("Value inside Some: {}", x);
       } else {
           println!("It was None!");
       }

       // while let
       let mut stack = vec![1, 2, 3];
       while let Some(top) = stack.pop() {
           println!("{}", top);
       }
    */
    // ------------------------------------------------------------------

    // let else - partally working but can't handle error
    /* fn test() -> Option<i32> {
        let some_option = Some(5);
        let Some(4) = some_option else {
            return Err("The option was none");
        };
        some_option
    }
    test(); */
}
