use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; 2_i32.pow(n)],
    }
    let matches: Vec<((usize, u32), (usize, u32))> = (0..a.len() / 2)
        .map(|i| ((i * 2, a[i * 2]), (i * 2 + 1, a[i * 2 + 1])))
        .collect();
    println!("{}", execute_match(matches));
}

fn execute_match(matches: Vec<((usize, u32), (usize, u32))>) -> usize {
    let results: Vec<(usize, u32)> = matches.iter().map(|battle| sub_battle(*battle).0).collect();
    let next_matches: Vec<((usize, u32), (usize, u32))> = (0..results.len() / 2)
        .map(|i| (results[i * 2], results[i * 2 + 1]))
        .collect();
    match next_matches.len() {
        1 => {
            let semi_champion = sub_battle(next_matches[0]).1;
            semi_champion.0 + 1
        }
        _ => execute_match(next_matches),
    }
}

fn sub_battle(battle: ((usize, u32), (usize, u32))) -> ((usize, u32), (usize, u32)) {
    let a = battle.0;
    let b = battle.1;
    match a.1 > b.1 {
        true => (a, b),
        false => (b, a),
    }
}
