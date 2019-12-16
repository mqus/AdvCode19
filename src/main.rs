use core::fmt;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader, Read, stdin};
use std::mem::swap;
use std::ops::Range;
use std::str::FromStr;

use crate::intcode::Machine;

mod intcode;

#[cfg(feature = "d1_1")]
fn main() -> Result<(), Box<dyn Error>> {
	let mut sum = 0;
	for l in BufReader::new(stdin()).lines() {
		let mass = i64::from_str((l?).as_str())?;
		let fuel = mass / 3 - 2;
		sum += fuel;
		println!("{}", fuel);
	}
	println!("-----------------\n{}", sum);

	Ok(())
}

#[cfg(feature = "d1_2")]
fn main() -> Result<(), Box<dyn Error>> {
	let mut sum = 0;
	for l in BufReader::new(stdin()).lines() {
		let mass = i64::from_str((l?).as_str())?;
		let fuel = get_fuel_mass(mass);

		sum += fuel;
		println!("{}", fuel);
	}
	println!("-----------------\n{}", sum);

	Ok(())
}

fn get_fuel_mass(mass: i64) -> i64 {
	let fuel = mass / 3 - 2;
	if fuel <= 0 {
		0
	} else {
		fuel + get_fuel_mass(fuel)
	}
}

#[cfg(feature = "d3_1")]
fn main() {
	let mut current = (0i32, 0i32);
	//read
	let mut points = HashSet::new();
	let mut input = BufReader::new(stdin()).lines();
	let line_1 = input.next().unwrap().unwrap();
	for step in line_1.split(',') {
		let (dir, dists) = step.split_at(1);
		let (dx, dy) = match dir {
			"L" => (-1, 0),
			"R" => (1, 0),
			"U" => (0, 1),
			"D" => (0, -1),
			_ => unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap() {
			current.0 += dx;
			current.1 += dy;
			points.insert(current.clone());
		}
	}
	//test
	current = (0, 0);
	let mut conflicts = BinaryHeap::new();
	let line_2 = input.next().unwrap().unwrap();
	for step in line_2.split(',') {
		let (dir, dists) = step.split_at(1);
		let (dx, dy) = match dir {
			"L" => (-1, 0),
			"R" => (1, 0),
			"U" => (0, 1),
			"D" => (0, -1),
			_ => unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap() {
			current.0 += dx;
			current.1 += dy;
			if points.contains(&current) {
				conflicts.push(Reverse(current.0.abs() + current.1.abs()))
			}
		}
	}
	println!("{:?}", conflicts);
}

#[cfg(feature = "d3_2")]
fn main() {
	let mut current = (0i32, 0i32);
	//read
	let mut points = HashMap::new();
	let mut input = BufReader::new(stdin()).lines();
	let line_1 = input.next().unwrap().unwrap();
	let mut i = 0i32;
	for step in line_1.split(',') {
		let (dir, dists) = step.split_at(1);
		let (dx, dy) = match dir {
			"L" => (-1, 0),
			"R" => (1, 0),
			"U" => (0, 1),
			"D" => (0, -1),
			_ => unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap() {
			i += 1;
			current.0 += dx;
			current.1 += dy;
			points.insert(current.clone(), i);
		}
	}
	//test
	current = (0, 0);
	let mut conflicts = BinaryHeap::new();
	let line_2 = input.next().unwrap().unwrap();
	let mut i2 = 0;
	for step in line_2.split(',') {
		let (dir, dists) = step.split_at(1);
		let (dx, dy) = match dir {
			"L" => (-1, 0),
			"R" => (1, 0),
			"U" => (0, 1),
			"D" => (0, -1),
			_ => unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap() {
			i2 += 1;
			current.0 += dx;
			current.1 += dy;
			if let Some(dist) = points.get(&current) {
				conflicts.push(Reverse(dist + i2))
			}
		}
	}
	println!("{:?}", conflicts);
}

#[cfg(feature = "d3_1_false")]
fn main() {
	let mut first = true;
	//read
	let mut lines = BTreeSet::new();
	for l in stdin().lock().lines() {
		let (mut current_x, mut current_y) = (0, 0);
		let s = l.unwrap();
		for a in s.split(',') {
			let (dir, dists) = a.split_at(1);
			let dist: i64 = dists.parse().unwrap();
			let line = match dir {
				"L" => {
					let tmp = current_x;
					current_x -= dist;
					Line::Horz {
						x1: current_x,
						x2: tmp,
						y: current_y,
						first,
					}
				}
				"R" => {
					let tmp = current_x;
					current_x += dist;
					Line::Horz {
						x1: tmp,
						x2: current_x,
						y: current_y,
						first,
					}
				}
				"U" => {
					let tmp = current_y;
					current_y += dist;
					Line::Vert {
						x: current_x,
						y1: tmp,
						y2: current_y,
						first,
					}
				}
				"D" => {
					let tmp = current_y;
					current_y -= dist;
					Line::Vert {
						x: current_x,
						y1: current_y,
						y2: tmp,
						first,
					}
				}
				_ => unreachable!(),
			};
			lines.insert(line);
		}
		first = false;
	}

	let mut mindist = core::i64::MAX;
	let hfilter = |line: &Line|
			if let Line::Horz { x1, x2, y, first } = line {
				Some((*x1, *x2, *y, *first))
			} else {
				None
			};
	let vfilter = |line: &Line|
			if let Line::Vert { x, y1, y2, first } = line {
				Some((*x, *y1, *y2, *first))
			} else {
				None
			};
	println!("linecount:{:?}", lines.iter().collect::<Vec<_>>());
	let min = lines.iter()
			.filter_map(hfilter)
			.map(|(hx1, hx2, hy, hfirst)| {
				lines.iter()
						.filter_map(vfilter)
						.filter_map(|(vx, vy1, vy2, vfirst)| {
							//println!("point:{} vs {}",hfirst,vfirst);
							if vx >= hx1 &&
									vx <= hx2 &&
									vy1 <= hy &&
									vy2 >= hy &&
									vfirst == hfirst &&
									(vx != 0 || hy != 0) {
								println!("one");
								Some((vx, hy))
							} else {
								None
							}
						})
						.map(|(x, y)| x.abs() + y.abs())
						.collect::<Vec<_>>()
//                        .unwrap_or(core::i64::MAX)
			})
			.flatten()
			.collect::<Vec<_>>();
	//.unwrap();
	println!("MIN:{:?}", min)
}

#[derive(PartialEq, Eq, Debug)]
enum Line {
	Horz { x1: i64, x2: i64, y: i64, first: bool },
	Vert { x: i64, y1: i64, y2: i64, first: bool },
}

impl PartialOrd for Line {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Line {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Line::Horz { x1, .. }, Line::Horz { x1: x_2, .. }) => x1.cmp(x_2),
			(Line::Horz { x1, .. }, Line::Vert { x: x_2, .. }) => x1.cmp(x_2),
			(Line::Vert { x: x_1, .. }, Line::Horz { x1: x_2, .. }) => x_1.cmp(x_2),
			(Line::Vert { x: x_1, .. }, Line::Vert { x: x_2, .. }) => x_1.cmp(x_2),
		}
	}
}


#[cfg(feature = "d4_1")]
fn main() {
	let min = 147981;
	let max = 691423;
	let (count, x) = count1(0, -1, min, max, false, 6);
	println!("----\ncount:{}:{}", count, x);
}

fn count1(prefix: u32, lastd: i8, min: u32, max: u32, includes_double: bool, digits_left: u8) -> (u32, bool) {
	if digits_left == 0 {
		if !includes_double {
			return (0, false)
		}
		match prefix {
			i if i < min => (0, false),
			i if i > max => (0, true),
			_ => {
				println!("{}", prefix);
				(1, false)
			}
		}
	} else {
		let start = lastd.max(0);
		let mut count = 0;
		for i in start..=9i8 {
			let (c, should_stop) = count1(prefix * 10 + i as u32, i, min, max, includes_double || i == lastd, digits_left - 1);
			count += c;
			if should_stop {
				return (count, true)
			}
		}
		(count, false)
	}
}

#[cfg(feature = "d4_2")]
fn main() {
	let min = 147981;
	let max = 691423;
	let (count, x) = count2(0, -1, min, max, false, 6, 0);
	println!("----\ncount:{}:{}", count, x);
}

fn count2(prefix: u32, lastd: i8, min: u32, max: u32, includes_double: bool, digits_left: u8, current_run: u8) -> (u32, bool) {
	if digits_left == 0 {
		if !includes_double && current_run != 2 {
			return (0, false)
		}
		match prefix {
			i if i < min => (0, false),
			i if i > max => (0, true),
			_ => {
				println!("{}", prefix);
				(1, false)
			}
		}
	} else {
		let start = lastd.max(0);
		let mut count = 0;
		for i in start..=9i8 {
			let mut includes_double2 = includes_double;
			let mut current_run2 = 1;
			if lastd == i {
				current_run2 = current_run + 1
			}
			if current_run == 2 && lastd != i {
				includes_double2 = true;
			}
			let (c, should_stop) = count2(prefix * 10 + i as u32, i, min, max, includes_double2, digits_left - 1, current_run2);
			count += c;
			if should_stop {
				return (count, true)
			}
		}
		(count, false)
	}
}

#[cfg(any(feature = "d6_1", feature = "d6_2"))]
fn main() {
	let mut map: HashMap<String, Vec<String>> = HashMap::new();
	let mut map_back = HashMap::new();
	for l in BufReader::new(stdin()).lines() {
		let s = l.unwrap();
		let mut x = s.split(')');
		let a = x.next().unwrap().to_string();
		let b = x.next().unwrap();
		map.entry(a.clone())
				.and_modify(|v| v.push(b.to_string()))
				.or_insert_with(|| vec![b.to_string()]);
		map_back.insert(b.to_string(), a.to_string());
	}

	#[cfg(feature = "d6_1")] {//make a DFS, add the depths at each node (depth = # of indirect&direct orbits)
		let mut sum = 0;
		let mut stack = vec![("COM".to_string(), 0)];
		let empty_vec = vec![];
		while let Some((a, depth)) = stack.pop() {
			sum += depth;
			for b in map.get(&a).unwrap_or(&empty_vec) {
				stack.push((b.clone(), depth + 1));
			}
		}
		println!("{}", sum);
	}

	#[cfg(feature = "d6_2")] {//make a BFS
		let mut queue = VecDeque::from(vec![("YOU".to_string(), 0)]);
		let empty_vec = vec![];
		let mut found = HashSet::new();
		while let Some((a, depth)) = queue.pop_front() {
			if found.contains(&a) { continue }
			found.insert(a.clone());
			//println!("{}:{}",&a,depth);
			if &a == "SAN" {
				println!("---\nDIST:{}", depth - 2)
			}
			if let Some(parent) = map_back.get(&a) {
				queue.push_back((parent.clone(), depth + 1))
			}
			for b in map.get(&a).unwrap_or(&empty_vec) {
				queue.push_back((b.clone(), depth + 1));
			}
		}
	}
}

