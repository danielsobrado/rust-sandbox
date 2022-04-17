fn main() {
    let row: Vec<String> = vec![String::from("abc"), 
                                    String::from("abcd"),
                                    String::from("abbcc")];

    // Check if every String starts with ´a´ 
    let result: bool = row.into_iter()  
        .map(|it| { it.chars().nth(0).unwrap() == 'a' })
        .reduce(|acc, mk | acc && mk).unwrap();

    println!("{}", result);

    // The same using fold
    let row2: Vec<String> = vec![String::from("abc"), 
        String::from("abcd"),
        String::from("abbcc")];

    let result2 = row2
    .into_iter()
        .map(|it| { it.chars().nth(0).unwrap() == 'a' })
        .fold(true, |acc, mk| acc && mk);

    println!("{}", result2);

}
