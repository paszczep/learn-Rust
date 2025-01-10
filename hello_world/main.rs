
// via _ Compiler will not warn about function being unused
fn _fun_fun() {
	let x: u8 = 42;
	let _ = 42;
}

fn _tuples() {
	let pair_first = ('b', 4);
	pair.0; // this is 'b'
	pair.1; // this is 4
	let pair_second: (char, i32) = ('a', 17);
	// Tuples can be destructured when doing an assignment
	let (value_first, value_second) = ('a', 17);
	
	println!("{pair_second}");
	println!("Hello world!");
	let (val_first, val_second) = pair_second;
	println!("{val_first} {val_second}");
}

fn _if_clause(condition: bool) -> u8 {
	if condition {
		6
	} else {
		3
	}
}


fn main() {
	println!("Hello world!");
	_tuples();
	let _ =_if_clause();
}