#[cfg(any(feature = "d10_1", feature = "d10_2"))]
fn main() {
	let mut asteroid_locations = HashSet::new();
	let mut ggt_memo = HashMap::new();
	let mut ggt_memoized = move |(a, b)| ggt_memo.entry((a, b)).or_insert_with(|| ggT(a, b)).clone();


	for (i, l) in BufReader::new(stdin()).split(b'\n').enumerate() {
		l.unwrap().into_iter()
				//.map(|s|dbg!(s))
				.enumerate()
				.filter(|(i, b)| *b == b'#')
				.map(|(j, _)| (i as i8, j as i8))
				.for_each(|p| { asteroid_locations.insert(p); });
	}
	let mut max = ((0, 0), 0);
	asteroid_locations.iter()
			.map(|a| {
				let s = asteroid_locations.iter()
						.filter(|b| {
							let d = p_minus(b, a);
							let ggt = ggt_memoized(p_abs(&d));
							(1..ggt).all(|i| {
								let p = p_plusmul(a, &d, i, ggt);
								!asteroid_locations.contains(&p)
							}) && (a.1 != b.1 || a.0 != b.0)
						}).count();

				println!("{}", s);
				(a.clone(), s)
			})
			.for_each(|p| if p.1 > max.1 { max = p });
	println!("{}/{}", max.1, asteroid_locations.len());
	#[cfg(feature = "d10_2")] {
		let a = &max.0;
		//get all 1-dist points, calculate
		let s: BTreeMap<_, _> = asteroid_locations.iter()
				.filter(|b| {
					let d = p_minus(b, a);
					let ggt = ggt_memoized(p_abs(&d));
					(1..ggt).all(|i| {
						let p = p_plusmul(a, &d, i, ggt);
						!asteroid_locations.contains(&p)
					}) && (a.1 != b.1 || a.0 != b.0)
				})
				.map(|p| ((p_angle(a, p) * 1000.) as i16, p.clone()))
				.collect();

		let t200 = s.iter().skip(199).next().unwrap();
		println!("thing:{:?} to {:?}", a, t200)
	}
}

