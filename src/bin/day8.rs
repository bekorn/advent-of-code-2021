fn main() {
    enum Part {
        One,
        Two,
    }
    let part = Part::Two;

    let input = {
        let filename = "inputs/day8.txt";
        std::fs::read_to_string(filename).expect("Failed to read the file :(")
    };

    // let input =
    //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    if let Part::One = part {
        let mut digits1478 = 0u32;

        for line in input.lines() {
            let (_raw_digits, raw_outputs) = line.split_once(" | ").unwrap();

            for raw_output in raw_outputs.split(' ') {
                let digit_count = raw_output.len();
                if digit_count == 2 || digit_count == 4 || digit_count == 3 || digit_count == 7 {
                    digits1478 += 1;
                }
            }
        }

        println!("Number of digits 1, 4, 7, or 8 in outputs is {digits1478}");
    }

    if let Part::Two = part {
        type Display = [bool; 7];
        type Signal = [Display; 10];
        type Mapping = [u8; 7];

        /*
          0*      1       2       3       4
         aaaa    ....    aaaa    aaaa    ....
        b    c  .    c  .    c  .    c  b    c
        b    c  .    c  .    c  .    c  b    c
         ....    ....    dddd    dddd    dddd
        e    f  .    f  e    .  .    f  .    f
        e    f  .    f  e    .  .    f  .    f
         gggg    ....    gggg    gggg    ....

          5       6       7       8       9
         aaaa    aaaa    aaaa    aaaa    aaaa
        b    .  b    .  .    c  b    c  b    c
        b    .  b    .  .    c  b    c  b    c
         dddd    dddd    ....    dddd    dddd
        .    f  e    f  .    f  e    f  .    f
        .    f  e    f  .    f  e    f  .    f
         gggg    gggg    ....    gggg    gggg

        sum     digits
        2       1
        3       7
        4       4
        5       2, 3, 5
        6       0, 6, 9
        7       8

        segment count
        a       8
        b       6 <-
        c       8
        d       7
        e       4 <-
        f       9 <-
        g       7
        */
        let digits: [Display; 10] = [
            //[  a,     b,     c,     d,     e,     f,     g]
            [true, true, true, false, true, true, true],
            [false, false, true, false, false, true, false],
            [true, false, true, true, true, false, true],
            [true, false, true, true, false, true, true],
            [false, true, true, true, false, true, false],
            [true, true, false, true, false, true, true],
            [true, true, false, true, true, true, true],
            [true, false, true, false, false, true, false],
            [true, true, true, true, true, true, true],
            [true, true, true, true, false, true, true],
        ];

        let mut output_sum = 0;
        for line in input.lines() {
            let (raw_signals, raw_outputs) = line.split_once(" | ").unwrap();

            let mut signal = Signal::default();
            for (i, raw_signal) in raw_signals.split(' ').enumerate() {
                for c in raw_signal.chars() {
                    signal[i][c as usize - b'a' as usize] = true;
                }
            }

            let mut segment_count = [0u8; 7];
            for display in &signal {
                for (i, &d) in display.iter().enumerate() {
                    segment_count[i] += d as u8;
                }
            }
            let b = segment_count.iter().position(|&sc| sc == 6).unwrap();
            let e = segment_count.iter().position(|&sc| sc == 4).unwrap();
            let f = segment_count.iter().position(|&sc| sc == 9).unwrap();

            let mut display_sum = [0u8; 10];
            for (i, display) in signal.iter().enumerate() {
                display_sum[i] = display.iter().filter(|&&b| b).count() as u8;
            }

            let cf = &signal[display_sum.iter().position(|&d| d == 2).unwrap()]; // digit 1
            let acf = &signal[display_sum.iter().position(|&d| d == 3).unwrap()]; // digit 7
            let bcdf = &signal[display_sum.iter().position(|&d| d == 4).unwrap()]; // digit 4

            let a = acf.iter().zip(cf).position(|(&a, &b)| a ^ b).unwrap();
            let c = segment_count
                .iter()
                .enumerate()
                .position(|(i, &sc)| sc == 8 && i != a)
                .unwrap();
            let d = bcdf
                .iter()
                .enumerate()
                .position(|(i, &s)| s && i != b && i != c && i != f)
                .unwrap();
            let g = segment_count
                .iter()
                .enumerate()
                .position(|(i, &sc)| sc == 7 && i != d)
                .unwrap();

            let mut mapping = Mapping::default();
            mapping[a] = 0;
            mapping[b] = 1;
            mapping[c] = 2;
            mapping[d] = 3;
            mapping[e] = 4;
            mapping[f] = 5;
            mapping[g] = 6;

            let mut output = 0;
            for raw_output in raw_outputs.split(' ') {
                let mut display = Display::default();
                for c in raw_output.chars() {
                    display[c as usize - 'a' as usize] = true;
                }

                let mut mapped_display = Display::default();
                for (i, &b) in display.iter().enumerate() {
                    mapped_display[mapping[i] as usize] = b;
                }

                let digit = digits
                    .iter()
                    .position(|&digit| digit == mapped_display)
                    .unwrap();

                output = output * 10 + digit;
            }
            output_sum += output;
        }
        println!("output sum is {output_sum}");
    }
}
