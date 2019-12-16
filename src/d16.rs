use std::io::{stdin, Read};

pub fn run(){
	let s= stdin_to_string();

	#[cfg(feature="d16_1")]{
		let v=part1(s.as_str());
		println!("vec:{}",to_string(&v[0..8]));
	}

	#[cfg(not(feature="d16_1"))]{
		let (v1,offs)=part2(s.as_str());
		println!("vec2:{}",to_string(&v1[offs..offs+8]));
	}
}

fn part1(s:&str)->Vec<i8>{
	let mut v= string_to_vec(s);
	let mut tmp=vec![0;v.len()];

	for i in 0..100{
		iteration(&mut v, &mut tmp, 0);
	}
	v
}

fn part2(digits:&str) ->(Vec<i8>, usize){
	let mut v_single = string_to_vec(digits);
	let mut v_all = Vec::with_capacity(10000* v_single.len());
	let offset =get_count(&v_single[0..7]);
	//copy them over
	for _i in 0..10000{
		for j in &v_single {
			v_all.push(*j);
		}
	}
	//build a reusable array in heap for temporary values
	let mut tmp=vec![0; v_all.len()- offset];
	for i in 0..100{
		iteration(&mut v_all, &mut tmp, offset);
	}
	// return final array and offset
	(v_all, offset)
}
fn stdin_to_string() ->String{
	let mut s=String::new();
	stdin().read_to_string(&mut s).unwrap();
	s
}

fn string_to_vec(s:&str) ->Vec<i8>{
	s.bytes()
			.filter(|b|b'0'<=*b && *b<=b'9')
			.map(|b|(b-b'0') as i8)
			.collect()
}

// this is basically pattern[i]-pattern[i-1], we'll need that later.
const D_VAL:[i32;4]=[1,1,-1,-1];
fn iteration(arr:&mut Vec<i8>, tmp:&mut Vec<i32>, offset:usize){

	let l=arr.len();
	let mut sum=0;
	//go over the array backwards, but further than the offset is not needed as we don't need values before the offset.
	for i in (offset..l).rev(){
		let i1=i+1;

		//while going over backwards, keep a sum of the encountered values and save them in the temporary array.
		sum+=arr[i] as i32;
		tmp[i-offset]=sum;

		// now we calculate the actual future value for arr[i]. We start by filling it with the current sum
		// which is 1*arr[i] + 1*arr[i+1]+... + 1*arr[l-1].
		let mut accum=sum;

		// We will now only work with the known sums of suffixes.

		// for all i where there is more than a sequence of 1s in the associated pattern, we must substract the sum of the region which starts where the next 0s begin
		// then we substract the sum of region beginning where the next -1s are, adding the next 0s etc. Visualization:
		// Example I: i=2 (we have to get pattern*3), i1=3
		//d: 0  1  2  3  4  5  6  7  8  9
		//         1  1  1  1  1  1  1  1   tmp[i1   - 1] = tmp[i]
		// -                1  1  1  1  1   tmp[i1*2 - 1] * -1
		// -                         1  1   tmp[i1*3 - 1] * -1
		// ==============================
		//         1  1  1  0  0  0 -1 -1
		//
		//
		// Example II: i=1 (we have to get pattern*2), i1=2
		//d: 0  1  2  3  4  5  6  7  8  9
		//      1  1  1  1  1  1  1  1  1  tmp[i1   - 1] = tmp[i]
		// -          1  1  1  1  1  1  1  tmp[i1*2 - 1] * -1
		// -                1  1  1  1  1  tmp[i1*3 - 1] * -1
		// +                      1  1  1  tmp[i1*4 - 1] *  1
		// +                            1  tmp[i1*5 - 1] *  1
		// ==============================
		//      1  1  0  0 -1 -1  0  0  1
		//
		// etc.

		//we start with j=2 and increase it as long as i1*j-1 is still within range. The sign is determined from D_VAL (defined above)
		//we add all results to our accumulator accum.
		let mut j=2;
		while l>j*i1-1{
			accum += tmp[j*i1-1-offset]*D_VAL[j&0b11];
			j+=1;
		}
		//we then remove the minus and take the last digit and stick it back into the array.
		//As each step accesses only previous tmp values and the current value of the array, we can update inplace.
		arr[i] = (accum.abs() %10) as i8;
	}
}

fn to_string(v:&[i8])->String{
	v.iter().map(|i|(*i as u8+b'0') as char).collect()
}

#[test]
fn test_iters(){

	let a = [
//		"12345678",
		"80871224585914546619083218645595",
		"19617804207202209144916044189917",
		"69317163492948606335995924319873"];
	let b = [/*"01029498",*/"24176176","73745418","52432133"];
	for i in 0..3{
		let out=b[i].to_string();
		let v=part1(a[i]);
		let out2=to_string(&v[0..8]);
		println!("done");
		assert_eq!(out,out2, "i:{}",i);
	}
}
#[test]
fn test_iters2(){

	let a = [
//		"12345678",
		"03036732577212944063491565474664",
		"02935109699940807407585447034323",
		"03081770884921959731165446850517"];
	let b = ["84462026","78725270","53553731"];
	for i in 0..3{
		let out=b[i].to_string();
		let (v,offs)=part2(a[i]);
		let out2=to_string(&v[offs..offs+8]);
		println!("done");
		assert_eq!(out,out2, "i:{}",i);
	}
}

fn get_count(v:&[i8]) -> usize{
	let mut accum=0;
	for i in v{
		accum = 10*accum+(*i as usize);
	}
	accum
}