fn p_angle(base: &(i8, i8), target: &(i8, i8)) -> f32 {
	let d = p_minus(target, base);
	let (x, y) = (f32::from(d.0), f32::from(d.1));
	std::f32::consts::PI - y.atan2(x)
}

#[test]
fn test_p_angle() {
	macro_rules! test_eps {
		($a:expr,$b:expr) => {
			assert!( (($a)-($b)).abs()<=std::f32::EPSILON,"{} != {}",$a,$b)
		}
	}
	use std::f32::consts::PI;
	let base = (0, 0);
	let p1 = (1, 0);
	let p2 = (-1, 0);
	let p3 = (0, 1);
	let p4 = (0, -1);
	test_eps!(p_angle(&base,&p1),PI);
	test_eps!(p_angle(&base,&p2),0.);
	test_eps!(p_angle(&base,&p3),PI/2.);
	test_eps!(p_angle(&base,&p4),PI/2.*3.);
}


fn p_minus(a: &(i8, i8), b: &(i8, i8)) -> (i8, i8) {
	(a.0 - b.0, a.1 - b.1)
}

fn p_plusmul(a: &(i8, i8), b: &(i8, i8), fac: i8, div: i8) -> (i8, i8) {
	(a.0 + b.0 / div * fac, a.1 + b.1 / div * fac)
}

