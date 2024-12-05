const MAX_TIME:u8 = 50;

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

fn init_schedule(kind_of_product: u8, kind_of_machine: u8) -> Vec<u8> {
	let mut schedule: Vec<u8> = Vec::new();

	for _i in 0..(kind_of_product as usize)*(kind_of_machine as usize)*(MAX_TIME as usize) {
		schedule.push(0);
	}

	schedule
}

fn print_schedule(schedule: Vec<u8>) {
	println!("***schedule***");
	for i in 0..schedule.len() {
		if i%(MAX_TIME as usize) == 0 { print!("\n") }
		print!("{}",schedule[i]);
	}
	print!("\n");
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

	let mut schedule: Vec<u8> = init_schedule(pro.len() as u8, mac.len() as u8);
	print_schedule(schedule);
}
