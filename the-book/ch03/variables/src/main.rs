fn main() {
    //    let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let heart_eyed_cat = '😻';
    println!("The heart eyed cat: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: ({}, {}, {})", tup.0, tup.1, tup.2);
}
