use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};


const CSV_HEADER: &'static str = "time:u128, to:String, from:String, amount:f64\n";

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
}

impl Record {
	fn new(to: String, from: String, amount: f64) -> Result<Record, &'static str> {
		let mut _time: u128 = 0;
		match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(t) => _time = t.as_nanos(),
			Err(_) => return Err("YOU ARE IN ERROR!\n\tSYSTEM TIME BEFORE EPOCH TIME!"),
		}
		Ok(Record {
			time: _time,
			to: to,
			from: from,
			amount: amount,
		})
	}
}

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	// reuse args, no reason to let a perfectly good variable name go to waste. 
	let args: Args = Args::new(&args).unwrap_or_else(|e| {
		println!("Problem parsing arguments: {}", e);
		process::exit(1);
	});

	let a_record = Record::new(String::from("Bob"), String::from("Alice"), 100.0)
		.unwrap_or_else(|e| {
			println!("System Error: {}", e);
			process::exit(2);
		});

	println!("Key: {}\nin_file: {}\nout_file: {}", args.key, args.in_file, args.out_file);
	println!("Time: {}To: {}From: {}Amount: {}", a_record.time, a_record.to, a_record.from, a_record.amount);
		
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
