use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        price: [usize; n],
        connects: [[usize;2]; m],
    }
    let mut connect_roads: Vec<Vec<usize>> = vec![];
    let entry_points: HashSet<Vec<usize>> = connects.iter().map(|r| vec![r[0]]).collect();
    for road in entry_points {
        connect_road(&road, &connects, connect_roads.as_mut());
    }
    let buy_and_sale_points: Vec<(usize, usize)> = connect_roads
        .iter()
        .filter_map(|roads| {
            if roads.len() > 1 {
                Some((roads[0] - 1, *roads.last().unwrap() - 1))
            } else {
                None
            }
        })
        .collect();
    let profits: Vec<i64> = buy_and_sale_points
        .iter()
        .map(|(buy_point, sale_point)| price[*sale_point] as i64 - price[*buy_point] as i64)
        .collect();
    if let Some(max) = profits.iter().max() {
        println!("{}", max);
    } else {
        println!("0");
    }
}

fn connect_road<'a>(
    roads: &'a Vec<usize>,
    connects: &Vec<Vec<usize>>,
    connect_roads: &'a mut Vec<Vec<usize>>,
) {
    let next_entry_points: Vec<usize> = connects
        .iter()
        .filter(|c| c[0].eq(roads.last().unwrap()))
        .map(|c| c[1])
        .collect();
    for next_entry_point in next_entry_points {
        let mut roads = roads.clone();
        roads.push(next_entry_point);
        connect_road(&roads, connects, connect_roads);
    }
    connect_roads.push(roads.clone());
}
