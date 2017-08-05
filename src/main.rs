use std::env;

fn kolakoski(n: usize) -> Vec<u32> {
    let mut sequence = Vec::new();

    // This keeps track of what element in the sequence we look
    // at to determine the length of the next run.
    let mut length_index : usize = 1;

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

        // Push as much of the current number to fill this run.
        for _ in 0..expected_len {
            sequence.push(last_num);
        }

        length_index += 1;

        // Next run must use the other number.
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
