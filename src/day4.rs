pub fn solution() {
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::One;

    let filename = "inputs/day4.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    let mut input_lines = input.lines();
    // move input range from [0..99] to [1..100], 0 is reserved for marked
    let numbers: Vec<u8> = input_lines.next().unwrap()
        .split(',').map(|s| s.parse().unwrap()).collect();

    type Cell = Option<u8>;
    type Board = [[Cell; 5]; 5];
    let mut boards: Vec<Board> = Default::default();
    {
        let mut board: Board = Default::default();
        loop {
            if input_lines.next().is_none() {
                break;
            }

            for row in 0..5 {
                let mut numbers = input_lines.next().unwrap().split_whitespace();
                for col in 0..5 {
                    let number = numbers.next().unwrap().parse::<u8>().unwrap();
                    board[row][col] = Some(number);
                }
            }

            boards.push(board.clone());
        }
    }

    fn play(boards: &mut Vec<Board>, number: u8)
    {
        for board in boards.iter_mut() {
            for cell in board.iter_mut().flatten() {
                if *cell == Some(number) {
                    *cell = None;
                }
            }
        }
    }

    fn does_win(board: &Board) -> bool
    {
        for row in board {
            if row.iter().all(Option::is_none) {
                return true;
            }
        }
        for col in 0..5 {
            if board.iter().map(|row| row[col]).all(|c| c.is_none()) {
                return true;
            }
        }
        return false;
    }

    let mut number_iter = numbers.iter();
    let (winner_board, winning_number) = loop {
        let &number = number_iter.next().unwrap();

        play(&mut boards, number);

        match part {
            Part::One => {
                if let Some(winner) = boards.iter().find(|b| does_win(b)) {
                    break (winner, number);
                }
            }
            Part::Two => {
                if boards.len() != 1 {
                    boards.retain(|b| !does_win(b));
                } else {
                    if does_win(&boards[0]) {
                        break (&boards[0], number);
                    }
                }
            }
        }
    };

    let mut sum: u32 = 0;
    for cell in winner_board.iter().flatten() {
        sum += cell.unwrap_or(0) as u32;
    }
    println!("Solution is {}", sum * winning_number as u32);
}