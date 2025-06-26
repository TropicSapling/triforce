macro_rules! debug {
	($e:expr) => ({println!("");dbg!($e)})
}

// TODO: use Rust-style 'annotate_snippets' errors instead
// - https://docs.rs/annotate-snippets/latest/annotate_snippets/
macro_rules! error {
	($e:expr) => ({
		eprint!("error: ");
		eprintln!($e);
		std::process::exit(1);
	})
}
