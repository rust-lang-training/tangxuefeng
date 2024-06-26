use std::io;
fn main () {
    println!("输入要求对应项的斐波那契数值");
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("读取输⼊失败");
    let ipt: u32= input.trim().parse().expect("请输入整数值");
    println!("{:?}", ipt);
    if ipt == 0 {
        println!("第0项is0");
    } else if ipt == 1 {
        println!("第1项is1");
    } else if ipt == 2 {
        println!("第2项is1");
    } else {
        let mut prev = 1;
        let mut pprev = 1;
        for _i in 3..=ipt{
            let s = pprev;
            pprev = prev + pprev;
            prev = s
        }
        println!("第{ipt}的斐波那契的值是{pprev}");
    }
}

// fn main () {
//     println!("输入要求对应项的斐波那契数值");
//     let mut input = String::from("");
//     io::stdin().read_line(&mut input).expect("读取输⼊失败");
//     let ipt: u32= input.trim().parse().expect("请输入整数值");
//     println!("第{ipt}项的斐波那契值is{:?}", fib(ipt));
// }
// fn fib (n: u32) -> u32{
//     if n == 0 {
//        return 0;
//     } 
//     if n == 1 || n == 2 {
//         return 1;
//     }
//     fib(n - 1) + fib(n - 2)
// }



