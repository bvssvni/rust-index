#[crate_id = "index#0.1"];
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

/// Checks that each item in vector is larger than previous one.
pub fn check_order<T: Ord>(data: &~[T]) -> bool {
	!data.windows(2).any(|w| w[0] >= w[1])
}

/// Returns Found(i) if the item was found in the Index.
/// Returns FoundNext(i), the index to insert, if the item was not found.
/// Worst case performance: O(log(N)).	
pub fn index_of<T: Ord>(list: &[T], item: &T) -> IndexMatch {
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
pub fn insert<T: Ord>(list: &mut ~[T], item: T) -> bool {
	use std::vec::OwnedVector;

	match index_of(*list, &item) {
		Found(_) => false,
		FoundLarger(i) => {
			list.insert(i, item);
			true
		},
	}
}

/// Creates a set containing elements from 'a' and 'b'.
pub fn or<T: Ord + Eq + Clone>(a: &[T], b: &[T]) -> ~[T] {
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

/// Creates a set containing elements that are both in 'a' and 'b'.
pub fn and<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
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

/// Creates a set of elements that are in 'a' but not in 'b'.
pub fn except<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
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

