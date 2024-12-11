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
enum SizePaper {
	A4,
	A3,
	A2,
	A1,

	B5,
	B4,
	B3,
	B2,
	B1,
}
impl SizePaper {
	fn as_size(&self) -> (u32, u32) {
		match self {
			SizePaper::A1 => (594,841),
			SizePaper::A2 => (420,594),
			SizePaper::A3 => (297,420),
			SizePaper::A4 => (210,297),

			SizePaper::B1 => (728,1030),
			SizePaper::B2 => (515,728),
			SizePaper::B3 => (364,515),
			SizePaper::B4 => (257,364),
			SizePaper::B5 => (182,257),
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
	//return how many pieces fit vertically by (incorrect direction, reverse direction)
	fn fit_vertically(&self, paper_size: &SizePaper, margin: u32) -> (u32, u32){
		let ban_ver = (self.as_size()).0;
		let pap_ver = (paper_size.as_size()).0;
		let pap_hor = (paper_size.as_size()).1;

		(ban_ver / (pap_ver + (margin*2)), ban_ver / (pap_hor + (margin*2)))
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
fn get_imposition_patternn(machines: Vec<Machine>, products: Vec<Product>) /*-> Vec<Vec<u8>>*/ {
	let mut sort_size_a: Vec<(SizePaper, Vec<u8>)> = Vec::new();
	let mut sort_size_b: Vec<(SizePaper, Vec<u8>)> = Vec::new();

	//sort products by size and group identifier together based on size
	for product in products {
		match product.size {
			size if (size == SizePaper::A1) ||
					  (size == SizePaper::A2) ||
					  (size == SizePaper::A3) ||
					  (size == SizePaper::A4) => {
				let mut max = sort_size_a.len();
				for i in (0..sort_size_a.len()).rev() {
					if sort_size_a[i].0 <= size { max = i }
				}
				if max == sort_size_a.len() { sort_size_a.push((size, vec![product.identifier])) }
				else {
					if sort_size_a[max].0 == size {
						(sort_size_a[max].1).push(product.identifier);
					}
					else { sort_size_a.insert(max, (size, vec![product.identifier])) }
				}
			},
			size if (size == SizePaper::B1) ||
					  (size == SizePaper::B2) ||
					  (size == SizePaper::B3) ||
					  (size == SizePaper::B4) ||
					  (size == SizePaper::B5) => {
				let mut max = sort_size_b.len();
				for i in (0..sort_size_b.len()).rev() {
					if sort_size_b[i].0 <= size { max = i }
				}
				if max == sort_size_b.len() { sort_size_b.push((size, vec![product.identifier])) }
				else {
					if sort_size_b[max].0 == size {
						(sort_size_b[max].1).push(product.identifier);
					}
					else { sort_size_b.insert(max, (size, vec![product.identifier])) }
				}
			},
			_ => (),
		}
	}
	println!("{:?}\n{:?}",sort_size_a,sort_size_b);


	for machine in &machines {
		for i in 0..sort_size_a.len() {
			let (ver, hor) = (machine.size).fit_vertically(&sort_size_a[i].0, 10);
			print!("{:?} <- {:?}: ",machine.size, sort_size_a[1].0);
			println!("({}, {})",ver,hor);

			//if ver == 0
		}
	}

}

//fn get_imposition(machine_size: Size, products: Vec<Product>) -> Vec<Vec<u8>> {}

fn main() {
	let pro: Vec<Product> = vec![
		Product {
			identifier: 0,
			num: 25000,
			color: 4,
			size: SizePaper::A4,
		},
		Product {
			identifier: 1,
			num: 10000,
			color: 2,
			size: SizePaper::A2,
		},
		Product {
			identifier: 2,
			num: 20000,
			color: 4,
			size: SizePaper::A3,
		},
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

	get_imposition_patternn(mac, pro);
}
