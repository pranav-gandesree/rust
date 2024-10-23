// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Guess the number game");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("generated secreet number is {secret_number}");

//     loop {
//         println!("please enter a number ");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = guess.trim().parse().expect("failed to parse");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }




//ownership in rust

//  fn main(){
//     let s1 = String:: from("hii im pranav");
//     let s2 = s1;
//     println!("{}", s2);
// }



// fn main(){
//     let s1 = String::from("hii im pranav. understandign rustt");
//     takes_ownership(s1);
//     // println!("{}",s1); s1 cannot be used here as it goes out of scope after using it in the above fn
// }

// fn takes_ownership(s2: String){
//     println!("{}", s2);
// }



//this is without references

// fn main(){
//     let mut s1 = String::from("hii im pranav. understandign rustt");
//     s1 = takes_ownership(s1);
//     println!("{}",s1);
    
//  } //we can use s1 here as made s1 mut and store the return value in s1

// fn takes_ownership(s2: String) -> String{
//     println!("{}", s2);
//     return  s2;
// }


// borrowing and references


// fn main(){
//     let s1 = String::from("hello world from india");
//     let s2 = &s1;

//     println!("{}", s2);
//     println!("{}", s1);
// }


// fn main(){
//     let s1 = String::from("hi im pranavjew");
//     borrow_variable(&s1); // here we are passing the address of the s1 instead of s1
//     println!("{}", s1); // and it works coz we are not passing ownsership
// }

// fn borrow_variable(s2: &String){
//     println!("{}", s2);
// }


// mutable referencess

// fn main(){
//     let mut s1 = String::from("hellooo");
//     update_str(&mut s1);
//     println!("{}", s1)
// }

// fn update_str(s: &mut String){
//     s.push_str("worlddd !!");
// }



//structs and implements
 

// struct Rect {
//     width: u32,
//     height: u32,
//  }  
 
//  impl Rect {
//      fn area(&self) -> u32 {
//           self.width * self.height
//      }
//  }
 
//  fn main() {
//      let rect = Rect {
//          width: 30,
//          height: 50,
//      };
//      print!("The area of the rectangle is {}", rect.area());
//  }



//traits for structs

// pub trait Summary{
//     fn summarize(&self) -> String;
// }

// struct  User {
//     name: String,
//     age: i32
// }

// impl Summary for User {
//     fn summarize(&self) -> String {
//         return format!("age of {}, is{}", self.name, self.age);
//     }
// }

// fn main(){
//     let user = User{
//         name: String:: from("pranavvv"),
//         age: 21
//     };

//     print!("{}", user.summarize());
// }




//lifetimess

// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len(){
//         return a;
//     } else {
//         return b;
//     }
// }
// fn main(){
//     let str1 = String::from("aaaa");
//     let str2 = String::from("bbbbbbbbb");

//     longest(&str1, &str2);
// }



//serialization and deserialzation using serde

//Serialization: Convert a Rust struct into a JSON string.
//Deserialization: Convert a JSON string back into a Rust struct.


use serde::{Serialize, Deserialize};
use serde_json;

// Define a struct that will be serialized and deserialized
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

// Function to serialize a struct to JSON
fn serialize(person: &Person) -> String {
    serde_json::to_string(person).expect("Failed to serialize")
}

// Function to deserialize a JSON string to a struct
fn deserialize(json_data: &str) -> Person {
    serde_json::from_str(json_data).expect("Failed to deserialize")
}

fn main() {
    // Create an instance of the Person struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // Serialize the person struct to JSON
    let serialized_person = serialize(&person);
    println!("Serialized: {}", serialized_person);

    // Deserialize the JSON string back to a Person struct
    let deserialized_person: Person = deserialize(&serialized_person);
    println!("Deserialized: {:?}", deserialized_person);
}
