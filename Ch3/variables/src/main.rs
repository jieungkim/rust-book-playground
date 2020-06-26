// const MAX_POINTS: u32 = 100_000;

fn main() {
    /* JIEUNG: error version
    let x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);
    */
    let mut x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces);

    /* JIEUNG: error version
    let mut spaces = "   "; 
    spaces = spaces.len();
    */

    println!("안녕하세요!");
    another_function(5, 6);

    /* JIEUNG error
    let x = (let y = 6)
    */

    let x = 5; 

    let y = {
        let x = 3;
        x + 1 // if we type ; in here, it will raise error
    };

    println!("{} {}", x, y);
    
    let x = plus_one(5);
    println!("x: {}", x);

}

fn another_function(x : i32, y : i32) {
    println!("another function!:{} {}", x, y);
}

fn plus_one(x : i32) -> i32 {
    x + 1
}

/* JIEUNG: error due to the last ; in the function. return type mismatch
fn plus_one(x : i32) -> i32 {
    x + 1
}
*/
