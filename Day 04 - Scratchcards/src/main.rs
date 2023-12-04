use std::{collections::HashSet, fs::read_to_string};

fn part_1() {
    let file_lines = read_to_string("input.txt").expect("Could not read file");
    let mut total_sum = 0u32;

    for file_line in file_lines.lines() {
        let split = file_line
            .find("|")
            .expect("Could not find pattern delimiter");

        let winning_numbers = file_line[0..split]
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Could not parse value"))
            .collect::<HashSet<i32>>();

        let mut line_sum = None;

        file_line[(split + 1)..]
            .split_ascii_whitespace()
            .for_each(|value| {
                let value = value.parse::<i32>().expect("Could not parse value");
                if winning_numbers.contains(&value) {
                    line_sum = match line_sum.take() {
                        Some(line_sum) => Some(line_sum * 2),
                        None => Some(1),
                    }
                }
            });

        total_sum += line_sum.unwrap_or(0);
    }

    println!("Part 1: {total_sum}");
}

fn part_2() {
    let file_lines = read_to_string("input.txt").expect("Could not read file");
    let num_of_cards = file_lines.lines().count();
    let mut instances = vec![1u32; num_of_cards];

    for (card_idx, file_line) in file_lines.lines().enumerate() {
        let split = file_line
            .find("|")
            .expect("Could not find pattern delimiter");

        let winning_numbers = file_line[0..split]
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Could not parse value"))
            .collect::<HashSet<i32>>();

        let number_of_matches: usize = file_line[(split + 1)..]
            .split_ascii_whitespace()
            .map(|value| {
                let value = value.parse::<i32>().expect("Could not parse value");
                if winning_numbers.contains(&value) {
                    1
                } else {
                    0
                }
            })
            .sum();

        for offset in 1..=number_of_matches {
            let new_idx = card_idx + offset;
            instances[new_idx] += instances[card_idx];
        }
    }

    println!("Part 2: {}", instances.iter().sum::<u32>());
}

fn main() {
    part_1();
    part_2();
}