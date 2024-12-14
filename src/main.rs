use std::{
    fs::{File, OpenOptions},
    io::Write,
    process::{self, Stdio},
};

const DAY_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/day_template.txt"
));

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo <command> <day>");
        process::exit(1);
    }

    let day: u32 = args[2].parse::<u32>().unwrap_or(0);
    if day == 0 || day > 25 {
        eprintln!("Invalid day");
        process::exit(1);
    }

    let command = args[1].as_str();
    match command {
        "scaffold" => scaffold(day),
        "solve" => solve(day),
        _ => {
            eprintln!("Invalid command");
            process::exit(1);
        }
    }
}

fn scaffold(day: u32) {
    let input_file_path = format!("inputs/day_{:02}.txt", day);
    match create_file(&input_file_path) {
        Ok(_) => println!("Created file: {}", input_file_path),
        Err(error) => {
            eprintln!("Error creating file: {}", error);
            process::exit(1);
        }
    };

    let src_file_path = format!("src/bin/day_{:02}.rs", day);
    let mut src_file = match create_file(&src_file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error creating file: {}", error);
            process::exit(1);
        }
    };

    match src_file.write_all(
        DAY_TEMPLATE
            .replace("%DAY_NUMBER%", &day.to_string())
            .as_bytes(),
    ) {
        Ok(_) => println!("Created file: {}", src_file_path),
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }
}

fn solve(day: u32) {
    let cmd_args = vec![
        "run".to_string(),
        "--quiet".to_string(),
        "--bin".to_string(),
        format!("day_{:02}", day).to_string(),
        "--".to_string(),
        format!("inputs/day_{:02}.txt", day).to_string(),
    ];

    let mut cmd = process::Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    return OpenOptions::new().create(true).write(true).open(path);
}
