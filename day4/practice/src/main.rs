// trait Summary {
//     fn summarize(&self) -> String;
// }

// struct NewsArtical {
//     headline: String,
//     location: String,
//     author: String,
//     content: String,
// }
// impl Summary for NewsArtical {
//     fn summarize(&self) -> String {
//         format!(
//             "{}, by {} ({})",
//             &self.headline,
//             &self.author,
//             &self.location,
//         )
//     }
// }
// struct Tweet {
//     username: String,
//     content: String,
//     reply: bool,
//     retweet: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {} ", &self.username, &self.content)
//     }
// }
// fn notify<T>(item: &T)
// where
//     T: Summary,
// {
//     println!("Breaking news! {}", item.summarize())
// }
// fn main() {
//     println!("Hello, world!");
// }

// #[derive[Debug]]
// struct Point {
//    x: f32,
//    y: f32
// }
// trait Display  {
    
// }
// impl  Display for  Point{
    
// }
// fn main () {
//     let p = Point {
//         x: 1.0,
//         y: 1.0
//     };
//     println!("{:?}", p)
// }
// use std::io;
// use std::path::Path;

// fn test () {
//     let _ = check_file("/path/to/the/file.ext");
//     let _ = check_file("/path/to/the/file.ext".to_string());
// }
// fn check_file<P: AsRef<Path>>(_path: P) -> Result<(), io::Error>{
//     // let result = _path.as_ref();
//     Ok(())
// }
// fn main () {
//     test();
// }

// struct Rectangle<T> {
//     width: T,
//     height: T
// }
// impl <T :std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T> + From<u32>> Rectangle<T> {
//     fn area (&self)  -> T{
//         self.width * self.height
//     }
//     fn perimeter (&self)  -> T{
//         // self.width + self.height + self.width + self.height
//         (self.width + self.height) * (T::from(2))
//     }
// }
// fn main () {
  
// }

// fn main () {
//     let x = 42;
//     let c1 = || println!("Hello World");
//     let c2  = || println!("x = {}", x);
//     let c3 = || {
//         let s = String::from("Hello local variable");
//         println!("local variable s = {}", s);
//     };
//     c1();
//     c2();
//     c3();
// }


// fn main () {
//     let mut s = String::from("Hello");
//     let mut c1 = || s.push_str(" Rust closure");
//     c1();
//     println!("s = {}", s);
// }

// fn main () {
//     let s = String::from("Hello World");
//     let closure = move || {
//         let tp = (s, 1);
//         println!("{:?}", tp)
//     };
//     closure();
//     // println!("{}", s);
// }

fn main () {
    let add_one = |x : i32| x + 1;
    let y = add_one(10);
    println!("y = {:?}", y);
}