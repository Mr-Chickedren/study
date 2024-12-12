#[derive(Debug)]
enum Relationship {
	Greater,
	Less,
	Equal,
	Error,
}

struct Format {
	name: String,
	size: (u32,u32),
}

struct FormatList {
	series_list: Vec<String>,
	dict: Vec<Vec<Format>>,
}
impl FormatList {
	fn new() -> Self {
		Self { series_list: Vec::new(), dict: Vec::new() }
	}
	fn comp(&self, a: String, b: String) -> Relationship {
		let mut series_a = String::new();
		let mut series_b = String::new();
		let mut exist = false;
		let mut i = 0;
		let mut size_a = 0;
		let mut size_b = 0;
		for ch in a.chars() {
			if !ch.is_digit(10) { series_a.push(ch) }
			else { break }
		}
		for ch in b.chars() {
			if !ch.is_digit(10) { series_b.push(ch) }
			else { break }
		}
		for series in self.series_list.clone() {
			if series == series_a {
				exist = true;
				break;
			}
			i += 1;
		}
		if exist && series_a == series_b {
			for format in &self.dict[i] {
				if a == format.name { size_a = format.size.0 * format.size.1 }
				if b == format.name { size_b = format.size.0 * format.size.1 }
			}
			if	size_a == 0 || size_b == 0 { Relationship::Error }
			else if size_a == size_b { Relationship::Equal }
			else if size_a > size_b { Relationship::Greater }
			else { Relationship::Less }
		}
		else { Relationship::Error }
	}
	fn add_series(&mut self, series: &str) {
		let mut exist = false;
		for i in 0..self.series_list.len() {
			if series.to_string() == self.series_list[i] { exist = true }
		}
		if !exist {
			self.series_list.push(series.to_string());
			self.dict.push(Vec::new());
		}
	}
	fn add_format(&mut self, format: &str, size: (u32,u32)) {
		let mut series = String::new();
		let mut exist = false;
		let mut ind = 0;
		for ch in format.chars() {
			if !ch.is_digit(10) { series.push(ch) }
			else { break }
		}
		for i in 0..self.series_list.len() {
			if series == self.series_list[i] {
				exist = true;
				ind = i;
				break;
			}
		}
		if exist { self.dict[ind].push(Format{ name: format.to_string(), size: size }) }
		else { println!("Error: not exist \"{}\". Please excute \"add_series()\"", format) }
	}
	fn show(&self) {
		println!("*** Format_List ***");
		for i in 0..self.dict.len() {
			println!("#{}", self.series_list[i]);
			for j in 0..self.dict[i].len() {
				println!(" -{:>4} {:4}*{:4}", self.dict[i][j].name, self.dict[i][j].size.0, self.dict[i][j].size.1);
			}
		}
		print!("\n");
	}
}

struct Product {
	size: String,
	color: u8,
	num: u32,
}

struct Products {
	product: Vec<Product>,
}
impl Products {
	fn new() -> Self {
		Self { product: Vec::new() }
	}
	fn add(&mut self, size: &str, color: u8, num: u32,) {
		self.product.push(Product{ size: size.to_string(), color: color, num: num });
	}
	fn show(&self) {
		println!("*** Products ***");
		for i in 0..self.product.len() {
			println!("{:>2}:{:>4} {:1} {}", i, self.product[i].size, self.product[i].color, self.product[i].num);
		}
		print!("\n");
	}
}

struct Machine {
	size: String,
	color: u8,
	speed: u32,
}

struct Machines {
	machine: Vec<Machine>,
}
impl Machines {
	fn new() -> Self {
		Self { machine: Vec::new() }
	}
	fn add(&mut self, size: &str, color: u8, speed: u32) {
		self.machine.push(Machine{ size: size.to_string(), color: color, speed: speed });
	}
	fn show(&self) {
		println!("*** Machines ***");
		for i in 0..self.machine.len() {
			println!("{:>2}:{:>4} {:1} {}", i, self.machine[i].size, self.machine[i].color, self.machine[i].speed);
		}
		print!("\n");
	}
}

	
fn main() {
	let mut flist = FormatList::new();
	flist.add_series("A");
	flist.add_series("B");
	flist.add_format("A1", (594,841));
	flist.add_format("A2", (420,594));
	flist.add_format("A3", (297,420));
	flist.add_format("A4", (210,297));
	flist.add_format("B1", (728,1030));
	flist.add_format("B2", (515,728));
	flist.add_format("B3", (364,515));
	flist.add_format("B4", (257,364));
	flist.add_format("B5", (182,257));
	flist.show();

	let mut plist = Products::new();
	plist.add("A4", 4, 25000);
	plist.add("A2", 2, 10000);
	plist.add("A3", 4, 20000);
	plist.add("A3", 3, 20000);
	plist.show();

	let mut mlist = Machines::new();
	mlist.add("KK1", 2, 5000);
	mlist.add("KK2", 4, 5000);
	mlist.show();

	println!("{:?}",flist.comp("A1".to_string(),"A3".to_string()))
}
