use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: u64,
        subscription_price: u64,
        array: [[u64;3];n],
    }
    let mut event_days: Vec<u64> = array
        .iter()
        .map(|el| vec![el[0], el[1], el[1] + 1])
        .flatten()
        .collect::<HashSet<u64>>()
        .iter()
        .map(|d| *d)
        .collect();
    // event_days.push(event_days.iter().max().unwrap() + 1_u64);
    event_days.sort();
    let event_days: Vec<(u64, u64)> = event_days.windows(2).map(|d| (d[0], d[1] - d[0])).collect();
    // println!("{:?}", event_days);
    let mut result = 0_u64;
    for (begin_day, days) in event_days {
        let day_cost = array
            .iter()
            .filter(|el| el[0] <= begin_day && begin_day <= el[1])
            .map(|el| el[2])
            .sum::<u64>()
            .min(subscription_price);
        // println!("day_cost: {}", day_cost);
        // println!("days: {}", days);
        result += day_cost * days;
    }
    println!("{}", result);

    // for (begin_day, end_day) in days.windows(2).into_iter() {}
    // let min_day = array.iter().map(|el| el[0] as u32).min().unwrap();
    // let max_day = array.iter().map(|el| el[1] as u32).max().unwrap();
    // for day in min_day..max_day + 1 {
    //     let day_cost = array
    //         .iter()
    //         .filter(|el| el[0] <= day && day <= el[1])
    //         .map(|el| el[2])
    //         .sum::<u32>()
    //         .min(subscription_price);
    //     result += day_cost;
    // }
}
