fn expression() {
    let y = {
        let x = 3;
        x + 1 // NO SEMICOLON! - returns value
    };

    println!("The value of y is: {y}");
}

/* this is
    a multiline comment */

fn five() -> u8 {
    5
}

fn flow_control() {
    let number: u8 = 5;
    if number < 3 {
        println!("TRUE")
    } else {
        println!("FALSE")
    }
}

fn flow_control_multiple() {
    let number: u32 = 32;
    if number % 4 == 0 {
        println!("divisible by 4")
    } else if number % 3 == 0 {
        println!("divisible by 3")
    } else if number % 2 == 0 {
        println!("divisible by 2")
    } else {
        println!("not divisible by 4, 3, 2")
    }
}

fn one_line_condition() {
    let condition: bool = true;
    let number:u32 = if condition {5} else {6};

    println!("value {number}")
}

fn loop_loop() {
    let mut counter:u8 = 0;

    let result: u8 = loop {
        counter += 1;
        println!("{counter}");
        if counter == 3 {
            break counter
        }
    };

    println!("result {result}")
}

fn while_loop() {
    let mut countdown: u8 = 3;

    while countdown != 0 {
        println!("while {countdown}");
        countdown -= 1;
    }
}

fn for_loop() {
    let elements: [u8; 4] = [2, 4, 7, 1];

    for el in elements {
        println!("iteration {el}")
    }
}

fn for_loop_range() {
    for el in 1..4 {
        println!("range {el}")
    }
}

fn main() {
    expression();
    five();
    flow_control();
    flow_control_multiple();
    one_line_condition();
    loop_loop();
    while_loop();
    for_loop();
    for_loop_range();
}