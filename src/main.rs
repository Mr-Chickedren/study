#[derive(Debug)]
#[allow(dead_code)]
enum Size {
	A1,
	A2,
	A3,
	A4,

	B1,
	B2,
	B3,
	B4,
	B5,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Product {
	num: u32,
	color: u8,
	size: Size,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Machine {
	color: u8,
	size: Size,
	speed: u32,
}

fn main() {
	let pro: Vec<Product> = vec![
		Product {
			num: 25000,
			color: 4,
			size: Size::A4,
		},
		Product {
			num: 10000,
			color: 2,
			size: Size::A2,
		},
		Product {
			num: 20000,
			color: 4,
			size: Size::A3,
		}
	];

	let mac: Vec<Machine> = vec![
		Machine {
			color: 2,
			size: Size::A2,
			speed: 5000,
		},
		Machine {
			color: 4,
			size: Size::A1,
			speed: 5000,
		}
	];

	println!("Product:\n{:#?}",pro);
	println!("Machine:\n{:#?}",mac);
}
