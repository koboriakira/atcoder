fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    match (0..a.len()).map(|i| a[i] * b[i]).sum::<i32>() {
        0 => println!("Yes"),
        _ => println!("No"),
    }
}
