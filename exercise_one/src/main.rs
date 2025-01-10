fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    /*
        Take the °F temperature and subtract 32.
        Multiply this number by 5.
        Divide this number by 9 to obtain your answer in °C.
    */
    (temperature - 32) * 5/9
}

/*
    def fibonacci_iterative(n):
        a, b = 0, 1
        for _ in range(n):
            a, b = b, a + b
        return a
*/

fn fibonacci_iterative(n: u32) -> u32 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    return a
}

fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n
    } else { 
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn main() {
    let temperature_value = fahrenheit_to_celsius(-5);
    println!("{temperature_value}");

    let fib_iter_val = fibonacci_iterative(25);
    println!("{fib_iter_val}");

    let fib_recur_val = fibonacci_recursive(25);
    println!("{fib_recur_val}");

}
