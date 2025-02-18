const ROUNDING_DIGIT: f32 = 4.0;
const DEBUG_IMPOSITION: bool = false;
const DEBUG_NUMBER_SYSTM: bool = false;
const DEBUG_ITER_COUNTER: bool = true;
const DEBUG_PUT_ERROR: bool = false;

enum Error {
	//Optimal solution does not exist	
	NotExist(String),
	//The first step failed
	FailedPhase(String),
	//The result is not what is needed
	NotNeed(String),
}

#[derive(PartialEq,Clone)]
enum Direction {
	Correct,
	Reverse,
}

#[derive(Debug, Clone)]
enum Attribute {
	I(Vec<u8>),
	P(Vec<u8>),
}
impl Attribute {
	fn extract(&self) -> Vec<u8> {
		match self {
			Attribute::I(x) => x.clone(),
			Attribute::P(x) => x.clone(),
		}
	}
	fn extract_conditional(&self) -> Vec<u8> {
		match self {
			Attribute::I(x) => {
				let mut tmp = x.clone();
				for t in &mut tmp { *t = 0 }
				tmp
			},
			Attribute::P(x) => x.clone(),
		}
	}
	fn len(&self) -> usize {
		match self {
			Attribute::I(x) => x.len(),
			Attribute::P(x) => x.len(),
		}
	}
}

struct Iter {
	first: usize,
	last: usize,
}
impl Iter {
	fn new(f: usize, l: usize) -> Self {
		Self { first: f, last: l }
	}
	fn into_iter(&self) -> impl Iterator<Item = usize> {
		self.first..self.last
	}
	fn first(&self) -> usize {
		self.first
	}
	fn last(&self) -> usize {
		self.last
	}
}

// Paper and machine combined
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
	// output one size smaller
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

// Classify ordered items into different sizes
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

