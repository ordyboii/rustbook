use std::io;

pub fn median_tables(mut tables: Vec<u32>) -> Result<u32, String> {
    if tables.is_empty() {
        return Err("Empty vector".to_string());
    }

    tables.sort();
    let len = tables.len();

    if len % 2 == 1 {
        Ok(tables[len / 2])
    } else {
        let mid = (tables[len / 2 - 1]) + (tables[len / 2]);
        Ok(mid / 2)
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
