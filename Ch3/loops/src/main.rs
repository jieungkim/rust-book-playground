fn main() {


    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("counter {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("launch!");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5{
        println!("value in index: {}", a[index]);

        index =index + 1;
    }

    for element in a.iter() {
        println!("value in index: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("launch!");
    loop {
        println!("re run");
    }
}
