use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: String,
}

fn main() {
    // Sample student data
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: String::from("300"),
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: String::from("100"),
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: String::from("200"),
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: String::from("200"),
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: String::from("100"),
        },
    ];

    // Display student details
    println!("PAU SMIS");
    println!("{:<4} {:<20} {:<15} {:<10}", "S/N", "Student Name", "Matric. Number", "Department Level");
    for (index, student) in students.iter().enumerate() {
        println!("{:<4} {:<20} {:<15} {:<10}", index + 2, student.name, student.matric_number, format!("{} {}", student.department, student.level));
    }

    // Save student details to a file
    let file_name = "pau_smis.txt";
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };

    // Write student details to the file
    for student in &students {
        if let Err(err) = writeln!(&mut file, "{} {} {} {}", student.name, student.matric_number, student.department, student.level) {
            eprintln!("Error writing to file: {}", err);
            return;
        }
    }

    println!("Student details have been successfully saved to {}", file_name);
}
