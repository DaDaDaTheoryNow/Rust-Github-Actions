use std::cmp::Ordering;
use std::io;

fn main() -> std::io::Result<()> {
    loop {
        println!("Choose operation:");
        println!("1. Encrypt");
        println!("2. Decrypt");
        println!("3. Exit");

        let choice = read_str();

        match choice.as_str() {
            "1" => encrypt_process(),
            "2" => decrypt_process(),
            "3" => break,
            _ => println!("Invalid choice, please enter 1, 2 or 3"),
        }
    }

    Ok(())
}

fn encrypt_process() {
    println!("Enter text to encrypt:");
    let input = read_str();

    println!("Enter password:");
    let password = read_str();

    if password.is_empty() {
        println!("Error: Password cannot be empty!");
        return;
    }

    let password_bytes = pad_or_trim(input.len(), password.as_bytes());
    let encrypted = cypher(input.as_bytes(), &password_bytes);

    println!("\nEncryption results:");
    println!("Original text: {}", input);
    println!("Password used: {}", password);
    println!("Encrypted data: {:?}\n", encrypted);
}

fn decrypt_process() {
    println!("Enter encrypted data (as byte string like [1, 2, 3]):");
    let encrypted_input = read_str();

    println!("Enter password:");
    let password = read_str();

    if password.is_empty() {
        println!("Error: Password cannot be empty!");
        return;
    }

    let encrypted_bytes = parse_byte_string(&encrypted_input);
    let password_bytes = pad_or_trim(encrypted_bytes.len(), password.as_bytes());
    let decrypted = decypher(&encrypted_bytes, &password_bytes);

    println!("\nDecryption results:");
    println!("Encrypted data: {:?}", encrypted_bytes);
    println!("Password used: {}", password);
    println!("Decrypted text: {}\n", decrypted);
}

fn parse_byte_string(input: &str) -> Vec<u8> {
    input
        .trim_matches(&['[', ']'][..])
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn cypher(input: &[u8], password: &[u8]) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ password[i % password.len()])
        .collect()
}

fn decypher(encrypted_data: &[u8], password: &[u8]) -> String {
    let decrypted_bytes: Vec<u8> = encrypted_data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ password[i % password.len()])
        .collect();

    String::from_utf8_lossy(&decrypted_bytes).into_owned()
}

fn pad_or_trim(limit: usize, password: &[u8]) -> Vec<u8> {
    let password_len = password.len();
    if password_len == 0 {
        return Vec::new();
    }

    match limit.cmp(&password_len) {
        Ordering::Less => password.iter().take(limit).copied().collect(),
        Ordering::Equal => password.to_vec(),
        Ordering::Greater => {
            let mut result = password.to_vec();
            for i in 0..(limit - password_len) {
                result.push(password[i % password_len]);
            }
            result
        }
    }
}

fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
