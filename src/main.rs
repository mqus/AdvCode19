use std::error::Error;
use std::io::{BufReader, stdin, BufRead};
use std::str::FromStr;
use std::fmt::{Display, Debug, Formatter};
use core::fmt;
use std::hint::unreachable_unchecked;
use std::collections::VecDeque;

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