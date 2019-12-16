use std::error::Error;
use std::io::{BufReader, stdin, BufRead};
use std::str::FromStr;
use std::fmt::{Display, Debug, Formatter};
use core::fmt;
use std::hint::unreachable_unchecked;
use std::collections::{VecDeque, HashMap, HashSet, BTreeSet, BinaryHeap, BTreeMap};
use std::cmp::{Ordering, Reverse};
use std::ops::Range;
use std::mem::swap;

mod intcode;

#[cfg(feature = "d1_1")]
fn main()->Result<(),Box<dyn Error>> {
    let mut sum =0;
    for l in BufReader::new(stdin()).lines(){
        let mass=i64::from_str((l?).as_str())?;
        let fuel=mass/3-2;
        sum+=fuel;
        println!("{}",fuel);
    }
    println!("-----------------\n{}",sum);

    Ok(())
}

#[cfg(feature = "d1_2")]
fn main()->Result<(),Box<dyn Error>> {
    let mut sum =0;
    for l in BufReader::new(stdin()).lines(){
        let mass=i64::from_str((l?).as_str())?;
        let fuel=get_fuel_mass(mass);

        sum+=fuel;
        println!("{}",fuel);
    }
    println!("-----------------\n{}",sum);

    Ok(())
}

fn get_fuel_mass(mass:i64) -> i64{
    let fuel = mass/3-2;
    if fuel<=0{
        0
    }else{
        fuel + get_fuel_mass(fuel)
    }
}

