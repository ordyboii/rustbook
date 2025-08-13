fn main() {
    println!("Variables practise\n");

    println!("------Farenheiht to Celsius conversion------\n");

    let example = 10.0;
    let converted = farenheiht_to_celsius(example);
    println!(
        "{:.1} degrees farenheiht is {:.2} degrees celsius",
        example, converted
    );

    println!("------Fibonacci sequence------\n");

    let fibonacci = fibonacci_sequence(20, true);
    match fibonacci {
        FibonacciSequence::CommaSeparated(seq) => println!("{seq}"),
        FibonacciSequence::U32Vec(seq) => println!("{:?}", seq),
    }

    println!("------Twelve days of Christmas------\n");

    let lyrics = twelve_days_of_christmas();
    println!("{lyrics}");
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

fn twelve_days_of_christmas() -> String {
    let mut paras: Vec<String> = vec![];

    for line in 0..12 {
        let mut lines: Vec<String> = vec![];

        let day = match line {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            _ => "twelfth",
        };

        lines.push(format!(
            "On the {day} day of Christmas, my true love sent to me"
        ));

        for gift in (0..=line).rev() {
            let gift = match gift {
                0 => "a partridge in a pear tree.",
                1 => "Two turtle doves,",
                2 => "Three French hens,",
                3 => "Four calling birds,",
                4 => "Five golden rings,",
                5 => "Six geese a-laying,",
                6 => "Seven swans a-swimming,",
                7 => "Eight maids a-milking,",
                8 => "Nine ladies dancing,",
                9 => "Ten lords a-leaping,",
                10 => "Eleven pipers piping,",
                _ => "Twelve drummers drumming,",
            };

            if line != 0 {
                lines.push(format!("And {gift}"));
            } else {
                lines.push(gift.to_string());
            }
        }

        paras.push(lines.join("\n"));
    }

    return paras.join("\n\n");
}
