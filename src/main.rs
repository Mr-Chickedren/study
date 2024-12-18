#[derive(PartialEq)]
enum Relationship {
	Greater,
	Less,
	Equal,
	Error,
}

#[derive(PartialEq)]
enum Direction {
	Correct,
	Reverse,
}

#[derive(Debug)]
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
	fn comp(&self, a: &String, b: &String) -> Relationship {
		if let Some((a_index, row)) = self.dict.iter().find_map(|row| { row.iter().position(|x| x.name == *a).map(|j| (j, row))}) {
			if let Some(b_index) = row.iter().position(|x| x.name == *b) {
				if a_index == b_index { Relationship::Equal }
				else if a_index < b_index { Relationship::Greater }
				else { Relationship::Less }
			}
			else { Relationship::Error }
		}
		else { Relationship::Error }
	}
	fn downgrade(&self, size: &String) -> Option<String> {
		if let Some((index, row)) = self.dict.iter().find_map(|row| { row.iter().position(|x| x.name == *size).map(|j| (j, row))}) {
			if index != row.len()-1 { Some(row[index+1].name.clone()) }
			else { None }
		}
		else { None }
	}
	fn put_size(&self, name: &String) -> Option<(u32,u32)> {
		if let Some(format) = self.dict.iter().flatten().find(|x| x.name == *name) { Some(format.size) }
		else { None }
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
		if format_list.dict.iter().any(|row| { row.iter().any(|item| item.name == *size) }) { self.product.push(Product{ size: size.to_string(), color: color, num: num }) }
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
		if format_list.dict.iter().any(|row| { row.iter().any(|item| item.name == *size) }) { self.machine.push(Machine{ size: size.to_string(), color: color, speed: speed }) }
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

struct Tally {
	data: Vec<Vec<(String, u8)>>,
}
impl Tally {
	fn new() -> Self {
		Self{ data: Vec::new() }
	}
	fn count(&mut self, flist: &FormatList, plist: &Products) {
		for i in 0..flist.dict.len() {
			let mut tmp = Vec::new();
			for j in 0..flist.dict[i].len() {
				let num = plist.product.iter().filter(|x| x.size == flist.dict[i][j].name).count();
				if num != 0 { tmp.push((flist.dict[i][j].name.clone(), num as u8)) }
			}
			if !tmp.is_empty() { self.data.push(tmp) }
		}
	}
	fn show(&self) {
		println!("*** Tally ***");
		for i in 0..self.data.len() {
			for j in 0..self.data[i].len() {
				println!("{:<4} {}", self.data[i][j].0, self.data[i][j].1)
			}
			print!("\n");
		}
	}
}

struct Tessellations {
	pattern: Vec<Vec<Vec<u8>>>,
}
impl Tessellations {
	fn new() -> Self {
		Self{ pattern: Vec::new() }
	}
	fn pack(&mut self, flist: &FormatList, (m_short,m_long): &(u32,u32), input_products: &Vec<String>, index: usize, margin: u32, dir: Direction, mut result: Vec<u8>) {
		let mut fit_short: u32 = 0;

		if let Some(p_size) = flist.put_size(&input_products[index]) {
			match dir {
				//under construction
				Direction::Correct => {
					fit_short = *m_short / (p_size.0+(2*margin));
					if fit_short != 0 && p_size.1+(2*margin) < *m_long {
						result[index] += fit_short as u8;
						println!("[{},{}] <- {}*{}", m_short, m_long, input_products[index], fit_short);
						self.pack(flist, &(*m_short,m_long-p_size.1+(2*margin)), input_products, index, margin, dir, result.clone());
					}
					else { println!("{:?}",result) }
				},
				//ok
				Direction::Reverse => {
					fit_short = *m_short / (p_size.1+(2*margin));
					if fit_short != 0 && p_size.0+(2*margin) < *m_long {
						result[index] += fit_short as u8;
						println!("[{},{}] <- {}*{}", m_short, m_long, input_products[index], fit_short);
						self.pack(flist, &(*m_short,m_long-p_size.0+(2*margin)), input_products, index, margin, dir, result.clone());
					}
					else { println!("{:?}",result) }
				},
			}
		}
	}
}
	
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

	let mut tally = Tally::new();
	tally.count(&flist, &plist);
	tally.show();

	let mut tess = Tessellations::new();
	tess.pack(&flist, &(636,939), &vec!["A2".to_string(),"A3".to_string(),"A4".to_string()], 2, 10, Direction::Reverse, vec![0;3]);
}