#[cfg(feature = "d3_1")]
fn main() {
	let mut current = (0i32,0i32);
	//read
	let mut points = HashSet::new();
	let mut input = BufReader::new(stdin()).lines();
	let line_1=input.next().unwrap().unwrap();
	for step in line_1.split(','){
		let(dir,dists)=step.split_at(1);
		let (dx,dy) = match dir{
			"L" =>(-1,0),
			"R" =>(1,0),
			"U" =>(0,1),
			"D" =>(0,-1),
			_=>unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap(){
			current.0 +=dx;
			current.1 +=dy;
			points.insert(current.clone());
		}
	}
	//test
	current=(0,0);
	let mut conflicts=BinaryHeap::new();
	let line_2=input.next().unwrap().unwrap();
	for step in line_2.split(','){
		let(dir,dists)=step.split_at(1);
		let (dx,dy) = match dir{
			"L" =>(-1,0),
			"R" =>(1,0),
			"U" =>(0,1),
			"D" =>(0,-1),
			_=>unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap(){
			current.0 +=dx;
			current.1 +=dy;
			if points.contains(&current){
				conflicts.push(Reverse(current.0.abs()+current.1.abs()))
			}
		}
	}
	println!("{:?}",conflicts);
}

#[cfg(feature = "d3_2")]
fn main() {
	let mut current = (0i32,0i32);
	//read
	let mut points = HashMap::new();
	let mut input = BufReader::new(stdin()).lines();
	let line_1=input.next().unwrap().unwrap();
	let mut i=0i32;
	for step in line_1.split(','){
		let(dir,dists)=step.split_at(1);
		let (dx,dy) = match dir{
			"L" =>(-1,0),
			"R" =>(1,0),
			"U" =>(0,1),
			"D" =>(0,-1),
			_=>unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap(){
			i+=1;
			current.0 +=dx;
			current.1 +=dy;
			points.insert(current.clone(),i);
		}
	}
	//test
	current=(0,0);
	let mut conflicts=BinaryHeap::new();
	let line_2=input.next().unwrap().unwrap();
	let mut i2=0;
	for step in line_2.split(','){
		let(dir,dists)=step.split_at(1);
		let (dx,dy) = match dir{
			"L" =>(-1,0),
			"R" =>(1,0),
			"U" =>(0,1),
			"D" =>(0,-1),
			_=>unreachable!()
		};
		for _ in 0..dists.parse::<i32>().unwrap(){
			i2+=1;
			current.0 +=dx;
			current.1 +=dy;
			if let Some(dist) = points.get(&current){
				conflicts.push(Reverse(dist+i2))
			}
		}
	}
	println!("{:?}",conflicts);
}

#[cfg(feature = "d3_1_false")]
fn main(){
    let mut first=true;
    //read
    let mut lines =BTreeSet::new();
    for l in stdin().lock().lines(){
        let (mut current_x, mut current_y)=(0,0);
        let s=l.unwrap();
        for a in s.split(','){
            let(dir,dists)=a.split_at(1);
            let dist:i64=dists.parse().unwrap();
            let line = match dir{
                "L" =>{
                    let tmp=current_x;
                    current_x -= dist;
                    Line::Horz {
                        x1: current_x,
                        x2: tmp,
                        y: current_y,
                        first
                    }
                }
                "R" =>{
                    let tmp=current_x;
                    current_x += dist;
                    Line::Horz {
                        x1: tmp,
                        x2: current_x,
                        y: current_y,
                        first
                    }
                }
                "U" =>{
                    let tmp=current_y;
                    current_y += dist;
                    Line::Vert {
                        x: current_x,
                        y1: tmp,
                        y2: current_y,
                        first
                    }
                }
                "D" =>{
                    let tmp=current_y;
                    current_y -= dist;
                    Line::Vert {
                        x: current_x,
                        y1: current_y,
                        y2: tmp,
                        first
                    }
                }
                _=>unreachable!(),
            };
            lines.insert(line);
        }
        first=false;
    }

    let mut mindist=core::i64::MAX;
    let hfilter=|line:&Line|
            if let Line::Horz { x1, x2, y, first } = line{
                Some((*x1,*x2,*y,*first))
            }else{
                None
            };
    let vfilter=|line:&Line|
            if let Line::Vert { x, y1, y2, first } = line{
                Some((*x,*y1,*y2,*first))
            }else{
                None
            };
    println!("linecount:{:?}",lines.iter().collect::<Vec<_>>());
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
									(vx!=0 || hy!=0){
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
    println!("MIN:{:?}",min)
}

#[derive(PartialEq,Eq,Debug)]
enum Line {
    Horz { x1: i64, x2: i64, y: i64, first: bool },
    Vert { x: i64, y1: i64, y2: i64, first: bool },
}

impl PartialOrd for Line{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other){
            (Line::Horz{x1,..},Line::Horz{x1:x_2,..}) => x1.cmp(x_2),
            (Line::Horz{x1,..},Line::Vert{x:x_2,..}) => x1.cmp(x_2),
            (Line::Vert{x:x_1,..},Line::Horz{x1:x_2,..}) => x_1.cmp(x_2),
            (Line::Vert{x:x_1,..},Line::Vert{x:x_2,..}) => x_1.cmp(x_2),
        }
    }
}


#[cfg(feature = "d4_1")]
fn main(){
    let min=147981;
    let max=691423;
    let (count,x)=count1(0,-1,min,max,false,6);
    println!("----\ncount:{}:{}",count,x);
}

fn count1(prefix:u32,lastd:i8,min:u32,max:u32,includes_double:bool,digits_left:u8) ->(u32,bool){
    if digits_left==0{
        if !includes_double{
            return (0,false)
        }
        match prefix{
            i if i<min => (0,false),
            i if i>max => (0,true),
            _=> {
                println!("{}",prefix);
                (1,false)
            }
        }
    }else{
        let start = lastd.max(0);
        let mut count=0;
        for i in start..=9i8{
            let (c,should_stop) = count1(prefix*10+i as u32, i, min, max, includes_double || i==lastd, digits_left-1);
            count+=c;
            if should_stop{
                return (count,true)
            }
        }
        (count,false)
    }
}

#[cfg(feature = "d4_2")]
fn main(){
    let min=147981;
    let max=691423;
    let (count,x)=count2(0,-1,min,max,false,6,0);
    println!("----\ncount:{}:{}",count,x);
}

fn count2(prefix:u32,lastd:i8,min:u32,max:u32,includes_double:bool,digits_left:u8,current_run:u8) ->(u32,bool){
    if digits_left==0{
        if !includes_double && current_run!=2{
            return (0,false)
        }
        match prefix{
            i if i<min => (0,false),
            i if i>max => (0,true),
            _=> {
                println!("{}",prefix);
                (1,false)
            }
        }
    }else{
        let start = lastd.max(0);
        let mut count=0;
        for i in start..=9i8{
            let mut includes_double2=includes_double;
            let mut current_run2=1;
            if lastd==i{
                current_run2 = current_run + 1
            }
            if current_run==2 && lastd!=i{
                includes_double2=true;
            }
            let (c,should_stop) = count2(prefix*10+i as u32, i, min, max, includes_double2, digits_left-1,current_run2);
            count+=c;
            if should_stop{
                return (count,true)
            }
        }
        (count,false)
    }
}

#[cfg(any(feature = "d6_1",feature = "d6_2"))]
fn main() {
    let mut map:HashMap<String,Vec<String>> = HashMap::new();
    let mut map_back=HashMap::new();
    for l in BufReader::new(stdin()).lines(){
        let s=l.unwrap();
        let mut x=s.split(')');
        let a=x.next().unwrap().to_string();
        let b=x.next().unwrap();
        map.entry(a.clone())
                .and_modify(|v|v.push(b.to_string()))
                .or_insert_with(||vec![b.to_string()]);
        map_back.insert(b.to_string(),a.to_string());
    }

    #[cfg(feature = "d6_1")] {//make a DFS, add the depths at each node (depth = # of indirect&direct orbits)
        let mut sum=0;
        let mut stack=vec![("COM".to_string(),0)];
        let empty_vec=vec![];
        while let Some((a,depth))=stack.pop(){
            sum+=depth;
            for b in map.get(&a).unwrap_or(&empty_vec){
                stack.push((b.clone(),depth+1));
            }
        }
        println!("{}",sum);
    }

    #[cfg(feature = "d6_2")] {//make a BFS
        let mut queue=VecDeque::from(vec![("YOU".to_string(),0)]);
        let empty_vec=vec![];
        let mut found=HashSet::new();
        while let Some((a,depth))=queue.pop_front(){
            if found.contains(&a){continue}
            found.insert(a.clone());
            //println!("{}:{}",&a,depth);
            if &a == "SAN"{
                println!("---\nDIST:{}",depth-2)
            }
            if let Some(parent) = map_back.get(&a){
                queue.push_back((parent.clone(),depth+1))
            }
            for b in map.get(&a).unwrap_or(&empty_vec){
                queue.push_back((b.clone(),depth+1));
            }
        }
    }
}

