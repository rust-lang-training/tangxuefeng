
use std::io;
fn main () {
    println!("输入数据");
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("读取输⼊失败");
    let ipt: i32= input.trim().parse().expect("请输入整数值");
    if ipt == 0 || ipt == 1 {
        println!("{ipt} 不是指数")
    } else {
        for i in 2..=ipt {
            if ipt  % i * i  == 0  && i * i <= ipt{
               println!("{ipt} 不是质数");
               return;
            }
        } 
        println!("{ipt}是质数");
    }
}
