fn main() {
    #[allow(dead_code)]
    enum Part {
        One,
        Two,
    }
    let part = Part::Two;

    let input = {
        let filename = "inputs/day7.txt";
        std::fs::read_to_string(filename).expect("Failed to read the file :(")
    };

    let mut positions = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if let Part::One = part {
        let mid = positions.len() / 2;
        positions.select_nth_unstable(mid);

        let x = positions[mid];
        let min_cost = positions.iter().map(|p| x.abs_diff(*p)).sum::<u32>();
        println!("Found optimal x: {x} with minimum cost: {min_cost}");
    }

    if let Part::Two = part {
        let calc_cost = |i: i32| {
            positions.iter().fold(0, |sum, &p| {
                let d = i.abs_diff(p);
                sum + (d * (d + 1)) / 2
            })
        };

        let mut iter = 1;
        let mut lower = *positions.iter().min().unwrap();
        let mut upper = *positions.iter().max().unwrap();

        loop {
            let range = upper - lower;
            let step = range / 3;
            println!("iteration {iter} | searching from {lower} to {upper}");

            let lower_x = lower + 1 * step;
            let upper_x = lower + 2 * step;

            let lower_cost = calc_cost(lower_x);
            let upper_cost = calc_cost(upper_x);

            println!(
                "lower_x: {lower_x:4}, cost: {lower_cost:->0$}",
                (lower_cost / 10_000_000) as usize
            );
            println!(
                "upper_x: {upper_x:4}, cost: {upper_cost:->0$}",
                (upper_cost / 10_000_000) as usize
            );

            let (x, min_cost) = {
                if lower_cost < upper_cost {
                    (lower_x, lower_cost)
                } else {
                    (upper_x, upper_cost)
                }
            };

            iter += 1;
            lower = x - step;
            upper = x + step;

            if step == 1 {
                println!("Found optimal x: {x} with minimum cost: {min_cost}");
                break;
            }
        }
    }
}
