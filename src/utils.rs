use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (i, line) in read_lines(file)
        .expect(format!("couldn't open file {}", file).as_str())
        .enumerate()
    {
        result.push(format!(
            "{}\n",
            line.expect(format!("couldn't open line {}", i).as_str())
        ));
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
