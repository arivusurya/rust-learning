#[derive(Debug)]
struct Rectangle {
    width  : u32,
    height : u32
}

impl Rectangle {
    fn area(&self)-> u32{
        self.width *  self.height
    }
}

impl Rectangle {
     fn square(size : u32)-> Rectangle {
        Rectangle {
            width : size,
            height :size
        }
     }
}

fn main(){
    let rect = Rectangle{
        width :40,
        height : 40
    
    };
    println!( "{:?} ",rect);
    println!( "{:?} ",Rectangle::square(32));
    println!("The area  of rect  is  {}" , rect.area())
}