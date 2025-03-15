use std::io::stdin;

// Performs the main decryption steps on the input string. Returns a string.
pub fn skyler_cypher_decrypt(input_string: &str) -> String {
    println!("Enter the cyphertext to be decrypted.");

    let mut cypher_vector: Vec<String> = input_string
        .split_whitespace()
        .map(String::from)
        .collect();

    cypher_vector = reverse_vec(cypher_vector);

    let cypher_vector: Vec<u8> = convert_to_decimal(cypher_vector);

    let cypher_vector: Vec<u8> = salt_removal(cypher_vector);

    let cypher_vector: Vec<String> = reverse_ascii_values(cypher_vector);

    let output_string: String = cypher_vector
        .iter()
        .map(|n| n.clone()) // Directly clone the String
        .collect::<Vec<String>>()
        .join(" "); // Join them with spaces

    return output_string;
}


// Reverses a vector of strings.
pub fn reverse_vec(input_vector: Vec<String>) -> Vec<String> {
    let input_vector: Vec<String> = input_vector
        .into_iter().rev().collect();

    return input_vector;
}

// Converts from hexadecimal to decimal.
pub fn convert_to_decimal(input_vector: Vec<String>) -> Vec<u8> {
    input_vector
        .into_iter()
        .map(|element| u8::from_str_radix(&element, 16).expect("Failed to parse to integer"))
        .collect()
}

// Removes the salt from the input vector.
pub fn salt_removal(mut input_vector: Vec<u8>) -> Vec<u8> {
    println!("Input the salt used in encryption.");
    let mut salt_string = String::new();

    stdin()
        .read_line(&mut salt_string)
        .expect("Failed to read line");

    let salt_string = salt_string.trim();

    let string_length: usize = salt_string
        .chars()
        .count();

    let middle: usize = input_vector.len() / 2;

    let num_to_remove: usize = string_length + middle;

    if num_to_remove <= input_vector.len() {
    input_vector.drain(middle..num_to_remove);
}

    return input_vector;
}

pub fn reverse_ascii_values(mut input_vector: Vec<u8>) -> Vec<String> {
    let mut output_vector = Vec::new();

    for &element in &input_vector {
        let encoded = element as u32;
        let mut candidates = Vec::new();

        for k in 0..10 { // Limit the search range
            let candidate_temp = encoded - 32 + 95 * k;
            let reversed_str = candidate_temp.to_string().chars().rev().collect::<String>();

            if let Ok(original) = reversed_str.parse::<u8>() {
                if (32..=126).contains(&original) { // Ensure valid ASCII
                    candidates.push(char::from(original));
                }
            }
        }

        // Format as [i/>] or just the character if it's unique
        if candidates.len() == 1 {
            output_vector.push(candidates[0].to_string());
        } else {
            let formatted = format!("[{}]", candidates.iter().collect::<String>().replace("", "/").trim_matches('/'));
            output_vector.push(formatted);
        }
    }

    output_vector
}


fn main() {
    println!("Enter the string to be decrypted.");
    let mut input_cypher = String::new();

    stdin()
        .read_line(&mut input_cypher)
        .expect("Failed to read line");

    let output: String = skyler_cypher_decrypt(input_cypher.trim()); // Fix: Trim to remove newline
    
    println!("Decoded string:");
    println!("{output}");
}

