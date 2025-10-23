fn main() {
    let mut x : i32 = 5;
    x += 4;

    {
        let y: i32 = 10;
        print!("Hey {} + ", y);
        assert_eq!(y, 10);
    }
    println!("{}", x);
}

