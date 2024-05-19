fn main() {
    let mut count = 0;
    loop {
        if count > 5 {
            break;
        }
        println!("{count}");
        count += 1;
    }
}
