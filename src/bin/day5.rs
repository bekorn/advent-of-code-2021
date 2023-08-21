use std::cmp::max;

#[derive(Debug, Copy, Clone, PartialEq)]
struct u32x2(u32, u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct i8x2(i8, i8);

#[derive(Debug)]
struct Line {
    begin: u32x2,
    end: u32x2,
}

struct LineIterator {
    current: u32x2,
    direction: i8x2,
    end: u32x2,
}

impl IntoIterator for &Line {
    type Item = u32x2;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        let direction = i8x2(
            self.end.0.cmp(&self.begin.0) as i8,
            self.end.1.cmp(&self.begin.1) as i8,
        );
        LineIterator {
            current: self.begin,
            direction,
            end: u32x2(
                self.end.0.wrapping_add(direction.0 as u32),
                self.end.1.wrapping_add(direction.1 as u32),
            ),
        }
    }
}

impl Iterator for LineIterator {
    type Item = u32x2;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.current;

        self.current.0 = self.current.0.wrapping_add(self.direction.0 as u32);
        self.current.1 = self.current.1.wrapping_add(self.direction.1 as u32);

        if point == self.end { return None; } else { Some(point) }
    }
}

#[test]
fn line_iterator_test() {
    let line = Line {
        begin: u32x2(5, 3),
        end: u32x2(8, 0),
    };

    for point in &line {
        println!("{:?}", point);
    }
}

fn main() {
    #[allow(dead_code)]
    enum Part { One, Two }
    let part = Part::Two;

    let filename = "inputs/day5.txt";
    let input = std::fs::read_to_string(filename).expect("Failed to read the file :(");

    let lines = {
        let mut lines: Vec<Line> = input.lines().map(|l| {
            let (begin, end) = l.split_once(" -> ").unwrap();
            let (begin_x, begin_y) = begin.split_once(',').unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            Line {
                begin: u32x2(begin_x.parse().unwrap(), begin_y.parse().unwrap()),
                end: u32x2(end_x.parse().unwrap(), end_y.parse().unwrap()),
            }
        }).collect();

        if let Part::One = part {
            lines.retain(|l| l.begin.0 == l.end.0 || l.begin.1 == l.end.1);
        }

        lines
    };

    let mut ground = {
        let range = lines.iter().fold(u32x2(0, 0), |r, l| u32x2(
            max(r.0, max(l.begin.0, l.end.0)),
            max(r.1, max(l.begin.1, l.end.1)),
        ));
        vec![vec![0u16; (range.1 + 1) as usize]; (range.0 + 1) as usize]
    };

    for line in &lines {
        for point in line {
            ground[point.0 as usize][point.1 as usize] += 1;
        }
    }

    let total = ground.iter().flatten().filter(|&&p| p > 1).count();
    print!("Total overlapping points {:?}", total);
}