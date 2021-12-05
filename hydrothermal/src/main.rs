use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let segments: Vec<_> = input
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(" -> ").unwrap();
            let (x1, y1) = first.split_once(',').unwrap();
            let (x2, y2) = second.split_once(',').unwrap();
            (
                (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap()),
                (x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap()),
            )
        })
        .collect();

    let mut point_counts = HashMap::new();
    for segment in &segments {
        if segment.0 .0 == segment.1 .0 {
            for y in std::cmp::min(segment.0 .1, segment.1 .1)
                ..=std::cmp::max(segment.0 .1, segment.1 .1)
            {
                let entry = point_counts.entry((segment.0 .0, y)).or_insert(0);
                *entry += 1;
            }
        } else if segment.0 .1 == segment.1 .1 {
            for x in std::cmp::min(segment.0 .0, segment.1 .0)
                ..=std::cmp::max(segment.0 .0, segment.1 .0)
            {
                let entry = point_counts.entry((x, segment.0 .1)).or_insert(0);
                *entry += 1;
            }
        }
    }

    let num_overlapping: usize = point_counts.values().filter(|&&c| c > 1).count();
    println!("part1: {}", num_overlapping);

    let mut point_counts_2 = HashMap::new();
    for segment in &segments {
        if segment.0 .0 == segment.1 .0 {
            for y in std::cmp::min(segment.0 .1, segment.1 .1)
                ..=std::cmp::max(segment.0 .1, segment.1 .1)
            {
                let entry = point_counts_2.entry((segment.0 .0, y)).or_insert(0);
                *entry += 1;
            }
        } else if segment.0 .1 == segment.1 .1 {
            for x in std::cmp::min(segment.0 .0, segment.1 .0)
                ..=std::cmp::max(segment.0 .0, segment.1 .0)
            {
                let entry = point_counts_2.entry((x, segment.0 .1)).or_insert(0);
                *entry += 1;
            }
        } else {
            // Diagonal. Arbitrarily point with lower x is start point.
            let start_x = std::cmp::min(segment.0 .1, segment.1 .1);
            let start_point = if start_x == segment.0 .1 {
                segment.0
            } else {
                segment.1
            };
            let end_point = if start_point == segment.0 {
                segment.1
            } else {
                segment.0
            };

            let y_increments = start_point.0 < end_point.0;
            if y_increments {
                let points = (start_point.0..=end_point.0).zip(start_point.1..=end_point.1);
                for point in points {
                    let entry = point_counts_2.entry(point).or_insert(0);
                    *entry += 1;
                }
            } else {
                let points = (end_point.0..=start_point.0)
                    .rev()
                    .zip(start_point.1..=end_point.1);
                for point in points {
                    let entry = point_counts_2.entry(point).or_insert(0);
                    *entry += 1;
                }
            };
        }
    }

    let num_overlapping: usize = point_counts_2.values().filter(|&&c| c > 1).count();
    println!("part2: {}", num_overlapping)
}
