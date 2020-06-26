fn main() {
    let number = 3; 

    if number < 5 {
        println!("matched");
    }
    else {
        println!("not matched");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6 
    };

    println!("{} {}", condition, number);
}

/* JIEUNG: error version due to type mismatches (in condition and types for branches */
/*
fn main(){
    let number = 3; 
    if number {
        println!("mismatch");
    }
}

fn main(){
    let condition = true;
    let number = if condition { 
        5
    } else {
        "six"
    };

    println!("error");
}
*/




