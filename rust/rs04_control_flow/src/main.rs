fn main() {
	// Block expression
	let x = {
		let i = 5;
		i + 10
	};

	println!("x: {}", x);

	// if
	let number = 6;
	if number % 4 == 0 {
		println!("number is divisible by 4");
	}
	else if number % 3 == 0 {
		println!("number is divisible by 3");
	}
	else if number % 2 == 0 {
		println!("number is divisible by 2");
	}
	else {
		println!("not divisible by 4, 3, or 2");
	}

	let n = 6;
	let even = if n % 2 == 0 {
		true
	}
	else {
		false
	};

	println!("Is even: {}", even);

	// while
	let mut c = 5;
	while c > 0 {
		println!("In while, c: {}", c);
		c -= 1;
	}

	// for
	for i in 5..10 {
		println!("In for, i: {}", i);
	}

	//TODO: Use a for loop to print out the ints from 0-9 (inclusive)

	for i in 0..=9{
		print!("{}, ",i);
	}
	println!("");

	//TODO: Use a for loop to print out the ints from 1-10 (inclusive)

	for i in 1..11{
		print!("{}, ",i);
	}
	println!("");

	// loop
	let mut n = 6;
	let m = loop {
		if n > 10 {
			break n
		}
		n += 1;
	};

	println!("m: {}", m);

	// Function test
	let c = 15;
	let d = 20;
	println!("{} + {} is: {}", c, d, a_func(c, d));
}

fn a_func(a: i32, b: i32) -> i32 {
	a + b
}
