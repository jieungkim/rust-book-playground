fn main() {

    let mut s = String::from("hello world!");

    let word = first_word(&s);
    println!("{}", word);

    s.clear();


    /* Some examples 
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];


    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    */

    let s = String::from("hello word!");

    // let mut s = String::from("hello world!");

    let word = first_word_second(&s);

    // s.clear() -> error due to barrow  -> even tough we define it as mut
    println!("the first word is : {}", word);


    let my_string = String::from("hello word");

    let _word = first_word_third(&my_string[..]);
    let my_string_literal = "hello word"; 

    let _word = first_word_third(&my_string_literal[..]);
    // string literal is already a string slice, so we can call any of those following things
    let _word = first_word_third(my_string_literal);
}



fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_word_second(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_third(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