#[cfg(any(feature = "d10_1",feature = "d10_2"))]
fn main(){
	let mut asteroid_locations=HashSet::new();
	let mut ggt_memo=HashMap::new();
	let mut ggt_memoized = move |(a,b)|ggt_memo.entry((a,b)).or_insert_with(||ggT(a,b)).clone();


	for (i,l) in BufReader::new(stdin()).split(b'\n').enumerate(){
		l.unwrap().into_iter()
				//.map(|s|dbg!(s))
				.enumerate()
				.filter(|(i,b)|*b==b'#')
				.map(|(j,_)|(i as i8,j as i8))
				.for_each(|p|{asteroid_locations.insert(p);});
	}
	let mut max=((0,0),0);
	asteroid_locations.iter()
			.map(|a|{
				let s=asteroid_locations.iter()
						.filter(|b|{
							let d= p_minus(b, a);
							let ggt=ggt_memoized(p_abs(&d));
							(1..ggt).all(|i|{
								let p=p_plusmul(a,&d,i,ggt);
								!asteroid_locations.contains(&p)
							}) && (a.1!=b.1 || a.0!=b.0)
						}).count();

				println!("{}",s);
				(a.clone(),s)
			})
			.for_each(|p|if p.1>max.1{max=p});
	println!("{}/{}",max.1,asteroid_locations.len());
	#[cfg(feature = "d10_2")] {
		let a=&max.0;
		//get all 1-dist points, calculate
		let s:BTreeMap<_,_>=asteroid_locations.iter()
				.filter(|b|{
					let d= p_minus(b, a);
					let ggt=ggt_memoized(p_abs(&d));
					(1..ggt).all(|i|{
						let p=p_plusmul(a,&d,i,ggt);
						!asteroid_locations.contains(&p)
					}) && (a.1!=b.1 || a.0!=b.0)
				})
				.map(|p|((p_angle(a,p)*1000.) as i16,p.clone()))
				.collect();

		let t200=s.iter().skip(199).next().unwrap();
		println!("thing:{:?} to {:?}",a,t200)
	}

}

