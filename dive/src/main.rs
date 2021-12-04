fn main() {
    part1();
    part2();
}

fn part1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let vals: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let splitted = l.split_once(' ').unwrap();
            (splitted.0, splitted.1.parse().unwrap())
        })
        .collect();

    let horizontal: i32 = vals
        .iter()
        .filter(|(v, _)| v == &"forward")
        .map(|v| v.1)
        .sum();
    let depth =
        vals.iter()
            .filter(|(v, _)| v != &"forward")
            .fold(0, |accum, element| match element.0 {
                "up" => accum - element.1,
                _ => accum + element.1,
            });

    println!("part1: {}", horizontal * depth)
}

fn part2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let vals: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let splitted = l.split_once(' ').unwrap();
            (splitted.0, splitted.1.parse().unwrap())
        })
        .collect();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for item in vals {
        match item.0 {
            "forward" => {
                horizontal += item.1;
                depth += aim * item.1;
            }
            "down" => aim += item.1,
            _ => aim -= item.1,
        }
    }

    println!("part2: {}", horizontal * depth)
}