fn p_abs(a: &(i8, i8)) -> (i8, i8) {
	(a.0.abs(), a.1.abs())
}

fn ggT(mut a: i8, mut b: i8) -> i8 {
	if b > a { swap(&mut b, &mut a) }
	if b == 0 {
		a
	} else {
		ggT(a - b, b)
	}
}


#[cfg(any(feature = "d8_1", feature = "d8_2"))]
fn main() {
	const width: usize = 25;
	const height: usize = 6;
	let mut layers = vec![];
	let mut reader = BufReader::new(stdin());
	loop {
		let mut tmp = [0u8; width * height];
		let res = reader.read_exact(&mut tmp);
		if res.is_err() {
			break;
		}
		layers.push(tmp);
	}
	#[cfg(feature = "d8_1")] {
		let zero_max: BTreeMap<_, _> = layers.iter()
				.enumerate()
				.map(|(i, bytes)|
						(dbg!(bytes.iter()
								.filter(|&b| *b == b'0')
								.count()),
						 i)
				)
				.collect();
		let l = &layers[*zero_max.iter().next().unwrap().1];
		let twos = l.iter().filter(|&b| *b == b'2').count();
		let ones = l.iter().filter(|&b| *b == b'1').count();
		println!("{}*{}={}", ones, twos, ones * twos);
	}
	#[cfg(feature = "d8_2")] {
		let mut final_image = [b'x'; width * height];
		for layer in &layers {
			for i in 0..width * height {
				if final_image[i] == b'x' {
					final_image[i] = match layer[i] {
						b'0' => b' ',
						b'1' => b'#',
						b'2' => b'x',
						_ => unreachable!()
					}
				}
			}
		}
		for i in 0..height {
			println!("{}", std::str::from_utf8(&final_image[i * width..(i + 1) * width]).unwrap());
		}
	}
}


#[cfg(feature = "d2_1")]
fn main() {
	let mut m = Machine::from_stdin();
	m.set_mem(1, 12);
	m.set_mem(2, 2);
	m.run_once(VecDeque::new());
	println!("memory at [0]:{}", m.get_mem(0));
}

#[cfg(feature = "d2_2")]
fn main() {
	let m = Machine::from_stdin();
	for noun in 0..100 {
		for verb in 0..100 {
			let mut m2 = m.clone();
			m2.set_mem(1, noun);
			m2.set_mem(2, verb);
			m2.run_once(VecDeque::new());
			println!("{} {}\t:{}", noun, verb, m2.get_mem(0));
			if m2.get_mem(0) == 19690720 {
				println!("---------------------------\n FOUND!");
				return;
			}
		}
	}
	println!("NOTHING FOUND!!!");
}


#[cfg(any(feature = "d5_1", feature = "d5_2"))]
fn main() {
	let mut input = VecDeque::new();
	let mut m = Machine::from_stdin();
	//println!("program: {:?}",program);
	#[cfg(feature = "d5_1")]
			input.push_back(1);
	#[cfg(feature = "d5_2")]
			input.push_back(5);

	while let Some(out) = m.run_to_output(&mut input) {
		println!("OUT:{}", out);
	}
}

