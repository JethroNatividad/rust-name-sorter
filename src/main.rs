use std::fs::File;
use std::io::{BufWriter, Write};
// Program that has a list of names, sorts them, and writes them to a file
// Input: names
// Process: sort
// Output: in file separated by newlines
// names:
//     Ling, Mai
//     Johnson, Jim
//     Zarnecki, Sabrina
//     Jones, Chris
//     Jones, Aaron
//     Swift, Geoffrey
//     Xiong, Fong
fn write_vector_to_file(vec: Vec<String>, file_path: &str) -> std::io::Result<()> {
    // Open the file in write mode
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    // Iterate over the vector and write each string to the file
    for line in vec {
        // Write the string followed by a newline character
        writeln!(writer, "{}", line)?;
    }

    // No need to manually flush the writer, as it will be flushed automatically when dropped
    Ok(())
}

fn main() {
    // Initialize a vector of names
    let mut names: Vec<String> = vec![
        "Ling, Mai".to_string(),
        "Johnson, Jim".to_string(),
        "Zarnecki, Sabrina".to_string(),
        "Jones, Chris".to_string(),
        "Jones, Aaron".to_string(),
        "Swift, Geoffrey".to_string(),
        "Xiong, Fong".to_string(),
    ];
    // Sort alphabetically
    names.sort_by(|a, b| a.cmp(&b));
    // write each name to a file
    println!("{:?}", names);
    match write_vector_to_file(names, "sorter.txt") {
        Ok(_) => println!("Successfully wrote to file."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}
