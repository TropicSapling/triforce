//use std::ops::{Bound, RangeBounds};

macro_rules! debug {
	($e:expr) => (println!("");dbg!($e))
}

/*// Why this is not in the std lib is a mystery...
pub trait Substr {
	fn substr<R>(&self, r: R) -> String where R: RangeBounds<usize>;
}

impl Substr for str {
	fn substr<R>(&self, r: R) -> String where R: RangeBounds<usize> {
		let beg = match r.start_bound() {
			Bound::Included(n) => *n,
			Bound::Excluded(n) => *n + 1,
			Bound::Unbounded   => 0
		};

		let end = match r.end_bound() {
			Bound::Included(n) => *n + 1,
			Bound::Excluded(n) => *n,
			Bound::Unbounded   => self.len()
		};

		self.chars().skip(beg).take(end - beg).collect()
	}
}*/
