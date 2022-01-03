fn main() {
    const SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3;
    let mut val = 8;
    println!("The value of val is {}", val);
    val = 9;
    println!("The value of val is {}", val);
    println!("The value of SITH is {}", SECONDS_IN_THREE_HOURS);

    let x = 5;
    let x = x + 32;
    {
        let x = x * x;
        println!("The value of x is {}", x);
    }
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    let guess: i32 = "84".parse().expect("Invalid guess");
    println!("The value of guess is {}", guess);

    let y = 5.5;
    let z: f32 = 3.34;
    let _w = y * z + z / 4.4;

    println!("The value of y is {} and z {}", y, _w);
    let x = (3, 4, 5, 6);
    println!("The value of x[0] {}", x.0);
    let b = ["monday"; 3];
    println!("The value of b[0] {}", b[2]);
}
