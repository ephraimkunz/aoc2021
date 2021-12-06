fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut fish: Vec<usize> = input.split(',').map(|f| f.parse().unwrap()).collect();
    let fish2 = fish.clone();

    for _ in 0..80 {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }
    }

    println!("part1: {}", fish.len());

    let mut counts = [0; 9];
    for i in fish2 {
        counts[i] += 1;
    }

    for _ in 0..256 {
        let mut counts2 = [0; 9];

        counts2[8] = counts[0];
        counts2[6] = counts[0] + counts[7];

        counts2[0] = counts[1];
        counts2[1] = counts[2];
        counts2[2] = counts[3];
        counts2[3] = counts[4];
        counts2[4] = counts[5];
        counts2[5] = counts[6];
        counts2[7] = counts[8];

        counts = counts2;
    }

    println!("part2: {}", counts.iter().sum::<usize>());
}
