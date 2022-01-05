fn main() {
    println!("Hello, world!");
    another_world(5, 'd');
}

fn another_world(x: i32, want: char) {
    println!("Hello, othr world {} {}!", x, want);

    let y = {
        let x = 3;
        x + 7
    };

    println!("The value of y is: {}", y);
}
