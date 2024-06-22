use std::io::{self, Write};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input = String::new();

    // 获取用户输入的两个数字，以空格分隔
    print!("请输入两个 2^31-1 以内的正整数，以空格分隔: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 解析输入的两个数字
    let mut numbers = input.trim();
    println!("获取输入的整数 {:?}", numbers);
    let num1: u32 = numbers.next().expect("请输入第一个数字").parse().expect("请输入一个有效的正整数");
    let num2: u32 = numbers.next().expect("请输入第二个数字").parse().expect("请输入一个有效的正整数");

    // 计算并输出GCD
    let result = gcd(num1, num2);
    println!("{} 和 {} 的最大公约数是 {}", num1, num2, result);
}