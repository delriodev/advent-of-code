use regex::Regex;
use std::{
    fs::{self, DirEntry},
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let day_entries = fs::read_dir(".")?
        .filter(|entry| match entry {
            Ok(dir_entry) => match dir_entry.file_name().into_string() {
                Ok(name) => name.contains("day"),
                Err(..) => false,
            },
            Err(..) => false,
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    match &day_entries.len() {
        0 => println!("Error: No dayx.rs file found"),
        1 => update_day(day_entries.first().unwrap()),
        _ => println!("Error: Multiple dayx.rs files found"),
    }

    Ok(())
}

fn update_day(dir_entry: &DirEntry) {
    let path = dir_entry.path();
    let current_file = dir_entry.file_name().into_string().unwrap();
    let completed_dir = format!("./completed/");
    let target_path = format!("{completed_dir}{current_file}");

    // move the day file into the completed folder
    match fs::rename(&path, &target_path) {
        Ok(()) => println!("Moved {} to {}", current_file, &completed_dir),
        Err(..) => println!("Failed to move {} to {}", current_file, &completed_dir),
    }

    let current_number = current_file
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();
    let next_number = current_number.parse::<u8>().unwrap() + 1;
    let next_day = format!("day{next_number}");
    let next_file = format!("{next_day}.rs");
    let template_path = "/home/ddr/Repos/advent-of-code/day-manager/advent_of_code_template.rs";

    // create a new day + 1 file and write the template to it
    let new_file = fs::File::create(&next_file);
    match new_file {
        Ok(mut f) => match transfer(template_path, &mut f) {
            Ok(()) => println!("Created file {}", &next_file),
            Err(..) => println!("Failed to write template to file {}", &next_file),
        },
        Err(..) => println!("Failed to create file {}", &next_file),
    }

    // Update main.rs file
    let main_path = "./main.rs";

    match fs::read_to_string(main_path) {
        Ok(content) => {
            let day_re = Regex::new(r"day\d+").unwrap();
            let sol_re = Regex::new(r"usize = \d+").unwrap();

            let m1 = day_re.replace_all(&content.as_str(), &next_day);
            let m2 = sol_re.replace_all(&m1, "usize = 0");

            match fs::write(main_path, m2.as_bytes()) {
                Ok(()) => println!("Updated main.rs"),
                Err(..) => println!("Failed to update main.rs file"),
            }
        }
        Err(..) => println!("Could not read {}", main_path),
    }
}

fn transfer(from: &str, to: &mut fs::File) -> io::Result<()> {
    to.write_all(&fs::read(from)?)?;
    Ok(())
}