// Keep track of how many papers of each size fit on a printing plate
struct Tessellations {
	pattern: Vec<Vec<Vec<Vec<u8>>>>,
	stage: Vec<Vec<u8>>,
}
impl Tessellations {
	fn new() -> Self {
		Self{ pattern: Vec::new(), stage: Vec::new() }
	}
	// Recursive functions
	fn pack_recursive(&mut self, flist: &FormatList, (m_short,m_long): &(u32,u32), input_products: &Vec<String>, index: usize, margin: u32, dir: Direction, mut result: Vec<u8>) {
		if let Some(p_size) = flist.put_size(&input_products[index]) {
			match dir {
				// case: put it on the side
				Direction::Correct => {
					let fit_short = *m_short / (p_size.0+(2*margin));
					if fit_short != 0 && p_size.1+(2*margin) < *m_long {
						result[index] += fit_short as u8;
						self.pack_recursive(flist, &(*m_short,m_long-p_size.1+(2*margin)), input_products, index, margin, dir, result.clone());

						if index != input_products.len()-1 {
							// case: exist one size smaller
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
				// case: put it vertically
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
	// The actual function to be called
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
	// machine * pattern_index * product_number
	pattern: Vec<Vec<Vec<u8>>>,
}
impl Impositions {
	fn new() -> Self {
		Self { pattern: Vec::new() }
	}
	fn calc(&mut self, tally: &Tally, tess: &Tessellations) {

		// number of products
		let mut n_products = 0;
		for i in 0..tally.data.len() {
			for j in 0..tally.data[i].len() {
				n_products += tally.data[i][j].1.len();
			}
		}

		for machine_index in 0..tess.pattern.len() {

			let mut stage = Vec::new();
			for series_index in 0..tess.pattern[machine_index].len() {
				for i in 0..tess.pattern[machine_index][series_index].len() {

					// calc imposition by DP and retain sequence (product number)
					let mut tmp = Vec::new();
					let mut sequence = Vec::new();
					for format_index in 0..tess.pattern[machine_index][series_index][i].len() {
						let dp_result = dp_enumerate(
							tally.data[series_index][format_index].1.len(),
							tess.pattern[machine_index][series_index][i][format_index] as usize
						);
						tmp.push(dp_result);
						sequence.extend(tally.data[series_index][format_index].1.clone());
					}

					// do flat [[4],[0,2]] -> [4,0,2]
					let comb_result: Vec<Vec<u8>> = generate_combinations(&tmp).into_iter().map(
						|matrix| matrix.into_iter().flatten().collect::<Vec<u8>>()
					).collect();

					if DEBUG_IMPOSITION {
						println!("--");
						for t in &comb_result {println!("{:?}",t)}
						println!("seq:{:?}",sequence);
					}

					// sort (product number) and push
					for j in 0..comb_result.len() {
						let mut tmp = vec![0;n_products];
						for k in 0..sequence.len() {
							tmp[sequence[k] as usize] = comb_result[j][k];
						}

						if DEBUG_IMPOSITION { println!("{:?}",tmp) }

						stage.push(tmp);
					}
				}
			}
			self.pattern.push(stage);
		}
	}
	// all pattern (select attribute for machine)
	fn generate_select_machine_patterns(&self) -> Vec<Vec<bool>> {
		let mut s: Vec<Vec<bool>> = Vec::new();
		for binary in 1..2_u32.pow(self.pattern.len() as u32) {
			s.push( (0..self.pattern.len()).rev().map(|i| (binary & (1 << i))!=0).collect() );
		}
		s
	}
	// number of each pattern combinations
	fn total_impositions_each_pattern(&self, use_imp: Option<usize>) -> (usize, Vec<usize>, Option<usize>) {
		let s = self.generate_select_machine_patterns();
		let mut n_imps = Vec::new();
		for i in 0..s.len() {
			let mut tmp = 1;
			for j in 0..s[i].len() {
				if s[i][j] { tmp *= self.pattern[j].len() }
			}
			n_imps.push(tmp);
		}

		let total_imp = n_imps.iter().sum::<usize>();

		match use_imp {
			Some(x) => {
				let mut total_use = 1;
				for i in 0..x {
					total_use *= total_imp - i;
				}

				(total_imp, n_imps, Some(total_use))
			},
			None => {
				(total_imp, n_imps, None)
			},
		}

	}
	// index -> each pattern(impositions) -> 2 phase simplex method
	fn search_optimal_solution(&self, plist: &Products, mlist: &Machines, n_use_imp: usize, use_imp_iter: Iter) -> Option< (Vec<Vec<f32>>, Vec<usize>, Vec<Vec<Attribute>>) > {

		let mut best: Option< (Vec<Vec<f32>>, Vec<usize>, Vec<Vec<Attribute>>) > = None;

		// all pattern (select attribute for machine)
		let s_attr = self.generate_select_machine_patterns();
		if DEBUG_NUMBER_SYSTM {
			for tmp in &s_attr { println!("{:?}",tmp) }
		}

		// number of each pattern combinations
		let (total_imp, n_imps, total_use) = self.total_impositions_each_pattern(Some(n_use_imp));

		for number in use_imp_iter.into_iter() {
			if DEBUG_ITER_COUNTER && !DEBUG_PUT_ERROR {
				print!("{}/({}..{}) total:{}\r"
					,number
					,use_imp_iter.first()
					,use_imp_iter.last() - 1
					,total_use.unwrap()
				)
			}
			if number >= total_use.unwrap() { eprintln!("Error: Index is over flow."); break;}

			// number -> select_sequence vecter [number of select]
			let mut num = number;
			let mut use_imp_indexes = vec![0;n_use_imp];

			let mut x = total_use.unwrap();
			for i in 0..n_use_imp {
				if i != 0 { num %= x }
				use_imp_indexes[i] = num / (x / (total_imp - i));
				x /= total_imp - i;
			}
			if DEBUG_NUMBER_SYSTM { println!("indexes:{:?}",use_imp_indexes) }

			let tmp = use_imp_indexes.clone();
			for i in 1..tmp.len() {
				for j in (0..i).rev() {
					if use_imp_indexes[i] >= tmp[j] { use_imp_indexes[i] += 1 }
				}
			}
			if DEBUG_NUMBER_SYSTM { println!("indexes:{:?}",use_imp_indexes) }

			let mut selected_imps = Vec::new();
			for use_imp_index in &use_imp_indexes {

				// reserve places and index for searching
				let mut u_index = *use_imp_index;
				let mut place: Option<usize> = None;
				for i in 0..n_imps.len() {
					if u_index >= n_imps[i] { u_index -= n_imps[i] }
					else { place = Some(i); break; }
				}
				if place == None { eprintln!("Error: Index is over flow. (patterns)") }
				if DEBUG_NUMBER_SYSTM { println!("place:{:?} -> {}",place,u_index) }

				// calc index for each machine
				let p = place.unwrap();
				let n = s_attr[p].len();
				let mut machine_selects: Vec<Option<usize>> = vec![None;n];
				for i in (0..n).rev() {
					if s_attr[p][i] {
						let t = self.pattern[i].len();
						machine_selects[i] = Some( u_index % t );
						u_index /= t;
					}
				}
				if DEBUG_NUMBER_SYSTM { println!("{:?}",machine_selects) }

				// create imposition from index
				let mut tmp_imps = Vec::new();
				for i in 0..machine_selects.len() {
					match machine_selects[i] {
						Some(ms) => {
							tmp_imps.push(self.pattern[i][ms].clone())
						},
						None => {
							tmp_imps.push(vec![0;self.pattern[0][0].len()])
						},
					}
				}
				if DEBUG_NUMBER_SYSTM { println!("{:?}",tmp_imps) }

				selected_imps.push(tmp_imps);
			}

			if DEBUG_NUMBER_SYSTM {
				println!("selected imps:");
				for tmp in &selected_imps { println!("{:?}",tmp) }
			}

			// add attribute
			let mut selected_imps_attr = Vec::new();
			for idx in 0..selected_imps.len() {
				let mut tmp = Vec::new();
				for v in &selected_imps[idx] {
					if v.iter().sum::<u8>() == 0 {
						tmp.push(Attribute::P(v.clone()));
					}
					else {
						tmp.push(Attribute::I(v.clone()));
					}
				}
				selected_imps_attr.push(tmp);
			}

			if DEBUG_NUMBER_SYSTM {
				println!("selected imps (add attribute):");
				for tmp in &selected_imps_attr { println!("{:?}",tmp) }
			}

			// fill vecter in attribute-P (printing)
			let mut tmp_v = vec![None;mlist.machine.len()];
			for i in 0..selected_imps_attr.len() {
				for j in 0..selected_imps_attr[i].len() {
					match &mut selected_imps_attr[i][j] {
						Attribute::I(v) => {
							tmp_v[j] = Some(v.clone());
						},
						Attribute::P(ref mut v) => {
							match &tmp_v[j] {
								Some(tv) => { *v = tv.clone() },
								None => {},
							}
						},
					}
				}
			}

			if DEBUG_NUMBER_SYSTM {
				println!("selected imps (fill vecter):");
				for tmp in &selected_imps_attr { println!("{:?}",tmp) }
			}

			// create problem: object function
			//[u = 0 + 3*x1 + 1*x2 + 2*x3 + 0*x4]
			let mut problem: Vec<Vec<f32>> = Vec::new();
			problem.push( vec![0.0; 1 + n_use_imp + plist.product.len()] );
			problem[0][0] = n_use_imp as f32;
			for i in 0..n_use_imp { problem[0][i + 1] = 1.0 }

			// create problem: conditions
			//[6 = 1*x1 + 2*x2 + 3*x3 + -1*x4]
			//[10 = 3*x1 + 2*x2 + 1*x3 + 1*x4]
			for i in 0..plist.product.len() {
				problem.push( vec![0.0; problem[0].len()] );
				problem[i + 1][0] = plist.product[i].num as f32;
			}
			for i in 0..selected_imps_attr.len() {
				for j in 0..selected_imps_attr[i].len() {
					let v_c = selected_imps_attr[i][j].extract_conditional();
					let v_nc = selected_imps_attr[i][j].extract();
					for k in 0..v_c.len() {
						problem[k + 1][0] -= mlist.machine[j].speed as f32 * v_c[k] as f32;
						problem[k + 1][i + 1] += mlist.machine[j].speed as f32 * v_nc[k] as f32;
					}
				}
			}
			// add slug-val
			for i in 0..plist.product.len() {
				problem[i + 1][1 + n_use_imp + i] = -1.0;
			}

			// calclate problem and reserve best result
			let mut bv = Vec::new();
			let result = two_phase_simplex_method(&mut problem, &mut bv);
			match result {
				Ok(_) => {
					match best {
						Some((ref mut prb,_,_)) => {
							if problem[0][0] < prb[0][0] {
								best = Some((problem.clone(),bv.clone(), selected_imps_attr))
							}
						},
						None => {
							best = Some((problem.clone(),bv.clone(), selected_imps_attr))
						},
					}
				},
				Err(err) => {
					if DEBUG_PUT_ERROR {
						match err {
							Error::NotExist(s) => { eprintln!("{} {}",number,s) }
							Error::FailedPhase(s) => { eprintln!("{} {}",number,s) }
							Error::NotNeed(s) => { eprintln!("{} {}",number,s) }
						}
					}
				},
			}
		}

		best
	}
	fn show(&self) {
		println!("*** Imposition Patterns ***");
		for i in 0..self.pattern.len() {
			println!("M{:<3}: {:?}", i, self.pattern[i]);
		}
		print!("\n");
	}
}

fn generate_combinations(target: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
	let mut result: Vec<Vec<Vec<u8>>> = Vec::new();
	let mut count = vec![0;target.len()];
	let mut r = 1;

	for i in 0..target.len() { r *= target[i].len() }

	for _ in 0..r {
		let mut tmp:Vec<Vec<u8>> = Vec::new();
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

	result
}

fn dp_enumerate(raider: usize, resource: usize) -> Vec<Vec<u8>> {

	// DP table
	let mut prev = vec![Vec::new(); resource + 1];
	let mut current = vec![Vec::new(); resource + 1];

	for i in 0..=resource {
		prev[i] = vec![vec![i as u8]];
	}

	// update DP table
	for _i in 2..=raider {
		for j in 0..=resource {
			let mut patterns = Vec::new();
			for k in 0..=j {
				for pattern in &prev[j - k] {
					let mut tmp = pattern.clone();
					tmp.push(k as u8);
					patterns.push(tmp);
				}
			}
			current[j] = patterns;
		}

		// swap DP table (shift)
		for tmp in &mut prev { tmp.clear() }
		std::mem::swap(&mut prev, &mut current);
	}

	prev[resource].clone()
}

// In the two-phase simplex method, it is necessary to create a artificial problem before the first application.
fn convert_artificial_problem_dict(prob: &mut Vec<Vec<f32>>) -> Vec<usize> {
	let mut bv = Vec::new();

	for i in 1..prob.len() {
		if prob[i][0] < 0.0 {
			for j in 0..prob[i].len() {
				prob[i][j] *= -1.0;
			}
		}
	}

	for i in 1..prob.len() {
		for j in 1..prob[i].len() {
			prob[i][j] *= -1.0;
		}
	}

	for i in prob[0].len()..(prob[0].len()+prob.len()-1) {
		bv.push(i);
		for j in 0..prob.len() {
			prob[j].push(0.0);
		}
	}
	for t in &mut prob[0] { *t = 0.0 }

	for i in 0..prob[0].len() {
		for j in 1..prob.len() {
			prob[0][i] += prob[j][i];
		}
	}

	bv
}

fn simplex_method(dict: &mut Vec<Vec<f32>>, bv: &mut Vec<usize>) -> Result<(), Error>{
	loop {
		let mut cnt = 0;

		for i in 1..dict[0].len() {
			if dict[0][i] < 0.0 {

				let mut ccnt = 0;
				let mut min: (f32,usize) = (0.0,0);
				for j in (1..dict.len()).rev() {
					if dict[j][i] < 0.0 {

						let tmp = dict[j][0] / dict[j][i].abs();
						if ccnt == 0 {
							min = (tmp,j);
						}
						else {
							if tmp < min.0 {
								min = (tmp,j);
							}
						}
						
						ccnt += 1;
					}
				}
				if ccnt == 0 || min.1 == 0 {
					return Err(Error::NotExist(format!("Error: Optimal solution does not exist.")));
				}

				//change process
				let tmp = -1.0 * dict[min.1][i];
				dict[min.1][i] = 0.0;
				dict[min.1][bv[min.1 - 1]] = -1.0;
				bv[min.1 - 1] = i;
				for eq in &mut dict[min.1] { *eq /= tmp }

				for k in 0..dict.len() {
					if k != min.1 {
						//substitution process
						let tmp = dict[k][i];
						dict[k][i] = 0.0;
						for l in 0..dict[k].len() { dict[k][l] += tmp * dict[min.1][l] }
					}
				}

				//sort
				let br = bv.remove(min.1 - 1);
				let dr = dict.remove(min.1);
				let mut ins = bv.len();
				for k in 0..bv.len() {
					if bv[k] > br { ins = k }
				}
				bv.insert(ins, br);
				dict.insert(ins + 1, dr);

				cnt += 1;
				break;
			}
		}

		if cnt == 0 { return Ok(()) }
	}
}

fn two_phase_simplex_method(dict: &mut Vec<Vec<f32>>, bv: &mut Vec<usize>) -> Result<(),Error> {
	let org_obj_fn = dict[0].clone();
	*bv = convert_artificial_problem_dict(dict);
	let org_bv = bv.clone();

	match simplex_method(dict, bv) {
		Ok(_) => {},
		Err(err) => { return Err(err) },
	}

	//zero judge and select remove not-basic-val
	let mut rm_nbv = Vec::new();
	for i in 0..dict[0].len() {
		if dict[0][i] > 0.1_f32.powf(ROUNDING_DIGIT) { rm_nbv.push(i) }
	}
	if org_bv != rm_nbv {
		return Err(Error::FailedPhase(format!("Error: The first step failed.")));
	}

	for i in (0..rm_nbv.len()).rev() {
		for k in 0..dict.len() { dict[k].remove(rm_nbv[i]); }
	}
	dict[0] = org_obj_fn;

	//substitution process
	for i in 0..bv.len() {
		let tmp = dict[0][bv[i]];
		dict[0][bv[i]] = 0.0;
		for j in 0..dict[0].len() {
			dict[0][j] += tmp * dict[i + 1][j];
		}
	}

	match simplex_method(dict, bv) {
		Ok(_) => {},
		Err(err) => { return Err(err) },
	}

	// slug val is not appropriate
	for i in bv {
		if *i > dict[0].len() - dict.len() {
			return Err(Error::NotNeed(format!("Error: The result is not what is needed.")));
		}
	}

	Ok(())
}

// for input/debug
fn show_dict(dict: &Vec<Vec<f32>>, bv: &Vec<usize>) {
	print!("min. u    = ");
	for i in 0..dict[0].len() {
		if i == 0 {
			print!("{:7.1} + ",dict[0][i]);
		}
		else if i == dict[0].len()-1 {
			print!("{:6.1}*x{:<3}",dict[0][i], i);
		}
		else {
			print!("{:6.1}*x{:<3} + ",dict[0][i], i);
		}
	}
	print!("\nsbj. ");
	for i in 1..dict.len() {
		if i != 1 { print!("     ") }
		print!("x{:<3} = ",bv[i-1]);

		for j in 0..dict[i].len() {
			if j == 0 {
				print!("{:7.1} + ",dict[i][j]);
			}
			else if j == dict[i].len()-1 {
				print!("{:6.1}*x{:<3}",dict[i][j], j);
			}
			else {
				print!("{:6.1}*x{:<3} + ",dict[i][j], j);
			}
		}

		print!("\n");
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
	plist.add(&flist, "A4", 4, 50000);
	plist.add(&flist, "A4", 4, 50000);
	plist.add(&flist, "A4", 4, 50000);
	plist.add(&flist, "A4", 4, 50000);
	plist.add(&flist, "A4", 4, 50000);
	plist.show();

	let mut mlist = Machines::new();
	mlist.add(&flist, "KK1", 4, 5000);
	mlist.add(&flist, "KK1", 4, 5000);
	mlist.show();

	let mut tally = Tally::new();
	tally.count(&flist, &plist);
	tally.show();

	let mut tess = Tessellations::new();
	tess.pack(&flist, &mlist, &tally, 10);
	tess.show(&mlist, &tally);

	let mut impo = Impositions::new();
	impo.calc(&tally, &tess);
	impo.show();

	let iter = Iter::new(0,10000000);
	let result = impo.search_optimal_solution(&plist, &mlist, 3, iter);

	if DEBUG_ITER_COUNTER && !DEBUG_PUT_ERROR { println!("") }

	if let Some((prb,bv,s_imps)) = result {
		println!("time: {}\nbv: {:?}\nselected impositions:",prb[0][0],bv);
		for i in 0..s_imps.len() {println!("{:?}",s_imps[i])}
		for i in 0..bv.len() {
			println!("{}: {}h",bv[i]-1, prb[i+1][0]);
		}
	}
	else {
		eprintln!("Not exist optimal solution...");
	}

}