fn p_angle(base:&(i8, i8), target:&(i8, i8)) -> f32 {
	let d=p_minus(target,base);
	let (x,y) = (f32::from(d.0),f32::from(d.1));
	std::f32::consts::PI-y.atan2(x)
}

#[test]
fn test_p_angle(){
	macro_rules! test_eps {
		($a:expr,$b:expr) => {
			assert!( (($a)-($b)).abs()<=std::f32::EPSILON,"{} != {}",$a,$b)
		}
	}
	use std::f32::consts::PI;
	let base = (0,0);
	let p1 = (1,0);
	let p2 = (-1,0);
	let p3 = (0,1);
	let p4 = (0,-1);
	test_eps!(p_angle(&base,&p1),PI);
	test_eps!(p_angle(&base,&p2),0.);
	test_eps!(p_angle(&base,&p3),PI/2.);
	test_eps!(p_angle(&base,&p4),PI/2.*3.);
}


fn p_minus(a:&(i8, i8), b:&(i8, i8)) -> (i8, i8){
	(a.0-b.0,a.1-b.1)
}

fn p_plusmul(a:&(i8,i8),b:&(i8,i8),fac:i8, div:i8) -> (i8,i8){
	(a.0+b.0/div*fac,a.1+b.1/div*fac)
}
fn p_abs(a:&(i8, i8)) -> (i8, i8){
	(a.0.abs(),a.1.abs())
}

fn ggT(mut a:i8,mut b:i8)->i8 {
	if b>a { swap(&mut b,&mut a) }
	if b==0{
		a
	}else{
		ggT(a-b,b)
	}
}



#[cfg(feature = "d2_1")]
fn main()->Result<(),Box<dyn Error>> {
    let mut nix=VecDeque::new();
    let mut program=intcode_read();
    println!("program: {:?}",program);
    program[1]=12;
    program[2]=2;

    let mut out=Some(0);
    //let mut pc=0;
    while let Some(pc) = out{
        println!("{}\t:{}",pc,program[pc]);
        out=intcode_execute(pc, &mut program, &mut nix).0;
    }
    println!("{:?}",program);
    Ok(())
}

#[cfg(feature = "d2_2")]
fn main()->Result<(),Box<dyn Error>> {
    let mut nix=VecDeque::new();
    let mut original_program=intcode_read();
    for noun in 0..100{
        for verb in 0..100{
            let mut program = original_program.clone();
            program[1]=noun;
            program[2]=verb;
            let mut out=Some(0);
            while let Some(pc) = out{
                out=intcode_execute(pc, &mut program,&mut nix).0;
            }
            println!("{} {}\t:{}",noun,verb,program[0]);
            if program[0]==19690720{
                println!("---------------------------\n FOUND!");
                return Ok(())
            }
        }
    }
    println!("NOTHING FOUND!!!");
    Ok(())
}



#[cfg(any(feature = "d5_1",feature = "d5_2"))]
fn main()->Result<(),Box<dyn Error>> {
    let mut input=VecDeque::new();
    let mut program=intcode_read();
    //println!("program: {:?}",program);
    #[cfg(feature = "d5_1")]
    input.push_back(1);
    #[cfg(feature = "d5_2")]
    input.push_back(5);

    let mut next=Some(0);

    while let Some(pc) = next{
        println!("{}\t:{}",pc,program[pc]);
        let o=intcode_execute(pc, &mut program, &mut input);
		next=o.0;
        if let Some(x)=o.1{
            println!("OUT:{}",x);
        }
    }
    //println!("{:?}",program);
    Ok(())
}

