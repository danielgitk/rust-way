fn main() {
    struct Test(i32, i32, i32);
    struct Help {
        name: String,
        description: String,
        age: i32,
    }

    let stud1 = Help {
        name: String::from("dani man"),
        description: String::from("hallelujah"),
        age: 3,
    };
    let stud2 = Test(43, 53, 23);

    println!(
        "{} {} {} {} {}",
        stud1.age, stud1.description, stud1.name, stud2.2, stud2.1
    )
}
