use std::io::{BufReader, stdin, BufRead};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::mem::swap;

type Single=i32;

#[cfg(feature ="d12_1")]
type Triple=(Single, Single, Single);
#[cfg(feature ="d12_1")]
pub fn run(){
	let mut pos =[(0, 0, 0);4];
	let mut vel =[(0, 0, 0);4];
	for (i,l) in BufReader::new(stdin()).lines().enumerate().take(4){
		pos[i]= parse_coords(l.unwrap());
	}
	for _ in 0..1000{
		//calculate velocity
		for i in 0..4{
			for j in i+1..4{
				let d_vel_j=map2(&pos[i], &pos[j], |x, y|(*x-*y).signum());
				vel[j] = map2(&vel[j], &d_vel_j, |a, b|*a+*b);
				vel[i] = map2(&vel[i], &d_vel_j, |a, b|*a-*b);
			}
		}
		//calculate position
		for i in 0..4{
			pos[i] = map2(&pos[i],&vel[i],|a,b|a+b);
		}
	}
	println!("pos:{:?}",pos);
	println!("vel:{:?}",vel);
	let e:Single=pos.iter()
			.zip(vel.iter())
			.map(|(a,b)|sum(a)*sum(b))
			.sum();
	println!("sum:{}",e);

	//calculate energy


}
#[cfg(not(feature ="d12_1"))]
type Triple=[Single;3];

#[cfg(feature ="d12_2")]
pub fn run(){
	let mut pos_all =[[0, 0, 0];4];
	//let mut seen=[HashMap::new(),HashMap::new(),HashMap::new()];
	let mut loops=vec![];
	for (i,l) in BufReader::new(stdin()).lines().enumerate().take(4){
		pos_all[i]= parse_coords(l.unwrap());
	}

	//for each dimension
	'outer:for dim in 0..3{
		let mut seen = HashMap::new();
		//convert:
		let mut vel =[0;4];
		let mut pos=[
			pos_all[0][dim],
			pos_all[1][dim],
			pos_all[2][dim],
			pos_all[3][dim],
		];
		for i in 0u64..{
			if let Some(&first_occurrence)=seen.get(&(pos,vel)){
				loops.push((first_occurrence,i-first_occurrence));
				continue 'outer;
			}else{
				seen.insert((pos,vel).clone(),i);
			}


			//calculate velocity
			for i in 0..4{
				for j in i+1..4{
					let d_vel_j=(&pos[i]-&pos[j]).signum();
					vel[j] += d_vel_j;
					vel[i] -= d_vel_j;
				}
			}
			//calculate position
			for i in 0..4{
				pos[i]+= vel[i];
			}

		}
	}
	println!("loops:{:?}",loops);
	println!("lcm:{}",lcm3(loops[0].1,loops[1].1,loops[2].1))



}

fn lcm3(a:u64,b:u64,c:u64) ->u64{
	lcm2(lcm2(a,b),c)
}


fn lcm2(a:u64,b:u64) ->u64{
	a/dbg!(gcd(a,b))*b
}

fn gcd(mut a:u64,mut b:u64)->u64 {
	if b>a { swap(&mut b,&mut a) }
	if b==0{
		a
	}else{
		gcd(a-b,b)
	}
}

fn parse_coords(line:String) ->Triple{
	let to_remove=&['x','y','z','=','<','>',' '];
	let l2=line.replace(|c|to_remove.contains(&c),"");
	let mut numbers=l2.split(',').map(str::parse::<Single>).map(Result::unwrap);
	#[cfg(feature ="d12_1")]
	return (
	 numbers.next().unwrap(),
	 numbers.next().unwrap(),
	 numbers.next().unwrap(),
	);

	#[cfg(not(feature ="d12_1"))]
	return [
	numbers.next().unwrap(),
	numbers.next().unwrap(),
	numbers.next().unwrap(),
	];
}

#[cfg(feature ="d12_1")]
fn sum(t:&Triple)->Single{
	t.0.abs()+t.1.abs()+t.2.abs()
}
#[cfg(feature ="d12_1")]
fn map<T:Fn(&Single)->Single>(inp:&Triple,f:T)->Triple{
	(f(&inp.0), f(&inp.1), f(&inp.2))
}
#[cfg(feature ="d12_1")]
fn map2<T:Fn(&Single,&Single)->Single>(i1:&Triple,i2:&Triple,f:T)->Triple{
	(f(&i1.0,&i2.0), f(&i1.1,&i2.1), f(&i1.2,&i2.2))
}
