fn main() {
    let lower: u32 = 382345;
    let upper: u32 = 843167;

    let count = (lower..=upper)
        .filter(|&x| {
            let mut current = 0;
            let mut has_successives = false;
            for c in x.to_string().chars() {
                let digit = c.to_digit(10).expect("Cannot parse digit");
                if digit < current {
                    return false;
                } else {
                    if current == digit {
                        has_successives = true;
                    }
                    current = digit;
                }
            }
            has_successives
        })
        .count();
    println!("{}", count);
}
