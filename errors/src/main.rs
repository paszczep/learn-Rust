use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");
    dbg!(&file_result);

    let greeting_file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {error:?}")
    };

    
    println!("Hello, world!");
}
