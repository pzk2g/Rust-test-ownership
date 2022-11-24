fn main() {
    //println!("Hello, world!");
    //let s1 = String::from("OMG");
    //let s2 = s1.clone();

    //println!("s1 = {}, s2 = {}", s1, s2);

    //let x = 5;
    //let y = x;
    //println!("x = {}, y = {}", x, y);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // error!
    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
    return &s[0..i];
     }
     }
     &s[..]
}
