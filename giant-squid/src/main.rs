fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().skip(1).filter(|l| !l.is_empty()).collect();

    let numbers: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut boards: Vec<Board> = lines.chunks(5).map(Board::new).collect();
    let mut boards2 = boards.clone();

    let mut done = false;
    for number in &numbers {
        for board in boards.iter_mut() {
            if let Some(result) = board.feed(*number) {
                println!("Win first: {}", result);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    let mut wins = 0;
    let num_boards = boards2.len();
    for number in numbers {
        for board in boards2.iter_mut() {
            if let Some(result) = board.feed(number) {
                wins += 1;

                if wins == num_boards {
                    println!("last winner: {}", result);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: [[isize; 5]; 5],
    won: bool,
}

impl Board {
    fn new(lines: &[&str]) -> Self {
        let mut board = Board {
            grid: [[0; 5]; 5],
            won: false,
        };

        for (i, line) in lines.iter().enumerate() {
            for (j, num) in line
                .split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .enumerate()
            {
                board.grid[i][j] = num;
            }
        }

        board
    }

    fn all_uncovered(&self) -> usize {
        let mut sum = 0;
        for line in self.grid {
            sum += line.iter().filter(|&&v| v != -1).sum::<isize>();
        }

        sum as usize
    }

    /// Retun None if there is no bingo, else return Some
    /// with a value of final score.
    fn feed(&mut self, input: usize) -> Option<usize> {
        if self.won {
            return None;
        }

        // Use -1 to mark a covered bingo, since the input will never be negative.
        for line in self.grid.iter_mut() {
            for item in line {
                if *item == input as isize {
                    *item = -1;
                }
            }
        }

        // Check rows.
        for line in self.grid.iter() {
            if line.iter().all(|&i| i == -1) {
                self.won = true;
                return Some(self.all_uncovered() * input as usize);
            }
        }

        // Check columns
        for i in 0..5 {
            let mut all_covered = true;
            for j in 0..5 {
                if self.grid[j][i] != -1 {
                    all_covered = false;
                    break;
                }
            }

            if all_covered {
                self.won = true;
                return Some(self.all_uncovered() * input as usize);
            }
        }

        None
    }
}
