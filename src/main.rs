const MAX_TIME: u8 = 30;

fn fact(n: u64) -> u64 {
	let mut res: u64 = 1;

	for i in 1..=n {
		res *= i;
	}

	res
}

fn comb(a:u64, b:u64) -> u64 {
	fact(a)/(fact(a-b)*fact(b))
}

#[derive(Debug)]
#[allow(dead_code)]
enum SizePaper {
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
impl SizePaper {
	fn as_size(&self) -> (u32, u32) {
		match self {
			SizePaper::A1 => (26,27),
			SizePaper::A2 => (28,28),
			SizePaper::A3 => (29,30),
			SizePaper::A4 => (31,32),

			SizePaper::B1 => (26,27),
			SizePaper::B2 => (28,28),
			SizePaper::B3 => (29,30),
			SizePaper::B4 => (31,32),
			SizePaper::B5 => (31,32),
		}
	}
}

#[derive(Debug)]
#[allow(dead_code)]
enum SizeBan {
	KK1,
	KK2,
	KK4,
	KK8,

	SR1,
	SR2,
	SR4,
	SR8,
}
impl SizeBan {
	fn as_size(&self) -> (u32, u32) {
		match self {
			SizeBan::KK1 => (26,27),
			SizeBan::KK2 => (28,28),
			SizeBan::KK4 => (29,30),
			SizeBan::KK8 => (31,32),

			SizeBan::SR1 => (26,27),
			SizeBan::SR2 => (28,28),
			SizeBan::SR4 => (29,30),
			SizeBan::SR8 => (31,32),
		}
	}
}

#[derive(Debug)]
#[allow(dead_code)]
struct Product {
	num: u32,
	color: u8,
	size: SizePaper,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Machine {
	color: u8,
	size: SizeBan,
	speed: u32,
}

fn init_schedule(kind_of_product: u8, kind_of_machine: u8) -> Vec<u8> {
	let mut schedule: Vec<u8> = Vec::new();

	for _i in 0..(kind_of_product as usize)*(kind_of_machine as usize)*(MAX_TIME as usize) {
		schedule.push(0);
	}

	schedule
}

fn print_schedule(schedule: Vec<u8>, kind_of_product: u8) {
	println!("***schedule***");
	for i in 0..schedule.len() {
		if i%(MAX_TIME as usize) == 0 {
			if (i/(MAX_TIME as usize))%(kind_of_product as usize) == 0 {
				print!("\nM{:<3}", i/((MAX_TIME as usize)*(kind_of_product as usize)));
			}
			else {
				print!("\n{:<4}","");
			}
		}
		print!("{}",schedule[i]);
	}
	print!("\n");
}

//Pattern of possible number of impositions for each machine size
//fn get_imposition_pattern(machine_size: Size) -> Vec<Vec<u8>> {}

//fn get_imposition(machine_size: Size, products: Vec<Product>) -> Vec<Vec<u8>> {}

fn main() {
	let pro: Vec<Product> = vec![
		Product {
			num: 25000,
			color: 4,
			size: SizePaper::A4
		},
		Product {
			num: 10000,
			color: 2,
			size: SizePaper::A2,
		},
		Product {
			num: 20000,
			color: 4,
			size: SizePaper::A3,
		}
	];

	let mac: Vec<Machine> = vec![
		Machine {
			color: 2,
			size: SizeBan::KK2,
			speed: 5000,
		},
		Machine {
			color: 4,
			size: SizeBan::KK1,
			speed: 5000,
		}
	];

	println!("Product:\n{:#?}",pro);
	println!("Machine:\n{:#?}",mac);

	let mut schedule: Vec<u8> = init_schedule(pro.len() as u8, mac.len() as u8);
	print_schedule(schedule, pro.len() as u8);

	//let impos: Vec<Vec<u8>> = get_imposition();
	//println!("{:?}",impos);
}
