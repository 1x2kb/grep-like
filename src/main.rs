use std::io::{BufRead, Error, Write};

fn main() {
    let lines = read_full_stdin("origin/")
        .unwrap()
        .into_iter()
        .reduce(|a, b| a + " " + &b)
        .unwrap();

    println!("{}", lines);
}

fn read_full_stdin(match_sequence: &str) -> Result<Vec<String>, Error> {
    let mut matching_lines = vec![];

    let mut read = 1;

    std::io::stdin()
        .lock()
        .lines()
        .into_iter()
        .for_each(|line_result| match line_result {
            Ok(line) => {
                let line = line.trim();
                if let Some(matched_index) = line.find(match_sequence) {
                    let line = line
                        .get(matched_index + match_sequence.len()..line.len())
                        .unwrap()
                        .into();
                    matching_lines.push(line);
                }
            }
            Err(_) => todo!(),
        });

    Result::Ok(matching_lines)
}
