fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = string_length(s1); // Using a tuple to return the ownership

    println!("Tuple: The length of {} is {}", s2, len);

    let len2 = string_length_reference(&s2); // Using a reference to avoid transferring the ownership

    println!("Reference: The length of {} is {}", s2, len2);

    let mut s3 = String::from("Hello");

    let len3 = change_string_length_reference(&mut s3);

    println!("Mutable: The length of {} is {}", s3, len3);

}

fn string_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn string_length_reference(s: &String) -> usize {
    // s.push_str(", world!!"); We cannot change a borrowed variable
    s.len()
}

// You can have only one mutable reference to a variable
fn change_string_length_reference(s: &mut String) -> usize {
    s.push_str(", world!!"); // We can change a borrowed variable
    s.len()
}
