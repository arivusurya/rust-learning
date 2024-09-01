use std::io;
fn main(){
    println!("Enter the number");
    let mut number =   String::new();
    // println!("{number}") 
    io::stdin().read_line(&mut number).expect("Thank you");
    let number : u32 = number.trim().parse().expect("Input not an interger");
    // println!("{number}")
    let output  =  if number > 50 {"Greater "}else { "smaller" };
    println!("The number is  {output} than 50") 
}