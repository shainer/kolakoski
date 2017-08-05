use std::env;

fn kolakoski(n: usize) -> Vec<u32> {
    let mut sequence = Vec::new();

    // This keeps track of what element in the sequence we look
    // at to determine the length of the next run.
    let mut length_index : usize = 1;

    // Keeps track of the length of the current run. Only really
    // used at the beginning since we start in the middle of a run.
    let mut current_length = 1;

    // Keeps track of the number for the last run.
    let mut last_num = 2;

    // Inits the sequence.
    sequence.push(1);

    // For n=1, we skip this and simply return at the end.
    if n >= 2 {
        sequence.push(2);
    }

    while sequence.len() < n {
        let expected_len = sequence[length_index];

        for _ in 0..(expected_len - current_length) {
            sequence.push(last_num);
        }
        current_length = 0;

        length_index += 1;

        match last_num {
            1 => last_num = 2,
            _ => last_num = 1
        }
    }

    sequence
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Wrong number of arguments.");
    }

    let n : usize = args[1].parse::<usize>().unwrap();

    if n <= 0 {
        panic!("N must be strictly positive.");
    }

    let seq = kolakoski(n);
    println!("{:?}", seq);
}
