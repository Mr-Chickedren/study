//#![allow(unused_variables)]
//#![allow(dead_code)]

//#[derive(PartialEq)]
//enum Relationship {
//	Greater,
//	Less,
//	Equal,
//	Error,
//}

#[derive(PartialEq,Debug,Clone)]
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
	//fn comp(&self, a: &String, b: &String) -> Relationship {
	//	if let Some((a_index, row)) = self.dict.iter().find_map(|row| { row.iter().position(|x| x.name == *a).map(|j| (j, row))}) {
	//		if let Some(b_index) = row.iter().position(|x| x.name == *b) {
	//			if a_index == b_index { Relationship::Equal }
	//			else if a_index < b_index { Relationship::Greater }
	//			else { Relationship::Less }
	//		}
	//		else { Relationship::Error }
	//	}
	//	else { Relationship::Error }
	//}
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
	data: Vec<Vec<(String, Vec<usize>)>>,
}
impl Tally {
	fn new() -> Self {
		Self{ data: Vec::new() }
	}
	fn count(&mut self, flist: &FormatList, plist: &Products) {
		for i in 0..flist.dict.len() {
			let mut tmp = Vec::new();
			for j in 0..flist.dict[i].len() {
				let list: Vec<usize> = plist.product.iter().enumerate().filter_map(|(index, x)| { if x.size == flist.dict[i][j].name { Some(index) } else { None } }).collect();
				if !list.is_empty() { tmp.push((flist.dict[i][j].name.clone(), list.clone())) }
			}
			if !tmp.is_empty() { self.data.push(tmp) }
		}
	}
	fn show(&self) {
		println!("*** Tally ***");
		for i in 0..self.data.len() {
			for j in 0..self.data[i].len() {
				println!("{:<4} {:?}", self.data[i][j].0, self.data[i][j].1)
			}
			print!("\n");
		}
	}
}


struct Tessellations {
	pattern: Vec<Vec<Vec<Vec<u8>>>>,
	stage: Vec<Vec<u8>>,
}
impl Tessellations {
	fn new() -> Self {
		Self{ pattern: Vec::new(), stage: Vec::new() }
	}
	fn pack_recursive(&mut self, flist: &FormatList, (m_short,m_long): &(u32,u32), input_products: &Vec<String>, index: usize, margin: u32, dir: Direction, mut result: Vec<u8>) {
		if let Some(p_size) = flist.put_size(&input_products[index]) {
			match dir {
				Direction::Correct => {
					let fit_short = *m_short / (p_size.0+(2*margin));
					if fit_short != 0 && p_size.1+(2*margin) < *m_long {
						result[index] += fit_short as u8;
						self.pack_recursive(flist, &(*m_short,m_long-p_size.1+(2*margin)), input_products, index, margin, dir, result.clone());

						if index != input_products.len()-1 {
							if let Some(dg_size) = flist.downgrade(&input_products[index]) {
								if dg_size == input_products[index+1] {
									self.pack_recursive(flist, &(*m_short,m_long-p_size.1+(2*margin)), input_products, index+1, margin, Direction::Reverse, result.clone());
								}
							}
						}
					}
					else {
						if result.iter().sum::<u8>() != 0 { self.stage.push(result) }
					}
				},
				Direction::Reverse => {
					let fit_short = *m_short / (p_size.1+(2*margin));
					if fit_short != 0 && p_size.0+(2*margin) < *m_long {
						result[index] += fit_short as u8;
						self.pack_recursive(flist, &(*m_short,m_long-p_size.0+(2*margin)), input_products, index, margin, dir, result.clone());
					}
					else {
						if result.iter().sum::<u8>() != 0 { self.stage.push(result) }
					}
				},
			}
		}
	}
	fn pack(&mut self, flist: &FormatList, mlist: &Machines, tally: &Tally, margin: u32) {
		let direction = vec![Direction::Correct, Direction::Reverse];

		for machine_index in 0..mlist.machine.len() {
			self.pattern.push(Vec::new());
			for series_index in 0..tally.data.len() {
				self.pattern[machine_index].push(Vec::new());
				let products_size = tally.data[series_index].iter().map(|(x,_)| x.clone()).collect();
				for product_index in 0..tally.data[series_index].len() {
					if let Some(m_size) = flist.put_size(&mlist.machine[machine_index].size) {
						for dir in &direction {
							self.pack_recursive(flist, &m_size, &products_size, product_index, margin, dir.clone(), vec![0; tally.data[series_index].len()]);
						}
					}
					let mut rm_list = Vec::new();

					for i in 0..self.stage.len() {
						for j in 0..self.stage.len() {
							let i_sum = self.stage[i].iter().sum::<u8>();
							let j_sum = self.stage[j].iter().sum::<u8>();
							if !rm_list.contains(&i) && !rm_list.contains(&j) && i != j && ( (self.stage[i][product_index] == self.stage[j][product_index] && i_sum <= j_sum) || ( i_sum-self.stage[i][product_index] == j_sum-self.stage[j][product_index] && self.stage[i][product_index] <= self.stage[j][product_index] ) ) {rm_list.push(i); break;}
						}
					}

					rm_list.sort_by(|a,b| b.cmp(a));
//					println!("rm:{:?}",rm_list);
//					println!("st:{:?}\n",self.stage);
					for rm_index in rm_list { self.stage.remove(rm_index); }
					self.pattern[machine_index][series_index].extend(self.stage.clone());
					self.stage.clear();
				}
			}
		}
	}
	fn show(&self, mlist: &Machines, tally: &Tally) {
		println!("*** Tessellation ***");
		for machine_index in 0..mlist.machine.len() {
			println!("{}", mlist.machine[machine_index].size);
			for series_index in 0..tally.data.len() {
				println!("{:?}", tally.data[series_index].iter().map(|(x,_)| x.clone()).collect::<Vec<_>>());
				for index in 0..self.pattern[machine_index][series_index].len() {
					println!("{:?}", self.pattern[machine_index][series_index][index]);
				}
			}
			print!("\n");
		}
	}
}

