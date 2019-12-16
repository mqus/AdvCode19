use std::io::{BufReader, stdin, BufRead};
use std::collections::VecDeque;

pub struct Machine{
	memory:Vec<Integer>,
	pc:Option<usize>,
	relative_base:Integer,
	pub instruction_counter:u32,
}

const MEMSIZE:usize=16*1024*1024;
type Integer=i64;

impl Machine{

	pub fn from_stdin_allocate(memsize:usize) -> Self {
		let mut m = Machine{
			memory: vec![0;memsize],
			pc: Some(0),
			relative_base: 0,
			instruction_counter: 0
		};
		for (pos,bytes) in BufReader::new(stdin()).split(b',').enumerate(){
			let mut i:Integer=0;
			let mut negative=false;
			//parse positive integer
			for b in bytes.unwrap(){
				if b.is_ascii_digit(){
					i = i*10 + (b-b'0') as Integer;
				}else if b == b'-'{
					negative=true;
				}
			}
			if negative{
				i=-i
			}
			m.memory.insert(pos,i);
		}
		m
	}
	pub fn from_stdin() ->Self{
		let mut m = Machine{
			memory: vec![],
			pc: Some(0),
			relative_base: 0,
			instruction_counter: 0
		};
		for bytes in BufReader::new(stdin()).split(b','){
			let mut i:Integer=0;
			let mut negative=false;
			//parse positive integer
			for b in bytes.unwrap(){
				if b.is_ascii_digit(){
					i = i*10 + (b-b'0') as Integer;
				}else if b == b'-'{
					negative=true;
				}
			}
			if negative{
				i=-i
			}
			m.memory.push(i);
		}
		m
	}



	pub fn run_to_output(&mut self,input:&mut VecDeque<Integer>) ->Option<Integer>{
		while !self.is_done(){
			if let Some(o)=self.execute_step(input){
				return Some(o)
			}
		}
		None
	}

	pub fn intcode_run_once(&mut self,mut input:VecDeque<Integer>) -> Integer{
		let mut out=-1;
		while !self.is_done(){
			if let Some(o)=self.execute_step(&mut input){
				out=o
			}
		}
		out
	}

	fn execute_step(&mut self, input:&mut VecDeque<Integer>) -> Option<Integer>{
		if self.pc.is_none(){
			return None
		}
		self.instruction_counter+=1;
		let pc = self.pc.unwrap();
		//println!("{}\t:{:?}",pc,&self.memory[pc..pc+4]);
		let mode1=((self.memory[pc]/100) % 10 )as u8;
		let mode2=((self.memory[pc]/1000) % 10)as u8;
		let mode3=((self.memory[pc]/10000) % 10)as u8;
		let opcode =self.memory[pc]%100;
		match opcode {
			99 => {
				self.pc=None;
				None
			},
			1 => { // ADD
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);
				self.write(mode3,pc+3,a+b);
				self.pc = Some(pc +4);
				None
			}
			2 => { // MUL
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);
				self.write(mode3,pc+3,a*b);
				self.pc = Some(pc +4);
				None
			}
			3 => { // INPUT
				let val=input.pop_front().unwrap();
				self.write(mode1,pc+1,val);
				self.pc = Some(pc +2);
				None
			}
			4 => { // OUTPUT
				let a=self.fetch(mode1,pc+1);
				self.pc = Some(pc+2);
				Some(a)
			}
			5 => { //JUMP IF TRUE
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);
				if a!=0{
					self.pc = Some(b as usize);
					None
				}else {
					self.pc = Some(pc +3);
					None
				}
			}
			6 => { //JUMP IF FALSE
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);
				if a==0{
					self.pc = Some(b as usize);
					None
				}else {
					self.pc = Some(pc +3);
					None
				}
			}
			7 => { // LESS THAN
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);

				self.write(mode3,pc+3,if a<b {1} else {0});

				self.pc = Some(pc +4);
				None
			}
			8 => { // EQUALS
				let a=self.fetch(mode1,pc+1);
				let b=self.fetch(mode2,pc+2);

				self.write(mode3,pc+3,if a==b {1} else {0});

				self.pc = Some(pc +4);
				None
			}
			9 => { // CHANGE RELATIVE BASE
				let a=self.fetch(mode1,pc+1);
				self.relative_base +=a;
				self.pc = Some(pc + 2);
				None
			}
			x => unreachable!("no such isntruction code: {} ",x)
		}
	}

	#[inline]
	fn fetch(&self,mode:u8,addr:usize) -> Integer {
		match mode {
			0 => self.memory[self.memory[addr] as usize], //position mode
			1 => self.memory[addr], //immediate mode
			2 => self.memory[(self.memory[addr] + self.relative_base) as usize], //relative mode
			_ => unreachable!(),
		}
	}

	#[inline]
	fn write(&mut self, mode:u8,addr:usize, value:Integer){
		let location = match mode {
			0 => self.memory[addr] as usize, //position mode
			1 => unreachable!("immediate write is illegal"), //immediate mode
			2 => (self.memory[addr] + self.relative_base) as usize, //relative mode
			_ => unreachable!(),
		};
		self.memory[location] = value;
	}

	#[inline]
	pub fn is_done(&self)->bool{
		self.pc.is_none()
	}

	#[inline]
	pub fn get_mem(&self,pos:usize) ->Integer{
		self.memory[pos]
	}

	#[inline]
	pub fn set_mem(&mut self,pos:usize, val:Integer){
		self.memory[pos]=val
	}
}
