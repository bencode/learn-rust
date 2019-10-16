fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 1;
    println!("The value of x is: {}", y);
    let y = 2;
    println!("The value of x is: {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 34.0;
    let remainer = 43 % 5;

    let t = true;
    let f: bool = false;
    let f2 = false;


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let one = tup.0;
    let two = tup.1;
    println!("The value of is {} {}", one, two);

    let a = [1, 2, 3, 4, 5];

    let b = [3, 3, 3, 3, 3];
    let b2 = [3; 5];

    let f = b[0];

    another_func(f);
}

fn another_func(x: i32) {
    println!("The value of x is: {}", x);
}
