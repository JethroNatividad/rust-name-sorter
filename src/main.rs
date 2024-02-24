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
}
