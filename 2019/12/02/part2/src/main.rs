#[derive(Debug)]
struct Machine {
    memory: Vec<usize>,
    ptr: usize,
}

impl Machine {
    fn new(input: Vec<usize>) -> Self {
        Machine {
            memory: input,
            ptr: 0usize,
        }
    }

    fn run(&mut self, noun: usize, verb: usize) -> usize {
        self.memory[1usize] = noun;
        self.memory[2usize] = verb;
        while self.memory.len() > self.ptr {
            match self.read() {
                99 => {
                    break;
                }
                1 => {
                    self.sum();
                }
                2 => {
                    self.mul();
                }
                _ => {
                    println!("{:?}", self);
                    panic!("Wtf?");
                }
            }
        }
        *self.memory.first().unwrap()
    }

    fn read(&mut self) -> usize {
        self.ptr += 1;
        self.memory[self.ptr - 1]
    }

    fn sum(&mut self) {
        let left = self.read();
        let right = self.read();
        let dest = self.read();
        self.memory[dest] = self.memory[left] + self.memory[right];
    }

    fn mul(&mut self) {
        let left = self.read();
        let right = self.read();
        let dest = self.read();
        self.memory[dest] = self.memory[left] * self.memory[right];
    }
}

fn main() {
    let input = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 5, 19, 23, 1, 23, 5, 27,
        1, 27, 13, 31, 1, 31, 5, 35, 1, 9, 35, 39, 2, 13, 39, 43, 1, 43, 10, 47, 1, 47, 13, 51, 2,
        10, 51, 55, 1, 55, 5, 59, 1, 59, 5, 63, 1, 63, 13, 67, 1, 13, 67, 71, 1, 71, 10, 75, 1, 6,
        75, 79, 1, 6, 79, 83, 2, 10, 83, 87, 1, 87, 5, 91, 1, 5, 91, 95, 2, 95, 10, 99, 1, 9, 99,
        103, 1, 103, 13, 107, 2, 10, 107, 111, 2, 13, 111, 115, 1, 6, 115, 119, 1, 119, 10, 123, 2,
        9, 123, 127, 2, 127, 9, 131, 1, 131, 10, 135, 1, 135, 2, 139, 1, 10, 139, 0, 99, 2, 0, 14,
        0,
    ]
    .to_vec();
    let mut noun = 0;
    let mut verb = 0;
    for inner_noun in 0..=99 {
        for inner_verb in 0..=99 {
            match Machine::new(input.clone()).run(inner_noun, inner_verb) {
                19690720 => {
                    noun = inner_noun;
                    verb = inner_verb;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    }
    println!("{}", 100 * noun + verb);
}
