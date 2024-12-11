#[derive(Debug)]
struct Product {
	num: u32,
	color: u8,
	size: String,
}

#[derive(Debug)]
struct Products {
	product: Vec<Product>,
}
impl Products {
	fn new() -> Self {
		Self { product: Vec::new() }
	}
	fn addition(&mut self, num: u32, color: u8, size: String) {
		self.product.push(Product{ num: num, color: color, size: size });
	}
}

#[derive(Debug)]
struct Machine {
	color: u8,
	size: (String, usize),
	speed: u32,
}

fn main() {
	let plist = Products::new();
	plist.addition(25000, 4, "A4");
	plist.addition(10000, 2, "A2");
	plist.addition(20000, 4, "A3");
	plist.addition(20000, 3, "A3");

	println!("{:?}",plist);
}