#[cfg(feature = "d7_1")]
fn main() -> Result<(), Box<dyn Error>> {
	let mut m = Machine::from_stdin();
	let (mut max_phase_out, mut max_phase_in) = (-1, [-1; 5]);
	for p1 in 0..=4 {
		let i1 = VecDeque::from(vec![p1, 0]);
		let o1 = m.clone().run_once(i1);
		for p2 in 0..=4 {
			if p2 == p1 { continue }
			let i2 = VecDeque::from(vec![p2, o1]);
			let o2 = m.clone().run_once(i2);
			for p3 in 0..=4 {
				if p3 == p1 || p3 == p2 { continue }
				let i3 = VecDeque::from(vec![p3, o2]);
				let o3 = m.clone().run_once(i3);
				for p4 in 0..=4 {
					if p4 == p1 || p4 == p2 || p4 == p3 { continue }
					let i4 = VecDeque::from(vec![p4, o3]);
					let o4 = m.clone().run_once(i4);
					for p5 in 0..=4 {
						if p5 == p1 || p5 == p2 || p5 == p3 || p5 == p4 { continue }
						let i5 = VecDeque::from(vec![p5, o4]);
						let o5 = m.clone().run_once(i5);
						if o5 > max_phase_out {
							max_phase_out = o5;
							max_phase_in = [p1, p2, p3, p4, p5];
						}
					}
				}
			}
		}
	}
	println!();

	println!("{}   (from {:?})", max_phase_out, max_phase_in);
	Ok(())
}

#[cfg(feature = "d7_2")]
fn main() -> Result<(), Box<dyn Error>> {
	let mut program = Machine::from_stdin();
	let (mut max_phase_out, mut max_phase_in) = (-1, [0; 5]);
	let mut phases = [5, 6, 7, 8, 9];
	for i1 in 0..5 {
		phases.swap(0, i1);
		for i2 in 1..5 {
			phases.swap(1, i2);
			for i3 in 2..5 {
				phases.swap(2, i3);
				for i4 in 3..5 {
					phases.swap(3, i4);
					let o = amps(&program, &phases);

					if o > max_phase_out {
						max_phase_out = o;
						max_phase_in = phases.clone();
					}
					phases.swap(3, i4);
				}

				phases.swap(2, i3);
			}

			phases.swap(1, i2);
		}

		phases.swap(0, i1);
	}
	println!();

	println!("{}   (from {:?})", max_phase_out, max_phase_in);
	Ok(())
}

fn amps(m: &Machine, phases: &[u8; 5]) -> i64 {
	let mut progs = [
		m.clone(),
		m.clone(),
		m.clone(),
		m.clone(),
		m.clone(),
	];
	let mut inputs: [VecDeque<_>; 5] = [
		vec![phases[0] as i64, 0].into(),
		vec![phases[1] as i64].into(),
		vec![phases[2] as i64].into(),
		vec![phases[3] as i64].into(),
		vec![phases[4] as i64].into(),
	];
	let mut i = 0;
	let mut loops = 0; //debug
	loop {
		let output = progs[i].run_to_output(&mut inputs[i]);
		if output.is_none() {
			println!("{:?}:\t{}\t{}", phases, inputs[i].front().unwrap(), loops);
			break inputs[i].pop_front().unwrap();
		}
		i = (i + 1) % 5;
		inputs[i].push_back(output.unwrap());
		loops += 1;
	}
}

#[cfg(any(feature = "d9_1", feature = "d9_2"))]
fn main() {
	use intcode::Machine;
	let mut m = Machine::from_stdin_allocate(512);
	println!("alloc!");
	let mut inp = VecDeque::new();
	#[cfg(feature = "d9_1")]
			inp.push_back(1);
	#[cfg(feature = "d9_2")]
			inp.push_back(2);
	while let Some(output) = m.run_to_output(&mut inp) {
		println!("output:{}", output);
	}
	println!("ins_count:{}", m.instruction_counter);
}

mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;

#[cfg(any(feature = "d11_1", feature = "d11_2",
feature = "d12_1", feature = "d12_2",
feature = "d13_1", feature = "d13_2",
feature = "d14_1", feature = "d14_2",
feature = "d16_1", feature = "d16_2",
feature = "d15_1", feature = "d15_2"))]
fn main() {
	#[cfg(any(feature = "d11_1", feature = "d11_2"))]
			d11::run();
	#[cfg(any(feature = "d12_1", feature = "d12_2"))]
			d12::run();
	#[cfg(any(feature = "d13_1", feature = "d13_2"))]
			d13::run();
	#[cfg(any(feature = "d14_1", feature = "d14_2"))]
			d14::run();
	#[cfg(any(feature = "d15_1", feature = "d15_2"))]
			d15::run();
	#[cfg(any(feature = "d16_1", feature = "d16_2"))]
			d16::run();
}
