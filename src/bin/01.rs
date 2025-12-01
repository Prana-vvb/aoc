use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;
        let mut pos = 50;

        for line in reader.lines() {
            let line = line?;
            let inp = line.trim();

            let (d, v) = inp.split_at(1);
            let v: i32 = match v.parse() {
                Result::Ok(n) => n,
                Result::Err(e) => panic!("{e}"),
            };

            if d == "L" {
                pos -= v;
            } else if d == "R" {
                pos += v;
            }

            pos = pos.rem_euclid(100);
            if pos == 0 {
                count += 1;
            }
        }

        Ok(count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut count = 0;
        let mut pos = 50;

        for line in reader.lines() {
            let line = line?;
            let inp = line.trim();

            let (d, v) = inp.split_at(1);
            let v: i32 = match v.parse() {
                Result::Ok(n) => n,
                Result::Err(e) => panic!("{e}"),
            };

            let prev = pos;
            if d == "L" {
                pos -= v;
                count += (prev - 1).div_euclid(100) - (pos - 1).div_euclid(100);
            } else if d == "R" {
                pos += v;
                count += pos.div_euclid(100) - prev.div_euclid(100);
            }
        }

        let count: usize = count.try_into()?;
        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