#[cfg(feature = "d7_1")]
fn main()->Result<(),Box<dyn Error>> {
    let mut program=intcode_read();
    let (mut max_phase_out,mut max_phase_in)=(-1,[-1;5]);
    for p1 in 0..=4{
        let i1=VecDeque::from(vec![p1,0]);
        let o1=intcode_run_once(program.clone(),i1);
        for p2 in 0..=4{
			if p2==p1 {continue}
            let i2=VecDeque::from(vec![p2,o1]);
            let o2=intcode_run_once(program.clone(),i2);
            for p3 in 0..=4{
				if p3==p1||p3==p2 {continue}
                let i3=VecDeque::from(vec![p3,o2]);
                let o3=intcode_run_once(program.clone(),i3);
                for p4 in 0..=4{
					if p4==p1||p4==p2||p4==p3 {continue}
                    let i4=VecDeque::from(vec![p4,o3]);
                    let o4=intcode_run_once(program.clone(),i4);
                    for p5 in 0..=4{
						if p5==p1||p5==p2||p5==p3||p5==p4 {continue}
                        let i5=VecDeque::from(vec![p5,o4]);
                        let o5=intcode_run_once(program.clone(),i5);
                        if o5>max_phase_out{
                            max_phase_out=o5;
                            max_phase_in = [p1,p2,p3,p4,p5];
                        }
                    }
                }
            }
        }
    }
	println!();

    println!("{}   (from {:?})",max_phase_out,max_phase_in);
    Ok(())
}

#[cfg(feature = "d7_2")]
fn main()->Result<(),Box<dyn Error>> {
    let mut program=intcode_read();
    let (mut max_phase_out,mut max_phase_in)=(-1,[0;5]);
    let mut phases=[5,6,7,8,9];
    for i1 in 0..5{
        phases.swap(0,i1);
        for i2 in 1..5{
            phases.swap(1,i2);
            for i3 in 2..5{
                phases.swap(2,i3);
                for i4 in 3..5{
                    phases.swap(3,i4);
                    let o=amps(&program,&phases);

                    if o>max_phase_out{
                        max_phase_out = o;
                        max_phase_in = phases.clone();
                    }
                    phases.swap(3,i4);
                }

                phases.swap(2,i3);
            }

            phases.swap(1,i2);
        }

        phases.swap(0,i1);
    }
    println!();

    println!("{}   (from {:?})",max_phase_out,max_phase_in);
    Ok(())
}

fn amps(program:&Vec<i64>, phases:&[u8;5]) ->i64{
    let mut progs=[
        program.clone(),
        program.clone(),
        program.clone(),
        program.clone(),
        program.clone(),
    ];
    let mut pcs=[Some(0);5];
    let mut inputs:[VecDeque<_>;5]=[
        vec![phases[0] as i64,0].into(),
        vec![phases[1] as i64].into(),
        vec![phases[2] as i64].into(),
        vec![phases[3] as i64].into(),
        vec![phases[4] as i64].into(),
    ];
    let mut i=0;
    let mut loops=0; //debug
    loop{
        let (output,pc_new) = intcode_run_to_output(&mut progs[i], &mut inputs[i], pcs[i]);
        if pc_new.is_none(){
            println!("{:?}:\t{}\t{}",phases,inputs[i].front().unwrap(),loops);
            return inputs[i].pop_front().unwrap();
        }
        pcs[i]=pc_new;
        i=(i+1)%5;
        inputs[i].push_back(output.unwrap());
        loops+=1;
    }
}

