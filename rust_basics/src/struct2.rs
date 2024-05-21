
use std::fmt::Debug;

struct Rect{
    width: u32,
    height: u32,    
}

// impl Rect{
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }
impl Debug for Rect{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Rectange of width {} and height {}", self.width, self.height)
    }

}

fn main(){
    println!("Hello rustians");
    let rect = Rect{
        width : 30,
        height : 50
    };
    // println!("Area of the reactangle is {}", rect.area());
    println!("{:?}", rect);                 // This will print the debug output of the struct and {:?} is used to print the debug output of the struct
}