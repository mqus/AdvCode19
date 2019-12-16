use crate::intcode::Machine;
use std::collections::{HashMap, VecDeque};

type Point=(i8,i8);
const DEFAULT:i64=0;
pub fn run(){
	let directions=[
		(0,1),
		(1,0),
		(0,-1),
		(-1,0),
	];
	let mut current_dir=0;
	let mut coloured=HashMap::new();
	let mut p:Point=(0,0);
	let mut input=VecDeque::new();
	let mut m=Machine::from_stdin_allocate(1024);
	#[cfg(feature ="d11_1")]
	input.push_back(DEFAULT);
	#[cfg(feature ="d11_2")]
	input.push_back(1);
	while let Some(color)=m.run_to_output(&mut input){
		coloured.insert(p,color);
		let rotate=m.run_to_output(&mut input).unwrap();
		current_dir=(current_dir+(rotate*2-1)+4)%4;
		p=(p.0+directions[current_dir as usize].0,
		   p.1+directions[current_dir as usize].1);
		input.push_back(*coloured.get(&p).unwrap_or(&DEFAULT))
	}
	println!("len:{}", coloured.len());
	let xmax=coloured.keys().map(|p|p.0).max().unwrap();
	let xmin=coloured.keys().map(|p|p.0).min().unwrap();
	let ymax=coloured.keys().map(|p|p.1).max().unwrap();
	let ymin=coloured.keys().map(|p|p.1).min().unwrap();
	println!("x:{} / {}",xmin,xmax);
	println!("y:{} / {}",ymin,ymax);
	let mut field=[[0u8;100];100];
	coloured.iter().for_each(|((x,y),c)|{
		field[(-*y+50) as usize][(*x+50) as usize]=*c as u8;
	});
	for line in field.iter(){
		for square in line.iter(){
			print!("{}",if *square==1{"#"}else{" "});
		}
		println!();
	}
}
