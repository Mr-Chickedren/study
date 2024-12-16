#[derive(PartialEq)]
enum Relationship {
	Greater,
	Less,
	Equal,
	Error,
}

enum Direction {
	Vertical,
	Horizontal,
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
	fn comp(&self, a: &String, b: &String) -> Relationship {
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
				if *a == format.name { size_a = format.size.0 * format.size.1 }
				if *b == format.name { size_b = format.size.0 * format.size.1 }
			}
			if	size_a == 0 || size_b == 0 { Relationship::Error }
			else if size_a == size_b { Relationship::Equal }
			else if size_a > size_b { Relationship::Greater }
			else { Relationship::Less }
		}
		else { Relationship::Error }
	}
	fn add_series(&mut self, series: &str) {
		if !series.bytes().any(|b| b.is_ascii_digit()) &&
			!self.series_list.contains(&series.to_string()) {
			self.series_list.push(series.to_string());
			self.dict.push(Vec::new());
		}
		else {
			println!("Error: \"{}\" is already exists. or using numbers in the series-name.", series)
		}
	}
	fn add_format(&mut self, format: &str, size: (u32,u32)) {
		let mut series = String::new();
		for ch in format.chars() {
			if !ch.is_digit(10) { series.push(ch) }
			else { break }
		}
		if let Some(series_index) = self.series_list.iter().position(|x| *x == series) {
			let mut max = self.dict[series_index].len();
			for i in 0..self.dict[series_index].len(){
				if self.dict[series_index][i].size.0*self.dict[series_index][i].size.1 < size.0*size.1 { max = i; break; }
			}
			self.dict[series_index].insert(max, Format{ name: format.to_string(), size: size });
		}
		else { println!("Error: \"{}\" is not exists. Please excute \"add_series()\"", format) }
	}
	fn fit(&self, machine_size: &String, input_direction: Direction, format: &String, product_direction: Direction, margin: u32) -> u32 {
		let mut ms = (0,0);
		let mut fs = (0,0);
		for i in 0..self.dict.len() {
			for j in 0..self.dict[i].len() {
				if self.dict[i][j].name == *machine_size { ms =  self.dict[i][j].size }
				if self.dict[i][j].name == *format { fs =  self.dict[i][j].size }
			}
		}

		if fs == (0,0) { return 0 }
		
		match input_direction {
			Direction::Vertical => match product_direction {
				Direction::Vertical => (ms.0 / (fs.1 + (2*margin))) as u32,
				Direction::Horizontal => (ms.0 / (fs.0 + (2*margin))) as u32,
			},
			Direction::Horizontal => match product_direction {
				Direction::Vertical => (ms.1 / (fs.0 + (2*margin))) as u32,
				Direction::Horizontal => (ms.1 / (fs.1 + (2*margin))) as u32,
			},
		}
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
	fn add(&mut self, format_list: &FormatList, size: &str, color: u8, num: u32,) {
		let mut exist = false;
		for i in 0..format_list.dict.len() {
			for j in 0..format_list.dict[i].len() {
				if format_list.dict[i][j].name == size.to_string() { exist = true; break; }
			}
		}
		if exist { self.product.push(Product{ size: size.to_string(), color: color, num: num }) }
		else { println!("Error: \"{}\" is not exist. Please excute \"add_format\".", size) }
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
	fn add(&mut self, format_list: &FormatList, size: &str, color: u8, speed: u32) {
		let mut exist = false;
		for i in 0..format_list.dict.len() {
			for j in 0..format_list.dict[i].len() {
				if format_list.dict[i][j].name == size.to_string() { exist = true; break; }
			}
		}
		if exist { self.machine.push(Machine{ size: size.to_string(), color: color, speed: speed }) }
		else { println!("Error: \"{}\" is not exist. Please excute \"add_format\".", size) }
	}
	fn show(&self) {
		println!("*** Machines ***");
		for i in 0..self.machine.len() {
			println!("{:>2}:{:>4} {:1} {}", i, self.machine[i].size, self.machine[i].color, self.machine[i].speed);
		}
		print!("\n");
	}
}

struct Category {
	format: String,
	product_indexes: Vec<usize>,
}
struct Sort {
	series_list: Vec<String>,
	dict: Vec<Vec<Category>>,
}
impl Sort {
	fn new() -> Self {
		Self { series_list: Vec::new(), dict: Vec::new() }
	}
	fn sort(&mut self, format_list: &FormatList, products: &Products) {
		let mut series = String::new();
		let mut exist_series = false;
		let mut exist_name = false;
		let mut max = 0;

		for product_index in 0..products.product.len() {
			series = String::new();
			exist_series = false;

			for ch in products.product[product_index].size.clone().chars() {
				if !ch.is_digit(10) { series.push(ch) }
				else { break }
			}
			for slist_index in 0..self.series_list.len() {
				if self.series_list[slist_index] == series {
					exist_series = true;

					exist_name = false;
					for dict_index in 0..self.dict[slist_index].len() {
						if self.dict[slist_index][dict_index].format == products.product[product_index].size.clone() {
							exist_name = true;
							self.dict[slist_index][dict_index].product_indexes.push(product_index);
							break;
						}
					}
					if !exist_name {
						max = self.dict[slist_index].len();
						for dict_index in 0..self.dict[slist_index].len() {
							if format_list.comp(&products.product[product_index].size, &self.dict[slist_index][dict_index].format) == Relationship::Greater { max = dict_index; break; }
						}
						self.dict[slist_index].insert(max, Category{ format: products.product[product_index].size.clone(), product_indexes: vec![product_index] });
					}

					break;
				}
			}
			if !exist_series {
				self.series_list.push(series);
				self.dict.push(vec![Category{ format: products.product[product_index].size.clone(), product_indexes: vec![product_index] }]);
			}
		}
	}
	fn put_sizes(&self, series: &String) -> Vec<String> {
		let pos = self.series_list.iter().position(|x| *x == *series);
		let mut sizes = Vec::new();
		match pos {
			Some(index) => {
				for i in 0..self.dict[index].len() { sizes.push(self.dict[index][i].format.clone()) }
			},
			None => { return Vec::new() }
		}
		sizes
	}
	fn show(&self) {
		println!("*** Products_Sort ***");
		for i in 0..self.dict.len() {
			println!("#{}", self.series_list[i]);
			for j in 0..self.dict[i].len() {
				println!(" -{:>4} {:?}", self.dict[i][j].format, self.dict[i][j].product_indexes);
			}
		}
		print!("\n");
	}
}

struct Pack {
	machine_size: String,
	series_name: String,
	illustration: Vec<u8>,
}
struct Packing {
	list: Vec<Pack>,
}
impl Packing {
	fn new() -> Self {
		Self { list: Vec::new() }
	}/*
	fn pack(&mut self, format_list: &FormatList, mlist: &Machines, sort: &Sort, margin: u32) {
		for machine_index in 0..mlist.machine.len() {
			for series_index in 0..sort.series_list.len() {
				let mut fit_ver = 0;
				println!("{}:{}:Horizontal", mlist.machine[machine_index].size, sort.series_list[series_index]);

				for i in 0..sort.dict[series_index].len() {
					let mut ill: Vec<u8> = vec![0; sort.dict[series_index].len()];
					fit_ver = format_list.fit(&mlist.machine[machine_index].size, Direction::Vertical, &sort.dict[series_index][i].format, Direction::Horizontal, margin);
					ill[i] = (fit_ver * format_list.fit(&mlist.machine[machine_index].size, Direction::Horizontal, &sort.dict[series_index][i].format, Direction::Horizontal, margin)) as u8;
					if ill[i] != 0 {
						self.list.push(Pack {machine_size: mlist.machine[machine_index].size.clone(), series_name: sort.series_list[series_index].clone(), illustration: ill.clone()});
						println!("{:?}",ill.clone());
					}

					while ill.iter().sum::<u8>() != ill[sort.dict[series_index].len() - 1] {
						if (ill[i] - fit_ver as u8) == 0 { break }
						else if let Some(dg_index) = sort.put_sizes(&sort.series_list[series_index]).iter().position(|&x| x == /* downgrade &sort.dict[series_index][i].format*/) {
							
						}
						else { break }
					}
				}

				//self.list.push(Pack {machine_size: mlist.machine[machine_index].size.clone(), series_name: sort.series_list[series_index].clone(), illustration: ill.clone()});
			}
		}
	}*/
	fn show(&self) {

		print!("\n");
	}
}

