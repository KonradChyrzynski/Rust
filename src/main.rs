fn number_of_variations_with_repetition(n: u32, m: u32) -> u32 {
    return u32::pow(n,m);
}

fn main() {
    let n2: u32 = 12;
    let m2: u32 = 2;
    println!("Number: {}", number_of_variations_with_repetition(n2,m2));
}

