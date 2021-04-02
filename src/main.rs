#![allow(dead_code)]
use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const CSV_HEADER: &'static str = "time:u128, to:String, from:String, amount:f64\n";
/*
	# holiday grains = post holiday brains
	# where am I?
	# what whas I doin?
	# what language even is this?
	# who wrote this?
	# how do I comment?
	# I have no idea what I'm doing, lolz.
*/

// should probabl write some file stuffs here...
fn write_csv_file() {

}

fn read_csv_file() {

}

// need a p2p daemon


// lol, this function works. 
// cryptograpically dumb. 
fn hash_xor(message: &String) -> String {
	let mut buffer = vec![1u8; 256]; //empty buffer of ones, sometimes a too
	let bytes = message.as_bytes();
	for (j, i) in buffer.iter_mut().enumerate() {
		*i ^= bytes[(j%bytes.len())];
	}
	let hash: String = String::from_utf8(buffer).unwrap();
	hash
}

// input: Sausage
// returns: Pig
// *Note: 100% pork products must be used to avoid reverse-fly scenarios 

fn sausage_to_pig() {}

fn nano_squanch() -> u128 {
	let mut _time: u128 = 0;
	if let Ok(t) = SystemTime::now().duration_since(UNIX_EPOCH) {
		_time = t.as_nanos();
	} else { panic!("SYSTEM TIME BEFORE EPOCH TIME"); } // halt here, this should never happen.
	_time
}

struct Args {
	key: String,
	in_file: String,
	out_file: String,
}

impl Args {
	fn new(a: &[String]) -> Result<Args, &'static str> {
		if a.len() < 3 { return Err("Not enough arguments"); }
		Ok(Args {
			// clone bone!
			key: a[1].clone(),
			in_file: a[2].clone(),
			out_file: a[3].clone(),
		})
	}
}

struct Record {
	time: u128,
	to: String,
	from: String,
	amount: f64,
	last_hash: String,
	hash: String,
}

impl Record {
	fn new(to: String, from: String, amount: f64, last_hash: String) -> Record {
		let t = nano_squanch();
		Record {
			time: t,
			to: to,
			from: from,
			amount: amount,
			last_hash: last_hash,	
			hash: hash_xor(&String::from("sfef aefaef arf bqrtn wret wrsgfdga df aret bretn wrtymn erm76e,65, 7ertwynwer6jnhw54me567myew54tynwsretb")) // message should be the record filds, but I gotta go for now...
		}
	}
}

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	// reuse args, no reason to let a perfectly good variable name go to waste. 
	let args: Args = Args::new(&args).unwrap_or_else(|e| {
		println!("Problem parsing arguments: {}", e);
		process::exit(1);
	});

	let a_record = Record::new(String::from("Bob"), String::from("Alice"), 100.0, hash_xor(&String::from("firsthash")));

	println!("Key: {}\nin_file: {}\nout_file: {}", args.key, args.in_file, args.out_file);
	println!("Time: {}\nTo: {}\nFrom: {}\nAmount: {}\nLast Hash: {:2X?}\nHash: {:2X?}\n", a_record.time, a_record.to, a_record.from, a_record.amount, a_record.last_hash.as_bytes(), a_record.hash.as_bytes());
		
	let mut f = File::open(args.in_file)?;
	let of = args.out_file;
	let key = args.key.as_bytes();

	let mut buffer = Vec::new();
	f.read_to_end(&mut buffer)?;
	println!("key {:?}\nbuffer\n{:02X?}\n", key, buffer);

	// encode and print
	for (j, i) in buffer.iter_mut().enumerate() {
		let l = key.len();
		*i ^= key[(j%l)];
	}
	println!("buffer\n{:02x?}\n", buffer);
	fs::write("bar_enc.txt", &buffer)?;

	// decode and print
	for (j, i) in buffer.iter_mut().enumerate() {
		let l = key.len();
		*i ^= key[(j%l)]; // xor and rotate through the key.
	}
	println!("buffer_dec\n{:02x?}\n", buffer);
	fs::write(of, &buffer)?;

	Ok(())
}
