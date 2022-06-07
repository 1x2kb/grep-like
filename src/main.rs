use std::io::Error;

fn main() {
    let lines = read_full_stdin()
        .unwrap()
        .into_iter()
        .reduce(|a, b| a + " " + &b)
        .unwrap();

    println!("{}", lines);
}

fn read_full_stdin() -> Result<Vec<String>, Error> {
    let mut matching_lines = vec![];

    let mut read = 1;

    let match_sequence = "origin/";

    while read != 0 {
        let mut line = String::new();
        read = std::io::stdin().read_line(&mut line)?;
        let line = line.trim();

        if let Some(result) = line.find(match_sequence) {
            println!("Found origin/ at location: {}", result);

            let line: String = line
                .get(result + match_sequence.len()..line.len())
                .unwrap()
                .into();
            println!("Line without text: {}", line);
            matching_lines.push(line);
        }
    }

    Result::Ok(matching_lines)
}
