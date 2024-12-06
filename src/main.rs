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


#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum ASize {
	A4,
	A3,
	A2,
	A1,
}
#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum BSize {
	B5,
	B4,
	B3,
	B2,
	B1,
}

#[derive(Debug)]
#[allow(dead_code)]
enum SizePaper {
	A(ASize),
	B(BSize),
}
impl SizePaper {
	fn as_size(&self) -> (u32, u32) {
		match self {
			SizePaper::A(asize) => match asize {
				ASize::A1 => (594,841),
				ASize::A2 => (420,594),
				ASize::A3 => (297,420),
				ASize::A4 => (210,297),
			},
			SizePaper::B(bsize) => match bsize {
				BSize::B1 => (728,1030),
				BSize::B2 => (515,728),
				BSize::B3 => (364,515),
				BSize::B4 => (257,364),
				BSize::B5 => (182,257),
			},
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
			SizeBan::KK1 => (636,939),
			SizeBan::KK2 => (469,636),
			SizeBan::KK4 => (318,469),
			SizeBan::KK8 => (234,318),

			SizeBan::SR1 => (788,1091),
			SizeBan::SR2 => (545,788),
			SizeBan::SR4 => (394,545),
			SizeBan::SR8 => (272,394),
		}
	}
}

#[derive(Debug)]
#[allow(dead_code)]
struct Product {
	identifier: u8,
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
fn get_imposition_patternn(_machine_size: SizeBan, products: Vec<Product>) /*-> Vec<Vec<u8>>*/ {
	let mut sort_size_a: Vec<(ASize, Vec<u8>)> = Vec::new();
	let mut sort_size_b: Vec<(BSize, Vec<u8>)> = Vec::new();

	//sort products by size and group identifier together based on size
	for product in products {
		match product.size {
			SizePaper::A(asize) => {
				let mut max = sort_size_a.len();
				for i in (0..sort_size_a.len()).rev() {
					if sort_size_a[i].0 <= asize { max = i }
				}
				if max == sort_size_a.len() { sort_size_a.push((asize, vec![product.identifier])) }
				else {
					if sort_size_a[max].0 == asize {
						(sort_size_a[max].1).push(product.identifier);
					}
					else { sort_size_a.insert(max, (asize, vec![product.identifier])) }
				}
			},
			SizePaper::B(bsize) => {
				let mut max = sort_size_b.len();
				for i in (0..sort_size_b.len()).rev() {
					if sort_size_b[i].0 <= bsize { max = i }
				}
				if max == sort_size_b.len() { sort_size_b.push((bsize, vec![product.identifier])) }
				else {
					if sort_size_b[max].0 == bsize {
						(sort_size_b[max].1).push(product.identifier);
					}
					else { sort_size_b.insert(max, (bsize, vec![product.identifier])) }
				}
			},
		}
	}

	println!("{:?}\n{:?}",sort_size_a,sort_size_b);
}

//fn get_imposition(machine_size: Size, products: Vec<Product>) -> Vec<Vec<u8>> {}

fn main() {
	let pro: Vec<Product> = vec![
		Product {
			identifier: 0,
			num: 25000,
			color: 4,
			size: SizePaper::A(ASize::A4)
		},
		Product {
			identifier: 1,
			num: 10000,
			color: 2,
			size: SizePaper::A(ASize::A2),
		},
		Product {
			identifier: 2,
			num: 20000,
			color: 4,
			size: SizePaper::A(ASize::A3),
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

	get_imposition_patternn(SizeBan::KK1, pro);
}
