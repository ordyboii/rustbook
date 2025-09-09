use crate::front_of_house::hosting::{self, Guest};

#[test]
fn it_works() {
    hosting::add_to_waitlist();
}

#[test]
fn adding_guests() {
    let guestlist = String::from("Adam, Jones,  Jake  ");
    let guests = hosting::add_guests(guestlist);

    let first_guest = guests.get(1);

    match first_guest {
        Some(first) => println!("First guest is {}", first),
        None => println!("No first guest found"),
    }

    let new_guests = vec![
        Guest::new(String::from("Jones"), 28),
        Guest::new(String::from("Alux"), 29),
    ];

    for guest in new_guests {
        println!(
            "New guest's name {}",
            format!("{} hello you are {}", guest.name, guest.age)
        );
    }

    assert_eq!(
        guests,
        vec![
            String::from("Adam"),
            String::from("Jones"),
            String::from("Jake")
        ]
    );

    let greeting = "Здравствуйте";
    let last_char = greeting.chars().last();
    assert_eq!(last_char, Some('е'));
}
