fn max_value(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &val in slice.iter() {
        if val > max {
            max = val;
        }
    }
    max
}

fn min_value(slice: &[i32]) -> i32 {
    let mut min = slice[0];
    for &val in slice.iter() {
        if val < min {
            min = val;
        }
    }
    min
}

fn max_value_ref(slice: &[i32]) -> &i32 {
    let mut max = &slice[0];
    for val in slice.iter() {
        if val > max {
            max = val;
        }
    }
    max
}

fn min_value_ref(slice: &[i32]) -> &i32 {
    let mut min = &slice[0];
    for val in slice.iter() {
        if val < min {
            min = val;
        }
    }
    min
}

fn average_value(slice: &[i32]) -> f64 {
    let mut sum = 0;
    for &val in slice.iter() {
        sum += val;
    }
    sum as f64 / slice.len() as f64
}

fn double_values(slice: &mut [i32]) {
    for v in slice  {
        *v = *v  * 2;
    }
}

fn main() {
    let mut arr = [1,2,4,3,5,6,7,8,0,10];
    println!("Max value: {}", max_value(&arr));
    println!("Min value: {}", min_value(&arr));
    println!("Max value reference: {}", max_value_ref(&arr));
    println!("Min value reference: {}", min_value_ref(&arr));
    println!("Average value: {}", average_value(&arr));
    
    double_values(&mut arr);
    println!("Doubled values: {:?}", arr);
}
