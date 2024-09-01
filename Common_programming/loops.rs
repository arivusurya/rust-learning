use std::io;
fn main(){
    // Loops
    let mut number = 3 ;
     while number !=0{
        println!("{}!" , number);
        number -=1
    }
println!("LIFTOFF!!!");



// iterator in the array 
let a = [1,23,4,5,56,6,6788,5] ;
 for  element in a.iter(){
    println!("the value is {element}")
 }

 for  num in (1..=4){
    println!("{num}")
 }



    // println!("Please the enter the  number  of times you want to loops ? : ");

    // let mut times = String::new();
    // io::stdin().read_line(&mut times).expect("Please enter the number");
    // let  times : i32  = times.trim().parse().expect("Please enter the valid Number ");  

    // let mut counter = 0;
    // let count = loop {
    //     // println!("again!");
    //     counter +=1;
    //     if counter == times {
    //         break  counter;
    //     }
        
    // };
    // println!("The loop  runs  for {}",count)
}