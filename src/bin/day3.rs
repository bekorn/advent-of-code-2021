// copy pasted from https://github.com/llogiq/partition/blob/master/src/lib.rs
// std partition is copy, and partition_in_place is unsafe
fn partition_index<T, P>(data: &mut [T], predicate: P) -> usize
    where P: Fn(&T) -> bool {
    let len = data.len();
    if len == 0 { return 0; }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) { l += 1; }
        while r > 0 && !predicate(&data[r]) { r -= 1; }
        if l >= r { return l; }
        data.swap(l, r);
    }
}

fn main()
{
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::Two;

    let filename = "inputs/day3.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    let mut inputs: Vec<Vec<bool>> = input.lines()
        .map(|s| s.chars().map(|c| c == '1').collect())
        .collect();

    if let Part::One = part {
        let mut bit_counts = vec![0; inputs[0].len()];

        for binary in &inputs {
            for (bit, count) in binary.iter().zip(bit_counts.iter_mut()) {
                *count += *bit as usize;
            }
        }

        let threshold = inputs.len() / 2;

        let mut gamma_str = String::with_capacity(bit_counts.len());
        let mut epsilon_str = String::with_capacity(bit_counts.len());
        for count in bit_counts
        {
            gamma_str.push(if count < threshold { '1' } else { '0' });
            epsilon_str.push(if count < threshold { '0' } else { '1' });
        }
        let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
        let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();

        println!("Gamma = {}, Epsilon = {}, The solution is {}", gamma, epsilon, gamma * epsilon);
    } else {
        let bit_count = inputs[0].len();
        // first partition is manual
        let (mut o2rates, mut co2rates) = {
            let p_i = partition_index(inputs.as_mut_slice(), |b| b[0]);
            let (ones, zeros) = inputs.split_at_mut(p_i);
            if ones.len() > zeros.len() { (ones, zeros) } else { (zeros, ones) }
        };
        // remaining partitions
        for layer in 1..bit_count {
            o2rates = {
                if o2rates.len() == 1 {
                    o2rates
                } else {
                    let p_i = partition_index(o2rates, |b| b[layer]);
                    if p_i == o2rates.len() - p_i { // prefers 1s
                        &mut o2rates[..p_i]
                    } else if p_i > o2rates.len() - p_i { // prefer larger partition
                        &mut o2rates[..p_i]
                    } else {
                        &mut o2rates[p_i..]
                    }
                }
            };
            co2rates = {
                if co2rates.len() == 1 {
                    co2rates
                } else {
                    let p_i = partition_index(co2rates, |b| b[layer]);
                    if p_i == co2rates.len() - p_i { // prefers 0s
                        &mut co2rates[p_i..]
                    } else if p_i > co2rates.len() - p_i { // prefer larger partition
                        &mut co2rates[p_i..]
                    } else {
                        &mut co2rates[..p_i]
                    }
                }
            };
        }

        fn into_decimal(v: &Vec<bool>) -> u32 {
            let mut val = 0;
            let mut digit_val = 1;
            for &digit in v.iter().rev() {
                if digit {
                    val += digit_val;
                }
                digit_val *= 2;
            }
            val
        }

        let o2rate = into_decimal(&o2rates[0]);
        let co2rate = into_decimal(&co2rates[0]);
        println!("O2 Rate: {},  CO2 Rate: {}", o2rate, co2rate);
        println!("Solution: {}", o2rate * co2rate);
    }
}