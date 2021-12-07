fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let positions: Vec<usize> = input.split(',').map(|p| p.parse().unwrap()).collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_cost = usize::MAX;
    for possible in min..=max {
        let mut possible_cost = 0;
        for i in &positions {

            // For part two, the cost is diff + diff -1 + diff - 2...
            // This has a closed form solution: (diff +1 * diff) / 2.
            let diff = std::cmp::max(i, &possible) - std::cmp::min(i, &possible);
            possible_cost += ((diff + 1) * diff) / 2;
        }

        min_cost = std::cmp::min(min_cost, possible_cost);
    }

    println!("{}", min_cost);
}
