use std::io;

fn main() {
    println!("Enter a year");

    loop {
        let mut provided_year = String::new();

        io::stdin().read_line(&mut provided_year)
                .expect("Failed to read line");

        let provided_year: u32 = match provided_year.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
        };

        if leap_year_check(provided_year) {
            println!("This is a leap year!");
        } else {
            println!("This is not a leap year!");
        }
        break;
    }
    
}

fn leap_year_check(year: u32) -> bool {
    year % 4 == 0 
} 