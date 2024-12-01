use dotenv::dotenv;

mod get;

pub const YEAR: u32 = 2024;

fn main() {
    dotenv().expect("Failed to load .env");
    let mut args = std::env::args();
    let task = args.nth(1);
    match task.as_deref() {
        Some("get") => {
            let day = args.next().expect("Usage: cargo xt get <day>");
            let day = day.parse().expect("day must be a positive number");
            get::get_input(day)
        }
        Some(_) | None => {
            println!("Available commands:");
            println!("get <day(u32)>       Gets the input for the provided day");
        }
    }
}
