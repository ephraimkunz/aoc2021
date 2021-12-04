use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let num_bits = input.lines().next().unwrap().len();

    let mut counts: Vec<(usize, usize)> = vec![];
    for _ in 0..num_bits {
        counts.push((0, 0));
    }
    for line in input.lines() {
        for (i, char) in line.chars().rev().enumerate() {
            if char == '0' {
                counts[i] = (counts[i].0 + 1, counts[i].1);
            } else {
                counts[i] = (counts[i].0, counts[i].1 + 1);
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, item) in counts.iter().enumerate() {
        let (zero, one) = item;
        if one > zero {
            gamma += 1 << i;
        }

        if one < zero {
            epsilon += 1 << i;
        }
    }

    println!("{}", gamma * epsilon);

    let mut o2_candidates = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect::<HashSet<_>>();
    let mut co2_candidates = o2_candidates.clone();

    for i in (0..num_bits).rev() {
        counts.clear();
        for _ in 0..num_bits {
            counts.push((0, 0));
        }
        for candidate in o2_candidates.iter() {
            for (i, item) in counts.iter_mut().enumerate() {
                if ((1 << i) & candidate) == 0 {
                    *item = (item.0 + 1, item.1);
                } else {
                    *item = (item.0, item.1 + 1);
                }
            }
        }

        if o2_candidates.len() == 1 {
            break;
        }

        if counts[i].0 > counts[i].1 {
            o2_candidates.retain(|c| ((!c) & (1 << i)) != 0);
        } else {
            o2_candidates.retain(|c| (c & (1 << i)) != 0);
        }
    }

    for i in (0..num_bits).rev() {
        counts.clear();
        for _ in 0..num_bits {
            counts.push((0, 0));
        }
        for candidate in co2_candidates.iter() {
            for (i, item) in counts.iter_mut().enumerate() {
                if ((1 << i) & candidate) == 0 {
                    *item = (item.0 + 1, item.1);
                } else {
                    *item = (item.0, item.1 + 1);
                }
            }
        }

        if co2_candidates.len() == 1 {
            break;
        }

        if counts[i].0 <= counts[i].1 {
            co2_candidates.retain(|c| ((!c) & (1 << i)) != 0);
        } else {
            co2_candidates.retain(|c| (c & (1 << i)) != 0);
        }
    }

    println!(
        "{}",
        o2_candidates.iter().next().unwrap() * co2_candidates.iter().next().unwrap()
    );
}
