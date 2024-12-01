#[macro_export]
macro_rules! aoc {
    ($($call:ident),*) => {
        fn main() {
            use std::io::prelude::*;
            let bin_name = env!("CARGO_BIN_NAME");
            let file_input_name = format!("inputs/{bin_name}.txt");
            let mut file_input = std::fs::File::open(file_input_name).expect("Failed to open file");
            let mut input = String::new();
            file_input.read_to_string(&mut input).expect("Failed to read file");
            $(
                let before = std::time::Instant::now();
                let result = $call(&input);
                let after = std::time::Instant::now();
                let delta = after - before;
                println!("{}:", stringify!($call));
                println!("{result}");
                println!("---");
                println!("{delta:?}");
                println!("");
            )*
        }
    };
}
