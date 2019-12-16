use std::io::{BufReader, stdin, BufRead};
use std::collections::{HashSet, HashMap};
use std::ops::Div;

pub fn run(){
	let mut map = HashMap::new();

	for l in BufReader::new(stdin()).lines(){
		let x=parse_formula(l.unwrap());



		if let Some(y)=map.insert((x.1).1,((x.1).0,x.0)){
			println!("double: {:?}",y);
		}
	}


	let mut leftover=HashMap::new();
	let ore=produce(&mut map,&mut leftover,"FUEL",1);
	println!("ore:{}",ore);

	const ore_left_over:u64=1_000_000_000_000;
	let mut fuel_min=dbg!(ore_left_over/ore);
	let mut fuel_max = 2*fuel_min;
	leftover.clear();
	assert!(produce(&map,&mut leftover,"FUEL",fuel_min)<ore_left_over);

	//go up
	loop {
		leftover.clear();
		if produce(&map,&mut leftover,"FUEL",fuel_max)>ore_left_over{
			break;
		}
		fuel_min=dbg!(fuel_max);
		fuel_max=2*fuel_min;
	}

	//go inside
	while fuel_max-fuel_min>1{
		let fuel_test=dbg!((fuel_max-fuel_min)/2+fuel_min);

		leftover.clear();
		let ore_used=produce(&map,&mut leftover,"FUEL",fuel_test);
		if dbg!(ore_used) == ore_left_over{
			println!("fuel from {}:{}",ore_left_over,fuel_test);
			break;
		} else if ore_used > ore_left_over{
			fuel_max = fuel_test;
		} else {
			fuel_min = fuel_test;
		}
	}
	println!("fuel_min:{}",fuel_min);
	println!("fuel_max:{}",fuel_max);



	let mut fuel_test =fuel_max;



	println!("leftover:{:?}",leftover);




}


fn parse_formula(s:String) -> (Vec<(u64,String)>,(u64,String)){
	let mut s1=s.split(" => ");
	let from=s1.next().unwrap();
	let mut to_it=s1.next().unwrap().split(' ');
	let from_vec:Vec<_>=from.split(", ").map(|s|{
		let mut is=s.split(' ');
		(is.next().unwrap().parse::<u64>().unwrap(),is.next().unwrap().to_string())
	}).collect();
	let to=(to_it.next().unwrap().parse::<u64>().unwrap(), to_it.next().unwrap().to_string());
	(from_vec,to)
}

fn produce(formulas:&HashMap<String,(u64,Vec<(u64,String)>)>,leftover:&mut HashMap<String,u64>,resource:&str,mut amount_needed:u64)->u64{
	if resource == "ORE"{
		return amount_needed;
	}


	let formula=formulas.get(resource).unwrap();

	let mut amount_left_over = *leftover.get(resource).unwrap_or(&0);
	let amount_per_formula = formula.0;

	if amount_needed<=amount_left_over{
		leftover.insert(resource.to_string(),amount_left_over-amount_needed);
		return 0;
	} else if amount_left_over>0{
		amount_needed-=amount_left_over;
	}
	//by now, nothing is left over.

	//round up amount_needed/amount_per_formula
	let count = if amount_needed%amount_per_formula == 0 {
		amount_needed/amount_per_formula
	} else {
		amount_needed/amount_per_formula+1
	};
	let mut ore_needed=0;

	for (i_amount,ingredient) in &formula.1{
		ore_needed+=produce(formulas,leftover,ingredient,i_amount*count);
	}




	leftover.insert(resource.to_string(),amount_per_formula*count-amount_needed);
	ore_needed

}
