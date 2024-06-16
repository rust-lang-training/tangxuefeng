use std::io;
fn main() {
    loop{
        println!("摄⽒度 --> 华⽒度请输⼊ 1");
        println!("华⽒度 --> 摄⽒度请输⼊ 2");
        println!("退出请输⼊ 0");
        println!("\n你的选择是：");
        let mut input  = String::from("");
        let _ = io::stdin().read_line(&mut input).expect("输入字符串错误");
        let choice = input.trim();
        if choice == "0" {
            break;
        }
        if choice == "1" {
            println!("\n请输⼊摄⽒度：");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("读取输⼊失败");
            let c: f64 = c.trim().parse().expect("请输⼊有效的数值");
            let f = c_to_f(c);
            println!("{:.2} 摄⽒度 --> {:.2} 华⽒度\n", c, f);
        }
        if choice == "2" {
            println!("\n请输⼊华⽒度：");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("读取输⼊失败");
            let c: f64 = c.trim().parse().expect("请输⼊有效的数值");
            let f = f_to_c(c);
            println!("{:.2} 华⽒度 --> {:.2} 摄⽒度\n", c, f);
        }
    }
}
fn c_to_f(c: f64) -> f64{
  c * 1.8 + 20.0
}
fn f_to_c(f: f64) -> f64{
    (f - 20.0) / 1.8
}