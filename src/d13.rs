use crate::intcode::{Machine, Integer};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

type Type=i64;

pub fn run(){
	let mut m=Machine::from_stdin_allocate(1024);

	let mut outputs=m.into_iter();
	#[cfg(feature="d13_1")]{
		let mut map = HashMap::new();
		while let Some(x)=outputs.next(){
			let y=outputs.next().unwrap();
			let t:Type=outputs.next().unwrap().into();
			map.insert((x,y),t);
			println!("{}\t{}",x,y);
		}
		let blockcount=map.iter().filter(|(_,&t)|t==2).count();
		println!("blocks:{}",blockcount)
	}
	#[cfg(not(feature="d13_1"))]{
		outputs.m.set_mem(0,2);
		let mut map = [[0;40];24];
		let mut score=0;
		let mut paddle_x:Integer=0;
		let mut ball_x:Integer=0;
		let mut c=0;
		while let Some(x)=outputs.next(){
			if x<0{
				outputs.next();
				score=outputs.next().expect("value should contain score!!");
				continue;
			}
			let y=outputs.next().unwrap() as usize;
			let t=outputs.next().unwrap() as u8;
			if t==3{
				paddle_x=x;
			}
			if t==4{
				ball_x=x;
				outputs.push((ball_x-paddle_x).signum());
			}
			map[y][x as usize]=t;
			c+=1;
			//print(&map,&score);
		}

		print(&map,&score);
		println!("c:{} instr_c:{}", c, outputs.m.instruction_counter);

	}
}


fn print(map:&[[u8;40];24], score:&Integer){
	let chars=&[' ','W','#','=','o'];
	println!("----------------------------------------");
	for line in map.iter(){
		for &square in line.iter(){
			print!("{}",chars[square as usize]);
		}
		println!()
	}
	println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
	println!("  Score:\t{}",score);
	println!("========================================");
	std::thread::sleep(Duration::from_millis(5));
}
