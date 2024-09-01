struct User {
    username : String,
    email : String,
    sign_in_count : u64,
    active  :bool,
}
fn main(){
    let mut user1  = User {
        email : String::from("test@gmail.com"),
        username : String::from("TestUser"),
        active : true,
        sign_in_count:1
    };

    let name =  user1.username;
    println!("{name}");
     user1.username = String::from("Ares");
     let name = user1.username;
     println!("{name}");

     let user2  =  builduser(String::from("arivusurya.3@gmail") , String::from("Ariv"));
    // println!("{}");
    let user3 = User{
        email : String::from("Test2"),
        username : String::from("test2"),
        ..user2
    };

    let rect =  (30,40);
    // let areaofrect = area(rect)
    println!("The area of the rectangle is {}" , area(rect));
    }
 
fn  area  (dimesion : (u32,u32)) -> u32 {
    dimesion.0 * dimesion.1
}


fn  builduser ( email:String ,  username : String) -> User{
    User{
        email : email,
        username : username,
        active : true,
        sign_in_count : 1
    }
}