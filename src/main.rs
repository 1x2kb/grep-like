use std::io::{Error};



fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("Printing out environment variables");
    // args.into_iter().for_each(|variable: String| println!("{}", variable));

    

    let lines = read_full_stdin().unwrap();

    println!("{:?}", lines);
    
}

fn read_full_stdin() -> Result<Vec<String>, Error> {
    let mut matching_lines = vec![];
    
    let mut read = 1;

    while read != 0 {
        let mut line = String::new();
        read = std::io::stdin().read_line(&mut line)?;

        if line.contains("origin/") {
            let line = line.replace("origin/", "");
            matching_lines.push(line);
        }
    }

    Result::Ok(matching_lines)
}