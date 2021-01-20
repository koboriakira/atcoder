fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let result = if (a - b).abs() < 3 { "Yes" } else { "No" };
    println!("{}", result.to_string());
}
