fn farenheiht_to_celsius(farenheiht: f32) -> f32 {
    ((farenheiht - 32.0) * 5.0) / 9.0
}

fn fibonacchi_sequence(length: u32) -> Vec<u32> {
    let vec: Vec<u32> = vec![1, 2];
    return vec;
}

fn main() {
    println!("Variables practise");

    let example = 10.0;
    let converted = farenheiht_to_celsius(example);
    println!(
        "{example} degrees farenheiht is {:.2} degrees celsius",
        converted
    );

    let fibonacchi = fibonacchi_sequence(3)
    println!(fibonacchi);
}
