// //use cargo expand command to show the expanded code of macro usage

// //this is a declarative macro that prints a name
// macro_rules! say_name {
//     () => {
//         println!("Hello, my name is pranav!");
//     };
// }
// fn main() {
//    say_name!();

//    let name = String::from("Pranav");

//    println!("{:?}", name); // debug
//    println!("{}", name); //  display
// }


//this is a procedural macro

use std::fmt::{Display, Debug};

// #[derive(Debug)]
struct User{
    name: String, 
    age: u32,
}

//this is a custom implementation of display trait
impl Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User name: {}", self.name)
    }
}

//this is a custom implementation of debug trait
impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ name: {}, age: {} }}", self.name, self.age)
    }
}


fn main(){
    let user = User{
        name: String::from("Pranav"),
        age: 25,
    };

    println!("{:?}", user);
    println!(" user name is {}", user);
}