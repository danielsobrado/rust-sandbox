    fn main() {
        let s1 = gives_ownership();

        let s2 = String::from("Hello");

        let s3 = takes_gives_ownership(s2); // Transfer the ownership from s2 to s3

        println!("{}", s1);   
        // println!("{}", s2); It will fail as the function took the ownership
        println!("{}", s3);

        let (s4, len) = string_length(s3); // Using a tuple to return the ownership

        println!("The length of {} is {}", s4, len)
    }

    fn gives_ownership() -> String {
        let s = String::from("Hello!");
        s
    }

    fn takes_gives_ownership(s: String) -> String {
        s
    }

    fn string_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }