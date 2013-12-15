
mod index {
	pub struct Index<T> {
		priv ids: ~[T],
	}

	pub fn new<T>(data: ~[T]) -> Index<T> {
		Index { ids: data }
	}

	pub fn to_vec<T:Clone>(a: Index<T>) -> ~[T] {
		a.ids.clone()
	}

	impl<T:Ord+Eq+Clone> Add<Index<T>, Index<T>> for Index<T> {
		fn add(&self, rhs: &Index<T>) -> Index<T> {
			Index { ids: or(self.ids, rhs.ids) }
		}
	}

	impl<T:Ord+Eq+Clone> Mul<Index<T>, Index<T>> for Index<T> {
		fn mul(&self, rhs: &Index<T>) -> Index<T> {
			Index { ids: and(self.ids, rhs.ids) }
		}
	}

	impl<T:Ord+Eq+Clone> Sub<Index<T>, Index<T>> for Index<T> {
		fn sub(&self, rhs: &Index<T>) -> Index<T> {
			Index { ids: except(self.ids, rhs.ids) }
		}
	}

	fn or<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
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

	#[deriving(Eq)]
	pub enum IndexOf {
		Found(int),
		Next(int)
	}
	
	pub fn index_of<T:Ord+Eq>(ind: Index<T>, item: T) -> IndexOf {
		let list = ind.ids;
		let mut low: int = 0;
		let mut high: int = list.len() as int - 1;
		while low <= high {
			let i = (low + high) / 2;
			if list[i] < item {
				low = i + 1;
				continue;
			}
			if list[i] > item {
				high = i - 1;
				continue;
			}			

			// return i;
			return Found(i);
		}

		// -(low + 1)
		Next(low)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_or_operator() {
		let a = ::index::new( ~[1, 2, 3] );
		let b = ::index::new( ~[2, 3, 4] );
		let c = a + b;
		assert_eq!(::index::to_vec(c), ~[1, 2, 3, 4]);
	}

	#[test]
	fn test_and_operator() {
		let a = ::index::new( ~[1, 2, 3] );
		let b = ::index::new( ~[2, 3, 4] );
		let c = a * b;
		assert_eq!(::index::to_vec(c), ~[2, 3]);
	}

	#[test]
	fn test_except_operator() {
		let a = ::index::new( ~[1, 2, 3] );
		let b = ::index::new( ~[2, 3, 4] );
		let c = a - b;
		assert_eq!(::index::to_vec(c), ~[1]);
	}

	#[test]
	fn test_advanced() {
		let a = ::index::new( ~[1, 5, 8] );
		let b = ::index::new( ~[0, 1, 5] );
		let c = ::index::new( ~[0, 1, 5, 8] );
		let d = a + b - c;
		assert_eq!(::index::to_vec(d), ~[]);
	}

	#[test]
	fn test_index_of_Index() {
		let a = ::index::new( ~[1, 5, 10] );
		let b = ::index::index_of(a, 5);
		assert_eq!(b, ::index::Found(1));
	}

	#[test]
	fn test2_index_of_Index() {
		let a = ::index::new( ~[1, 2, 10] );
		let b = ::index::index_of(a, 0);
		assert_eq!(b, ::index::Next(0));
	}

	#[test]
	fn test_strs() {
		let a = ::index::new( ~[~"apes", ~"banana", ~"monkey"] );
		let b = ::index::new( ~[~"banana", ~"monkey", ~"snakes"] );
		let c = a + b;
		assert_eq!(::index::to_vec(c), 
			~[~"apes", ~"banana", ~"monkey", ~"snakes"]);
	}

	#[test]
	fn test_example_2() {
		let a = ::index::new( ~[1, 2, 3] );
		let b = ::index::new( ~[2, 3, 4] );
		let c = ::index::new( ~[1, 4] );
		let d = a * (b - c);
		assert_eq!(::index::to_vec(d), ~[2, 3]);
	}
}

