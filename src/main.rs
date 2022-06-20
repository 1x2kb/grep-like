use std::io::{BufRead, Error, Lines, StdinLock, StdoutLock, Write};

enum WriteType {
    Single,
    Mutli,
}

fn main() {
    let matching_sequence = "origin/"; // TODO: read in from env::args.
    let mode_type = WriteType::Single; // TODO: read mode type from env::args.
    let lines = get_lines_from_stdin();
    let matches = scan_buffer_for_matches(lines, matching_sequence).unwrap();

    write_matches_to_output(matches, mode_type).unwrap();
}

fn get_lines_from_stdin() -> Lines<StdinLock<'static>> {
    std::io::stdin().lock().lines()
}

fn scan_buffer_for_matches(
    lines: Lines<StdinLock>,
    match_sequence: &str,
) -> Result<Vec<String>, Error> {
    let matching_lines = lines
        .into_iter()
        .filter_map(|line_result| match line_result {
            Ok(line) => {
                if let Some(matched_index) = line.find(match_sequence) {
                    return Option::Some(
                        line.get(matched_index + match_sequence.len()..line.len())
                            .unwrap()
                            .to_string(),
                    );
                }

                return Option::None;
            }
            Err(_) => Option::None,
        })
        .collect();

    Result::Ok(matching_lines)
}

fn write_matches_to_output(matches: Vec<String>, mode: WriteType) -> Result<(), Error> {
    let mut output = std::io::stdout().lock();

    match mode {
        WriteType::Single => write_single_line(matches, output),
        WriteType::Mutli => write_multi_line(matches, output),
    }
}

fn write_single_line(matches: Vec<String>, mut output: StdoutLock) -> Result<(), Error> {
    let line = matches.into_iter().reduce(|a, b| a + " " + &b).unwrap();

    output.write(line.as_bytes()).unwrap();
    output.flush()
}

fn write_multi_line(matches: Vec<String>, mut output: StdoutLock) -> Result<(), Error> {
    let output_copy = &mut output;

    matches.into_iter().for_each(move |matched_line| {
        output_copy.write(matched_line.as_bytes()).unwrap();
    });

    output.flush()
}
