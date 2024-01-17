use std::fs::File;
use std::io::{self, Write};

fn main() {
    // List of high-quality drink categories
    let drink_categories = vec![
        "Lager",
        "Stout",
        "Non-Alcoholic",
        "Legend",
        "Desperados",
        "Goldberg",
        "Turbo King Williams",
        "Maltina",
        "Amstel Malta",
        "Gulder",
        "Malta Gold Fayrouz",
        "Heineken",
        "Star",
    ];

    // Specify the file name to save the categories
    let file_name = "drink_categories.txt";

    // Attempt to create or open the file for writing
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };

    // Write the drink categories to the file
    for category in &drink_categories {
        if let Err(err) = writeln!(&mut file, "{}", category) {
            eprintln!("Error writing to file: {}", err);
            return;
        }
    }

    println!("Drink categories have been successfully saved to {}", file_name);
}
