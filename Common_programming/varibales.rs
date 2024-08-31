fn main(){
    // declare a varible  
    // every variables in the rust are immutable that means we cant change the value 
    //  mut  keyword help us to  chage the  value for specific varible
    let x = 15 ;
    println!("The value of x is  {x}");

    // here the x will be shadowed by the upcoming  x by rediclare it  ,
    // it helps us to  chage the types of x  as well
    // mut cant able to change the data type after diclaration

    let x = "fifteen";
    println!("the value of  x is {x}");

    // Const varibale   
    // cant change  
    // Uppercase Naming Convention
    // mut  cant help here 
    // data anotation  is  important on the const , you have to specify  the types of the varibles
    const SUBCRIBER_COUNT : i64 = 1000_00;
    println!("The value  of Sub Count is  {}",SUBCRIBER_COUNT)
 }