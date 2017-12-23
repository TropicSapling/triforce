use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	if args.len() < 2 {
		println!("Please specify an input file.");
	} else {
		let input = &args[1];
		let mut output = &String::from("default");
		if args.len() > 2 {
			output = &args[2];
		}

		println!("In: {}", input);
		println!("Out: {}", output);
	}
}