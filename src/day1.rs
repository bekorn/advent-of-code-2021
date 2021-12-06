pub fn solution()
{
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::One;

    let filename = "inputs/day1.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    let depths: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();

    let windowed_depths: Vec<u32> = match part {
        Part::One => depths,
        Part::Two => depths.windows(3).map(|window| window.iter().sum()).collect()
    };

    let increments = windowed_depths.windows(2).filter(|d| d[1] > d[0]).count();
    print!("Total number of increments: {}", increments);
}