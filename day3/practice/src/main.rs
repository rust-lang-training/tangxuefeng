// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s);
//     let x = 5;
//     makes_copy(x)
// }
// fn takes_ownership (some_string: String) {
//     println!("{:?}", some_string);
// }
// fn makes_copy (some_interger: i32) {
//     println!("{:?}", some_interger);
// }

// fn main () {
//     let names = [
//         String::from("John"),
//         String::from("Tom"),
//         String::from("Penny"),
//         String::from("Sheldon")
//     ];
//     for i in 0..4 {
//       let s = &names[i];
//       println!("{:?}", s)
//     }
//     println!("name[0] = {:?}". names[0]);
// }

// fn main () {
//     let s = String::from("hello world");
//     let bytes = s.into_bytes();
//     println!("{:?}", s);
// }


// fn main () {
//     let x = 10;
//     let y = &x;
//     assert!(*y == 10);
//     assert!(y == &10);
// }

// fn main () {
//     let mut x = 10;
//     let y = &mut x;
//     *y = 20;
//     println!("{:?}", y);
// }

// fn main () {
//     let s1 = String::from("hello word");
//     get_len(&s1);

// }
// fn get_len (s: &String) -> usize {
//     println!("字符串长度 {:?}", s);

//     println!("字符串长度是{:?}", s.len());
//     let s1 = &s;
//     println!("字符串长度是{:?}", s1.len());
//     s.len();
// }

// fn main () {
//     let mut s = String::from("hello");
//     let s = "123";
//     let s1 = &s;
//     let s2 = &s;
//     let s3 = &mut s;
//     let a = "123";
//     // println!("{:?} {:?} adn {:?}", s1, s2 , s3);
// }

// fn main () {
//     let mut s = String::from("hello word");
//     let rs = &s;
//     s.push_str(" i am rust ");
//     println!("The string is : {:?}", rs);
// }


// fn main () {
//     let s1 = String::from("hello word");
//     let rs1 = &s1;
//     let s2 = s1;
//     println!("string is {:?}", rs1)
// }

// fn main () {
//   let binding = gen_string();
//   let bytes = binding.as_bytes();
//   println!("bytes are: {:?}", bytes)
// }
// fn gen_string () -> String{
//     String::from("hellow world")
// }

// use std::rc::Rc;
// fn main () {
//     let s: Rc<String> = Rc::new(String::from("shirataki"));
//     println!("ref count: {}", Rc::strong_count(&s));
//     {
//         let t: Rc<String> = s.clone();
//         println!("ref count: {}", Rc::strong_count(&t));

//     }
//     let u: Rc<String> = s.clone();
//     println!("ref count: {}", Rc::strong_count(&s));

//     println!("ref count: {}", Rc::strong_count(&u));
// }

// fn main () {
//     let b = Box::new(5);
//     println!("b = {}", b);
    
// }

// 内部可变形
// Cell RefCell

// use std::cell::RefCell;
// fn main () {
//     let s = RefCell::new(String::from("hello word"));
//     append_string(&s);
//     println!("s is : {}", s.borrow());
// }
// fn append_string (s: &RefCell<String>) {
//     let rs = s.borrow();
//     let mut ms = s.borrow_mut();
//     (*ms).push_str(" i am rust");
//     println!("rs : {}", rs);
// }
// #[derive(Debug)]
// struct  User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn main () {
//     let user = User {
//         active: true,
//         username: String::from("唐雪峰"),
//         email: String::from("1852157055@qq.com"),
//         sign_in_count: 0
//     };
//     let User {
//         active,
//         username,
//         email,
//         sign_in_count
//     } = user;
//     println!("姓名: {:?} 是否在校: {:?} 邮箱是 {:?} sign_in_count: {:?}", username, active, email, sign_in_count);
//     // println!("user is {:?}", user);
// }

// #[derive(Debug)]
// struct  User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }
// fn main () {
//     let mut user = User {
//         active: true,
//         username: String::from("唐雪峰"),
//         email: String::from("1852157055@qq.com"),
//         sign_in_count: 0
//     };
//     user.active = false
// }

// #[derive(Debug)]

// struct Point(f32, f32);
// fn main () {
//     let p = Point(10.0, 11.0);
//     println!("point: ({}, {})", p.0, p.1);
//     let Point(x, y) = p;
//     println!("point({:?} {:?})", x, y);
// }

// #[derive(Debug)]
// struct  Recttangle {
//     width: f32,
//     height: f32
// }
// impl  Recttangle {
//     fn square (size: f32)  -> Self {
//         Self {
//             width: size,
//             height: size 
//         }
//     }
//     fn new (width: f32, height: f32) -> Self {
//         Self {
//             width,
//             height 
//         }
//     }
//     fn perimeter (&self)  -> f32{
//         (self.width + self.height) * 2.0
//     }
//     fn area (&self)  -> f32{
//         self.width * self.height
//     }
//     fn scale (&mut self, width_scale: f32, height_scale: f32) {
//         self.width  = self.width * width_scale;
//         self.height  = self.height * height_scale;
//     }
// }
// fn main () {
//    let mut tecttangle = Recttangle::new(1.0, 2.0);
//    println!("面积{}", tecttangle.area());
//    println!("周长{}", tecttangle.perimeter());
//    tecttangle.scale(2.0, 2.0);
//    println!("面积{}", tecttangle.area());
//    println!("周长{}", tecttangle.perimeter());

// }

// #[derive(Debug)]
// #[warn(dead_code)]
// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }
// fn main () {
//     let q = Message::Quit;
//     let m = Message::Move {x: 100, y: 100};
//     let w = Message::Write(String::from("hello word!!!"));
//     let c = Message::ChangeColor(255, 0 , 0);
//     println!("{:?} {:?} {:?} {:?}", q, m, w, c);
// }

// use std::cmp::Ordering::*;
// fn describe_point(x: i32, y: i32) -> &'static str {
//     match (x.cmp(&0), y.cmp(&0)) {
//         (Equal, Equal) =>  "原点",
//         (_, Equal) =>  "x轴",
//         (Equal, _) =>  "y轴",
//         (Greater, Greater) =>  "一象限",
//         (Less, Greater) =>  "二象限",
//         _ => "其他"
//     }
// }
// fn main () {
//     println!("{}", describe_point(0,0));
//     println!("{}",describe_point(5,0));
//     println!("{}",describe_point(0,5));
//     println!("{}",describe_point(5,5));
//     println!("{}",describe_point(-5,5));
//     println!("{}",describe_point(-5,-5));

// }
// use std::io;
// fn main () {
//     println!("输入要求对应项的斐波那契数值");
//     let mut input = String::from("");
//     io::stdin().read_line(&mut input).expect("读取输⼊失败");
//     let ipt: u32= input.trim().parse().expect("请输入整数值");
//     println!("第{ipt}项的斐波那契值is{:?}", fib(ipt));
// }
// fn fib (n: u32) -> u32{
//     match n {
//       0 => 0,
//       1 => 1,
//       2 => 1,
//       _ => fib(n - 1) + fib(n - 2)
//     }
// }
fn main () {
    let s = "alkasd7890anjas^&#^fg";
    let mut iter = s.chars();
    loop {
        match iter.next() {
            Some(n @ '0' ..= '9') => println!("数字 {:?}", n),
            Some(n @ ('a' ..= 'z' | 'A' ..= 'Z')) => println!("字符 {:?}", n),
            Some(c) => println!("其他 {:?}", c),
            None => {
                println!("结束");
                break;
            }
        }
    }
}



