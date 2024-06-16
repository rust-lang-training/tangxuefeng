use rand::Rng;
fn main () {
    println!("随机从1-100个数之前生成10个随机数进行冒泡排序");
    let mut arr: Vec<u32> = vec![];
    for _ in 1..=10 {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        arr.push(secret_number)
    }
    println!("生成的10个随机数的数组是{:?}", arr);
    let mut i = 0;

    while i < arr.len() - 1 {
        let mut j = i + 1;
        while j < arr.len(){
            if arr[j] < arr[i] {
                arr.swap(j, i);
            }
            j = j + 1
        }
        i = i + 1;
    }
    println!("排序之后的数组{:?}", arr)
}