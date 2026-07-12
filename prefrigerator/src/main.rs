fn main() {
    println!("Plz enter a Number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number: −273 < T ≤ 6000");

    if number > 100 {
        println!("Steam");
    } else if number < 0 {
        println!("Ice!");
    } else {
        println!("Water");
    }
}
