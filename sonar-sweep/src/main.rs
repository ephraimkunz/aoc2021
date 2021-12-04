
fn main() {
    part1();
    part2();
}

fn part1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let numbers = input
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let num_increasing = numbers
        .windows(2)
        .fold(0, |acc, n| if n[0] < n[1] { acc + 1 } else { acc });
    println!("part1: {}", num_increasing);
}

fn part2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let numbers = input
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let three_window_sums = numbers
        .windows(3)
        .map(|n| n.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    let num_increasing =
        three_window_sums
            .windows(2)
            .fold(0, |acc, n| if n[0] < n[1] { acc + 1 } else { acc });
    println!("part2: {}", num_increasing);
}
