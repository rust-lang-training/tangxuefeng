
use std::io;

fn main () {
    let mut arr = [0,0,0,0,0];
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("读取输⼊失败");
    let ipt = input.trim().split(" ");
    let mut index = 0;
    for i in ipt {
        let ii = i.parse().expect("输入有效字符");
        arr[index] = ii;
        index = index + 1;
        if (index == 5) {
            break;
        }
    }
    let sum = sum_fn(&arr);
    println!("arr is {:?}",arr);
    println!("sum is {:?}",sum);
}
fn sum_fn (arr: &[i32]) -> i32{
    let mut sum: i32 = 0;
    for n in arr {
        sum  = sum + n
    }
    sum
}