#[link(name="index")];
#[pkgid = "index#0.1"];
#[crate_type="rlib"];

#[deny(non_camel_case_types)];
#[deny(missing_doc)];

//! Boolean algebra library for indexing.

/// Is returned by 'index_of'.
#[deriving(Eq)]
pub enum IndexMatch {
	/// Item found at index.
	Found(uint),
	/// Item not found, but can be inserted at index.
	FoundLarger(uint)
}

/// Index is a struct used for doing algebraic set operations.
/// It contains a hidden vector of the type 'T'.
/// The items are sorted in ascending order.
pub struct Index<T> {
	priv ids: ~[T],
}

impl<T: Clone + Ord + Eq> Index<T> {

	/// Constructs a new Index without checking the order.
	/// This method takes ownership of the vector.
	pub fn new(data: ~[T]) -> Index<T> { Index { ids: data } }

	/// Checks that each item in vector is larger than previous one.
	pub fn check_order(data: &~[T]) -> bool {
		!data.windows(2).any(|w| w[0] >= w[1])
	}

	/// Clones the vector.
	pub fn to_vec(&self) -> ~[T] { self.ids.clone() }

	/// Returns Found(i) if the item was found in the Index.
	/// Returns FoundNext(i), the index to insert, if the item was not found.
	/// Worst case performance: O(log(N)).	
	pub fn index_of(&self, item: &T) -> IndexMatch {
		let list = &self.ids;
		let n = list.len() as int;
		if n == 0 { return FoundLarger(0u); }
		let mut low: int = 0;
		let mut high: int = n - 1;
		while low <= high {
			let i: int = (low + high) / 2;
			if list[i] < *item {
				low = i + 1;
				continue;
			}
			if list[i] > *item {
				high = i - 1;
				continue;
			}	

			return Found(i as uint);
		}

		FoundLarger(low as uint)
	}

	/// Inserts a new item.
	pub fn insert(&mut self, item: T) -> bool {
		match self.index_of(&item) {
			Found(_) => false,
			FoundLarger(i) => {
				self.ids.insert(i, item);
				true
			},
		}
	}
}

/// Creates a new Index containing all items from both arguments.
impl<T: Ord + Eq + Clone> Add<Index<T>, Index<T>> for Index<T> {
	fn add(&self, rhs: &Index<T>) -> Index<T> {
		Index { ids: or(self.ids, rhs.ids) }
	}
}

/// Creates a new Index containing items from both arguments.
impl<T: Ord + Eq + Clone> Mul<Index<T>, Index<T>> for Index<T> {
	fn mul(&self, rhs: &Index<T>) -> Index<T> {
		Index { ids: and(self.ids, rhs.ids) }
	}
}

/// Creates a new Index containing items from the left argument but not the second.
impl<T: Ord + Eq + Clone> Sub<Index<T>, Index<T>> for Index<T> {
	fn sub(&self, rhs: &Index<T>) -> Index<T> {
		Index { ids: except(self.ids, rhs.ids) }
	}
}

fn or<T: Ord + Eq + Clone>(a: &[T], b: &[T]) -> ~[T] {
	let mut i = 0;
	let mut j = 1;
	let an = a.len();
	let bn = b.len();
	let mut result: ~[T] = ~[];
	while i < an || j < an {
		let av = 
			if i < an {
				&a[i]
			} else {
				&a[an - 1]
			};
		let bv = 
			if j < bn {
				&b[j]
			} else {
				&b[bn - 1]
			};
		let min = 
			if i >= an {
				bv
			} else if j >= bn {
				av
			} else {
				if av < bv {
					av
				} else {
					bv
				}
			};
		if i < an && min == av {
			i += 1;
		}
		if j < bn && min == bv {
			j += 1;
		}

		result.push(min.clone());
	}

	result
}

fn and<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
	let mut i = 0;
	let mut j = 0;
	let an = a.len();
	let bn = b.len();
	let mut result: ~[T] = ~[];
	while i < an && j < bn {
		let av = &a[i];
		let bv = &b[j];
		let min =
			if av < bv {
				av
			} else {
				bv
			};
		let isAv = min == av;
		let isBv = min == bv;
		if isAv {
			i += 1;
		}
		if isBv {
			j += 1;
		}

		if isAv && isBv {
			result.push(min.clone());
		}
	}

	result
}

fn except<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
	let mut i = 0;
	let mut j = 0;
	let an = a.len();
	let bn = b.len();
	let mut result: ~[T] = ~[];
	while i < an {
		let av = &a[i];
		let bv =
			if j < bn {
				&b[j]
			} else {
				&b[bn - 1]
			};
		let min =
			if j >= bn {
				av
			} else {
				if av < bv {
					av
				} else {
					bv
				}
			};
		let isAv = min == av;
		let isBv = min == bv;
		if isAv {
			i += 1;
		}
		if j < bn && isBv {
			j += 1;
		}
		if isAv && !isBv {
			result.push(min.clone());
		}
	}

	result
}

