pub fn add_to_waitlist() {
    println!("Adding to waitlist");
}

pub fn add_guests(guestlist: String) -> Vec<String> {
    return guestlist
        .split(',')
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
}

#[derive(Debug)]
pub struct Guest {
    pub name: String,
    pub age: i32,
}

impl Guest {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}
