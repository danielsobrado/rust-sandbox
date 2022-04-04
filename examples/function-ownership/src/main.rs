fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    let i = 5;

    makes_copy(i);

    // println!("{}", s);   It will fail as the function took the ownership
    println!("{}", i);
}

fn takes_ownership(s1: String) {
    println!("{}", s1);
}

fn makes_copy(i1: i32) {
    println!("{}", i1);
}
