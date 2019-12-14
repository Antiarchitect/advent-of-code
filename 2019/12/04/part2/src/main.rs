fn main() {
    let lower: u32 = 382345;
    let upper: u32 = 843167;

    let count = (lower..=upper)
        .filter(|&x| {
            let mut clustered: Vec<Vec<_>> = Vec::new();
            for c in x.to_string().chars() {
                let digit = c.to_digit(10).expect("Cannot parse digit");
                match clustered.last_mut() {
                    None => {
                        clustered.push(vec![digit]);
                    }
                    Some(last) => {
                        let last_digit = *last.last().unwrap();
                        if last_digit > digit {
                            return false;
                        };
                        if last_digit == digit {
                            last.push(digit);
                        } else {
                            clustered.push(vec![digit]);
                        }
                    }
                }
            }

            // Remove combinations having no doubles at all
            if clustered.len() == 6 {
                return false;
            };

            // Check clustered array has only proper possible combinations (even doubles and single digits)
            let proper = clustered.iter().all(|x| x.len() == 1 || x.len() % 2 == 0);
            if !proper {
                return false;
            }
            println!("{:?}", clustered);
            true
        })
        .count();
    println!("{}", count);
}
