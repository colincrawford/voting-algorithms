mod boyer_moore;

use std::fs;

use crate::boyer_moore::boyer_moore;

const VOTES_INPUT_FILE: &str = "votes.txt";

fn main() {
    println!(
        "Running Boyer-Moore voting algorithm on votes in {}",
        VOTES_INPUT_FILE
    );

    let votes_file_contents: String = fs::read_to_string(VOTES_INPUT_FILE)
        .expect("The votes input file should exist at votes.txt");

    let votes: Vec<&str> = votes_file_contents.lines().collect();

    println!("Votes: {}", votes.join(" "));

    let result = boyer_moore(votes);

    println!("Vote Winner: {}", result.unwrap_or("No Winner"));
}
