use std::io;

fn main() {
    println!("请输入一段文本");
    // 读取输入文本
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("输入的文本 {input}");
    // 创建一个长度为 26 的数组来存储每个字母的出现次数，初始值为 0
    let mut letter_counts = [0; 26];

    // 遍历输入文本中的每个字符
    for ch in input.chars() {
        // 转换大写字母为小写字母
        let lower_ch = ch.to_ascii_lowercase();
        // 仅统计小写字母
        if lower_ch.is_ascii_lowercase() {
            let index = (lower_ch as usize) - ('a' as usize);
            letter_counts[index] += 1;
        }
    }

    // 输出结果
    for (i, &count) in letter_counts.iter().enumerate() {
        let letter = (i as u8 + b'a') as char;
        println!("{} => {}", letter, count);
    }
}