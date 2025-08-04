fn main() {
    println!("Variables practise");

    let example = 10.0;
    let converted = farenheiht_to_celsius(example);
    println!(
        "{:.1} degrees farenheiht is {:.2} degrees celsius",
        example, converted
    );

    let fibonacci = fibonacci_sequence(20, true);
    match fibonacci {
        FibonacciSequence::CommaSeparated(seq) => println!("{seq}"),
        FibonacciSequence::U32Vec(seq) => println!("{:?}", seq),
    }

    twelve_days_of_christmas();
}

fn farenheiht_to_celsius(farenheiht: f32) -> f32 {
    ((farenheiht - 32.0) * 5.0) / 9.0
}

enum FibonacciSequence {
    U32Vec(Vec<u32>),
    CommaSeparated(String),
}

fn fibonacci_sequence(length: u32, format: bool) -> FibonacciSequence {
    let mut sequence: Vec<u32> = vec![0, 1];
    let mut start = sequence[0];
    let mut end = sequence[1];

    for _ in 2..length {
        let sum = start + end;
        start = end;
        end = sum;

        sequence.push(sum);
    }

    if format {
        return FibonacciSequence::CommaSeparated(
            sequence
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
    }

    return FibonacciSequence::U32Vec(sequence);
}

fn twelve_days_of_christmas() {}

// On the first day of Christmas
// my true love sent to me
// a partridge in a pear tree.
//
// On the second day of Christmas,
// my true love sent to me
// Two turtle doves,
// and a partridge in a pear tree.
//
// On the third day of Christmas,
// my true love sent to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the fourth day of Christmas,
// my true love sent to me
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the fifth day of Christmas,
// my true love sent to me
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the sixth day of Christmas,
// my true love sent to me
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the seventh day of Christmas,
// my true love sent to me
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the eighth day of Christmas,
// my true love sent to me
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the ninth day of Christmas,
// my true love sent to me
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the tenth day of Christmas,
// my true love sent to me
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the eleventh day of Christmas,
// my true love sent to me
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the twelfth day of Christmas,
// my true love sent to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!
