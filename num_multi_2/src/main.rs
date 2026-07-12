fn main() {
   let mut number = String::new();
   std::io::stdin().read_line(&mut number).expect("Error!");
   let input_number: i32 = number.trim().parse().expect("Error!");
    println!("The number you typed is: {}", input_number * 2);
    

}
   