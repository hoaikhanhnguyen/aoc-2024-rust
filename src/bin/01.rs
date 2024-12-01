use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();

        let input_file = BufReader::new(File::open(INPUT_FILE)?);
        for line in  input_file.lines(){
            let row = line?;
            let rows: Vec<&str> = row.split_whitespace().collect();
            list1.push(rows[0].parse::<i32>()?);
            list2.push(rows[1].parse::<i32>()?);
        }
        list1.sort_unstable();
        list2.sort_unstable();

        let result: Vec<i32> = list1
            .iter()
            .zip(list2.iter())
            .map(|(a, b)| (a - b).abs())
            .collect();
// part 1 answer
        let sum: i32 = result.iter().sum();
        dbg!(sum);

        // part 2
        // Count occurrences in vec2
        let mut counts = std::collections::HashMap::new();
        for &num in &list2 {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Create a third vector with products
        let result2: Vec<i32> = list1
            .iter()
            .map(|&num| num * counts.get(&num).copied().unwrap_or(0)) // Multiply num by its count in vec2
            .collect();

        let sum2: i32 = result2.iter().sum();
        dbg!(sum2);

        let answer = reader.lines().flatten().count();
        Ok(answer)
    }

    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