#[cfg(any(feature = "d9_1",feature = "d9_2"))]
fn main(){
	use intcode::Machine;
	let mut m = Machine::from_stdin_allocate(512);
	println!("alloc!");
	let mut inp=VecDeque::new();
	#[cfg(feature = "d9_1")]
	inp.push_back(1);
	#[cfg(feature = "d9_2")]
	inp.push_back(2);
	while let Some(output) = m.run_to_output(&mut inp){
		println!("output:{}",output);
	}
	println!("ins_count:{}",m.instruction_counter);
}





fn intcode_run_to_output(program:&mut Vec<i64>,input:&mut VecDeque<i64>,mut next:Option<usize>) ->(Option<i64>,Option<usize>){
    while let Some(pc) = next{
        //println!("{}\t:{}",pc,program[pc]);
        let o=intcode_execute(pc, program, input);
        next=o.0;
        if let Some(x)=o.1{
            return (Some(x),next);
        }
    }
    (None,None)
}

fn intcode_run_once(mut program:Vec<i64>,mut input:VecDeque<i64>) -> i64{
    let mut out=-1;
    let mut next=Some(0);
    while let Some(pc) = next{
        //println!("{}\t:{}",pc,program[pc]);
        let o=intcode_execute(pc, &mut program, &mut input);
		next=o.0;
        if let Some(x)=o.1{
            out=x;
        }
    }
    out
}

fn intcode_read() -> Vec<i64>{
    let mut out = vec![];

    for bytes in BufReader::new(stdin()).split(b','){
        let mut i:i64=0;
        let mut negative=false;
        //parse positive integer
        for b in bytes.unwrap(){
            if b.is_ascii_digit(){
                i = i*10 + (b-b'0') as i64;
            }else if b == b'-'{
                negative=true;
            }
        }
        if negative{
            i=-i
        }
        out.push(i);
    }
    out
}

/*
#[derive(Debug)]
enum IntCodeError {
    UnknownInstruction(i64)
}
impl Display for IntCodeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        writeln!(f,"{:?}",&self)
    }
}

impl Error for IntCodeError{}
*/


fn intcode_execute(pc:usize, data:&mut Vec<i64>, input:&mut VecDeque<i64>) -> (Option<usize>,Option<i64>){
    let mode1=((data[pc]/100) % 10 )as u8;
    let mode2=((data[pc]/1000) % 10)as u8;
    let mode3=((data[pc]/10000) % 10)as u8;
    let opcode =data[pc]%100;
    match opcode {
        99 => (None,None),
        1 => { // ADD
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            let c=intcode_fetch(data,1,pc+3);
            data[c as usize]=a+b;
            (Some(pc +4),None)
        }
        2 => { // MUL
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            let c=intcode_fetch(data,1,pc+3);
            data[c as usize]=a*b;
            (Some(pc +4),None)
        }
        3 => { // INPUT
            let c=intcode_fetch(data,1,pc+1);
            data[c as usize]=input.pop_front().unwrap();
            (Some(pc +2),None)
        }
        4 => { // OUTPUT
            let a=intcode_fetch(data,mode1,pc+1);
            (Some(pc +2),Some(a))
        }
        5 => { //JUMP IF TRUE
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            if a!=0{
                (Some(b as usize),None)
            }else {
                (Some(pc +3),None)
            }
        }
        6 => { //JUMP IF FALSE
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            if a==0{
                (Some(b as usize),None)
            }else {
                (Some(pc +3),None)
            }
        }
        7 => { // LESS THAN
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            let c=intcode_fetch(data,1,pc+3);
            data[c as usize] = if a<b {1} else {0};
            (Some(pc +4),None)
        }
        8 => { // EQUALS
            let a=intcode_fetch(data,mode1,pc+1);
            let b=intcode_fetch(data,mode2,pc+2);
            let c=intcode_fetch(data,1,pc+3);
            data[c as usize] = if a==b {1} else {0};
            (Some(pc +4),None)
        }
        x => unreachable!("no such isntruction code: {} ",x)
    }
}

#[inline]
fn intcode_fetch(data:&Vec<i64>,mode:u8,addr:usize) -> i64{
    match mode {
        0 => data[data[addr] as usize],
        1 => data[addr],
        _ => unreachable!(),
    }
}
