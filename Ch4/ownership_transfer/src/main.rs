fn main() {

    let _s1 = gives_ownership();

    let _s2 = String::from("hello");

    let _s3 = takes_and_gives_back(_s2); 

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("'{}' length = {}", s2, len);


    let s1 = String::from("hello");

    let len = calculate_length_ref(&s1);

    println!("'{}' length = {}", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{} is s", s);

    /* error due to two mut ownership
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */

    {
        let mut s = String::from("hello"); 

        { 
            let r1 = &mut s;
            r1.push_str(", world");
        }

        let r2 = &mut s;
        r2.push_str(", another world");
        println!("{}", r2);
    }

    /* error since we borrow mutably... 
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}, {}, and {}", r1, r2, r3);
    */


}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}


fn takes_and_gives_back (a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}


// error due to access write ownership even though it only has read ownership
/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


/* error case - dangling pointer 

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

*/