struct Impositions {
	pattern: Vec<Vec<Vec<usize>>>,
}
impl Impositions {
	fn new() -> Self {
		Self { pattern: Vec::new() }
	}
	fn calc(&mut self, tally: &Tally, tess: &Tessellations, plist: &Products) {
		let mut imp_total: Vec<Vec<Vec<usize>>> = vec![Vec::new();tess.pattern.len()];

		for machine_index in 0..tess.pattern.len() {
			//println!("machine{}:\n-------------------",machine_index);

			for series_index in 0..tess.pattern[machine_index].len() {
				for kinds_index in 0..tess.pattern[machine_index][series_index].len() {

					let mut combos_select_total: Vec<Vec<Vec<usize>>> = Vec::new();
					for format_index in 0..tess.pattern[machine_index][series_index][kinds_index].len() {
						//println!("{}:{:?}",tess.pattern[machine_index][series_index][kinds_index][format_index],tally.data[series_index][format_index].1);

						let mut combos_select = Vec::new();
						if tess.pattern[machine_index][series_index][kinds_index][format_index] != 0 {
							generate_select_combinations(&tally.data[series_index][format_index].1, tess.pattern[machine_index][series_index][kinds_index][format_index] as usize, Vec::new(), &mut Vec::new(), &mut combos_select);
							combos_select_total.push(combos_select);
						}
					}
					//println!("{:?}",combos_select_total);
					let mut combos_han: Vec<Vec<Vec<usize>>> = Vec::new();
					generate_combination(&combos_select_total, &mut combos_han);
					//println!("{:?}",combos_han);

					for i_comb in 0..combos_han.len() {
						let mut tmp = vec![0;plist.product.len()];
						let imp_han: Vec<usize> = combos_han[i_comb].clone().into_iter().flat_map(|x| x).collect();
						for i_imp in &imp_han {
							tmp[*i_imp] += 1;
						}
						imp_total[machine_index].push(tmp);
					}
					//println!("");

				}
			}
		}

		generate_combination(&imp_total, &mut self.pattern);

		for i in 0..self.pattern.len() {
			println!("{:?}", self.pattern[i]);
		}
	}
	fn show(&self, mlist: &Machines, plist: &Products) {
		println!("*** Impositions ***");
		for i in 0..mlist.machine.len() {
			println!("#{}", mlist.machine[i].size);
			for j in 0..self.pattern[i].len() {
				for k in 0..plist.product.len() {
					print!("{:>3}", k);
				}
				print!("\n");
				for k in 0..self.pattern[i][j].len() {
					print!("{:>3}", self.pattern[i][j][k]);
				}
				print!("\n");
			}
		}
		print!("\n");
	}
}

fn generate_select_combinations(chars: &Vec<usize>, s: usize, current: Vec<usize>, count: &mut Vec<Vec<usize>>, results: &mut Vec<Vec<usize>>) {
	if current.len() == s {
		for cnt in count.clone() {
			let mut tmp = 0;
			for i in 0..chars.len() {
				if current.iter().filter(|&c| *c == chars[i]).count() == cnt[i] { tmp += 1 }
			}
			if tmp == chars.len() { return }
		}

		let mut cnt = vec![0;chars.len()];
		for i in 0..chars.len() {
			cnt[i] = current.iter().filter(|&c| *c == chars[i]).count();
		}
		count.push(cnt);
		results.push(current);
		return;
	}

   for c in chars.clone() {
      let mut new_current = current.clone();
      new_current.push(c);
      generate_select_combinations(chars, s, new_current, count, results);
   }
}

fn generate_combination(target: &Vec<Vec<Vec<usize>>>, result: &mut Vec<Vec<Vec<usize>>>) {
	let mut count = vec![0;target.len()];
	let mut r = 1;

	for i in 0..target.len() { r *= target[i].len() }

	for _ in 0..r {
		let mut tmp:Vec<Vec<usize>> = Vec::new();
		for i in 0..target.len() {
			tmp.push(target[i][count[i]].clone());
		}
		result.push(tmp);

		count[0] += 1;
		for i in 0..count.len()-1 {
			if count[i] == target[i].len() {
				count[i] = 0;
				count[i+1] += 1;
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
//	flist.show();

	let mut plist = Products::new();
	plist.add(&flist, "A3", 4, 25000);
	plist.add(&flist, "A4", 4, 25000);
	plist.add(&flist, "A2", 2, 10000);
	plist.add(&flist, "A3", 4, 20000);
//	plist.add(&flist, "A3", 3, 20000);
//	plist.add(&flist, "B1", 3, 20000);
//	plist.add(&flist, "B4", 3, 20000);
//	plist.add(&flist, "B4", 3, 20000);
//	plist.add(&flist, "A2", 3, 20000);
//	plist.show();

	let mut mlist = Machines::new();
	mlist.add(&flist, "KK1", 2, 5000);
	mlist.add(&flist, "KK2", 4, 5000);
	//mlist.add(&flist, "SR1", 4, 5000);
//	mlist.show();

	let mut tally = Tally::new();
	tally.count(&flist, &plist);
//	tally.show();

	let mut tess = Tessellations::new();
	tess.pack(&flist, &mlist, &tally, 10);
//	tess.show(&mlist, &tally);

	let mut impo = Impositions::new();
	impo.calc(&tally, &tess, &plist);
	//impo.show(&mlist,&plist);
}