//struct Imposition {}
	
fn main() {
	let mut flist = FormatList::new();
	flist.add_series("A");
	flist.add_series("B");
	flist.add_series("KK");
	flist.add_series("SR");
	flist.add_format("A1", (594,841));
	flist.add_format("A2", (420,594));
	flist.add_format("A3", (297,420));
	flist.add_format("A4", (210,297));
	flist.add_format("B1", (728,1030));
	flist.add_format("B2", (515,728));
	flist.add_format("B3", (364,515));
	flist.add_format("B4", (257,364));
	flist.add_format("B5", (182,257));
	flist.add_format("KK1", (636,939));
	flist.add_format("KK2", (469,636));
	flist.add_format("KK4", (318,469));
	flist.add_format("KK8", (234,318));
	flist.add_format("SR1", (788,1091));
	flist.add_format("SR2", (545,788));
	flist.add_format("SR4", (394,545));
	flist.add_format("SR8", (272,394));
	flist.show();

	let mut plist = Products::new();
	plist.add(&flist, "A3", 4, 25000);
	plist.add(&flist, "A4", 4, 25000);
	plist.add(&flist, "A2", 2, 10000);
	plist.add(&flist, "A3", 4, 20000);
	plist.add(&flist, "A3", 3, 20000);
	plist.add(&flist, "B1", 3, 20000);
	plist.add(&flist, "B4", 3, 20000);
	plist.add(&flist, "B3", 3, 20000);
	plist.add(&flist, "B3", 3, 20000);
	plist.show();

	let mut mlist = Machines::new();
	mlist.add(&flist, "KK1", 2, 5000);
	mlist.add(&flist, "KK2", 4, 5000);
	mlist.show();

	let mut products_sort = Sort::new();
	products_sort.sort(&flist, &plist);
	products_sort.show();

	let mut pk = Packing::new();
	//pk.pack(&flist, &mlist, &products_sort, 10);
	//pk.show();
}
