pub fn solution()
{
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::One;

    let filename = "inputs/day2.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    #[derive(Debug, Default)]
    struct State
    {
        horizontal: i32,
        depth: i32,
        aim: i32,
    }
    let mut state = State::default();

    let strategy = match part
    {
        Part::One => |direction: &str, amount: i32, state: &mut State| match direction
        {
            "forward" => state.horizontal += amount,
            "up" => state.depth -= amount,
            "down" => state.depth += amount,
            _ => ()
        },
        Part::Two => |direction: &str, amount: i32, state: &mut State| match direction
        {
            "forward" => {
                state.horizontal += amount;
                state.depth += state.aim * amount;
            }
            "up" => state.aim -= amount,
            "down" => state.aim += amount,
            _ => ()
        }
    };

    for line in input.lines()
    {
        let (direction, amount) = {
            let parts = line.split_once(' ').unwrap();
            (parts.0, parts.1.parse::<i32>().unwrap())
        };

        strategy(direction, amount, &mut state);
    }

    println!("Final state is {:?}", state);
    println!("Solution is {}", state.horizontal * state.depth);
}