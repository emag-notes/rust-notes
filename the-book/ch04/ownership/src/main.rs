fn main() {
    let s1 = String::from("hello");
    //    let s2 = s1;
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);

    let r1 = &mut s;
    //    let r2 = &mut s; compilation error

    let mut s = String::from("hello world");
    let word = first_word(&s[..]);
    //    s.clear(); compilation error!
    println!("{}'s first word is: {}", s, word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Dangling pointer! compilation error
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
