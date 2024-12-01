use std::io::Write;
use std::process::Command;

use crate::YEAR;

pub fn get_input(day: u32) {
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");
    let token = std::env::var("AOC_TOKEN").expect("Requires env varaible AOC_TOKEN");
    let cookie = format!("session={}", token);
    let user_agent = "User-Agent: xtask by oliv.pinon@gmail.com";
    let cmd = Command::new("curl")
        .arg(url)
        .arg("-H")
        .arg(user_agent)
        .arg("--cookie")
        .arg(cookie)
        .output()
        .expect("Failed to get the day's input with curl");

    let filepath = format!("inputs/day{day}.txt");
    let mut file =
        std::fs::File::create(&filepath).expect("Failed to create file to store today's input");
    let _ = file.write(&cmd.stdout).expect("Failed to write output");

    println!("Successfully downloaded {filepath}");
}
