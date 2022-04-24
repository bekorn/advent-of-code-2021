pub fn solution()
{
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::Two;

    let filename = "inputs/day6.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    let mut day = 0usize;
    let mut adults: [u64; 7] = [0; 7];
    let mut youngs: [u64; 9] = [0; 9];

    for days_left in input.split(',') {
        let days_left = days_left.parse::<usize>().unwrap();
        adults[days_left] += 1;
    }

    let day_limit = match part {
        Part::One => { 80 }
        Part::Two => { 256 }
    };

    while day < day_limit {
        let adults_day = day % adults.len();
        let youngs_day = day % youngs.len();

        let newadults = youngs[youngs_day];
        let newborns = newadults + adults[adults_day];
        youngs[youngs_day] = newborns;
        adults[adults_day] += newadults;

        // println!("Day {day:02} Adults {adults:?} Youngs {youngs:?}");
        // println!(
        //     "       +{newborns:_>5} {} +{newadults:_>5} {}",
        //     { let mut a = ["   "; 7]; a[adults_day] = " ^ "; a.join("") },
        //     { let mut y = ["   "; 9]; y[youngs_day] = " ^ "; y.join("") }
        // );

        day += 1;
    }

    println!("Final population is {}", adults.iter().sum::<u64>() + youngs.iter().sum::<u64>());
}