
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn mutability() {
    let mut x: u8 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn reusing_name() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}")
}

fn shadowing() {
    let x: u8 = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("inner x : {x}");
    }
    
    println!("basic x: {x}");
 
}

fn parse_expect() {
    let value: u16 = "42".parse().expect("Not a number!");
    
    println!("value value: {value}");
}

fn how_to_char() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");
}

fn tuple_declaration() {
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let x = _tup.0;
    println!("first tuple value: {x}");
    
    let (_, y, _) = _tup;
    println!("second tuple value: {y}");
    
    let z = _tup.2;
    println!("third tuple value: {z}");
}

fn array_declaration() {
    // the stack rather than the heap
    // every element of an array must have the same type
    // arrays in Rust have a fixed length
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let _m = months[2];
    println!("third month: {_m}");
    let _arr: [u8; 5] = [1, 2, 3, 4, 5];
    let _val = _arr[2];
    println!("third array val: {_val}");
}

fn fun_fun() {

    let x = 2.0;
  
    println!("{x}");
  
  }

fn main() {

    mutability();
    reusing_name();
    println!("{THREE_HOURS_IN_SECONDS}");
    shadowing();
    parse_expect();
    how_to_char();
    tuple_declaration();
    array_declaration();
    fun_fun();
}
