use std::io;

pub fn median_tables(mut tables: Vec<u32>) -> Option<u32> {
    if tables.is_empty() {
        return None;
    }

    tables.sort_by(|a, b| b.cmp(a));

    match tables.get(tables.len() / 2) {
        Some(n) => Some(*n),
        None => None,
    }
}

pub fn read_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
