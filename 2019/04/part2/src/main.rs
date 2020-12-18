struct Combination {
    current: String,
    last: String,
}

impl Iterator for Combination {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.current == self.last {
            return None;
        };

        self.current = format!(
            "{:0>6}",
            self.current.parse::<u32>().expect("Cannot parse as u32") + 1
        );
        Some(self.current.to_owned())
    }
}

fn main() {
    let count = Combination {
        current: "382345".to_string(),
        last: "843167".to_string(),
    }
    .filter(|x| {
        let mut clustered = Vec::new();
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

        // Check clustered array has at least one cluster with exaclty 2 numbers in it
        clustered.iter().any(|x| x.len() == 2)
    })
    .count();
    println!("{}", count);
}
