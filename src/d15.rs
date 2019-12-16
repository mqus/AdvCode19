use crate::intcode::Machine;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

#[derive(Clone)]
enum Direction {
	South,
	North,
	West,
	East,
}
const ALLDIR:[Direction;4]=[
Direction::South,
Direction::West,
Direction::East,
Direction::North,
];


impl Direction{

	pub fn to_input(&self) ->i64{
		match self{
			Direction::South => 1,
			Direction::North => 2,
			Direction::West => 3,
			Direction::East => 4,
		}
	}

	pub fn add_to_coords(&self, coord:&(i64,i64)) ->(i64,i64){
		match self{
			Direction::South => (coord.0,coord.1-1),
			Direction::North => (coord.0,coord.1+1),
			Direction::West => (coord.0-1,coord.1),
			Direction::East => (coord.0+1,coord.1),
		}
	}

	pub fn opposite(&self) ->Direction{
		match self{
			Direction::South => Direction::North,
			Direction::North => Direction::South,
			Direction::West => Direction::East,
			Direction::East => Direction::West,
		}
	}

}

const chars:[char;4]=['#',' ','X','?'];
pub fn run(){
	let found=walk();
	//paint_csv(&found);
	//paint(&found,None);
}


pub fn walk() -> HashMap<(i64,i64),u8>{
	use Direction::*;
	let mut m=Machine::from_stdin_allocate(1024);
	let mut current_pos=(0,0);
	let mut stack:Vec<((i64,i64),Direction)>=vec![];
	let mut visited=HashMap::new();
	visited.insert(current_pos.clone(),1);

	let mut i=m.into_iter();
	let mut iterations=0;
	let mut max_stack=0;
	loop{
		iterations+=1;
		let mut new=None;
		let mut next_dir=North;
		let mut should_push=true;
		'inner: for dir in ALLDIR.iter(){
			let n=dir.add_to_coords(&current_pos);
			if !visited.contains_key(&n){
				new=Some(n);
				next_dir=dir.clone();
				break 'inner;
			}
		}
		let next_pos=if let Some(x)=new {x} else {
			max_stack=max_stack.max(stack.len());
			if let Some((pos,dir))=stack.pop(){
				next_dir = dir.opposite();
				should_push=false;
				pos
			}else{
				println!("maxstack:{} || {} - 216 + 22 + 22 = {}",max_stack,max_stack,max_stack-216+22+22);
				return visited;
			}
		};
		i.push(next_dir.to_input());
		let t = i.next().expect("shouldnt stop here") as u8;
		visited.insert(next_pos.clone(),t);
		if t!=0{
			if should_push{
				stack.push((current_pos,next_dir));
			}
			current_pos=next_pos;
			if t==2{
				println!("FOUND! stacksize={}",stack.len())
			}
		}
		if iterations&0x0==0{//0x3ff
			//println!("iter:{}",iterations);
			//println!("stacklen:{}",stack.len());
			//paint(&visited, Some(current_pos.clone()));
			//std::thread::sleep(Duration::from_millis(30));
		}
	}


}
pub fn paint(visited:&HashMap<(i64,i64),u8>, pos:Option<(i64,i64)>){
	let (xmin,xmax,ymin,ymax) = visited.keys().fold((0,0,0,0),|(xn,xx,yn,yx),(x,y)| (
		xn.min(*x),
		xx.max(*x),
		yn.min(*y),
		yx.max(*y)));
	println!("-------------------------------------------");
	for y in ymin..=ymax{
		for x in xmin..=xmax{
			let c=if pos.filter(|(xp,yp)|*xp==x && *yp==y).is_some(){
				'D'
			} else if x==0 && y==0{
				'@'
			}else{
				let t=*visited.get(&(x,y)).unwrap_or(&3);
				chars[t as usize]
			};
			print!("{}",c);
		}
		println!();
	}
	println!("-------------------------------------------");
}

fn paint_csv(visited:&HashMap<(i64,i64),u8>){
	let (xmin,xmax,ymin,ymax) = visited.keys().fold((0,0,0,0),|(xn,xx,yn,yx),(x,y)| (
		xn.min(*x),
		xx.max(*x),
		yn.min(*y),
		yx.max(*y)));
	for y in (ymin..=ymax).rev(){
		for x in xmin..=xmax{
			let c=if x==0 && y==0{
				4
			}else{
				*visited.get(&(x,y)).unwrap_or(&3)
			};
			print!(",{}",c);
		}
		println!();
	}
}
