
mod index {
	pub fn or<T:Ord+Eq+Clone>(a: &[T], b: &[T]) -> ~[T] {
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

	#[test]
	fn test_or() {
		let a: ~[int] = ~[1, 2, 3];
		let b: ~[int] = ~[2, 3, 4];
		let c = or(a, b);
		assert_eq!(c, ~[1, 2, 3, 4]);
	}

	pub struct Group<T> {
		members: ~[T],
	}

	pub trait BooleanAlgebra<T> {
		fn or(&self, b: T) -> T;
		fn and(&self, b: T) -> T;
		fn except(&self, b: T) -> T;
	}

	impl<T:Ord+Eq+Clone> BooleanAlgebra<Group<T>> for Group<T> {
		fn or(&self, b: Group<T>) -> Group<T> {
			Group { members: self::or(self.members, b.members) }
		}
		fn and(&self, b: Group<T>) -> Group<T> {
			Group { members: self::and(self.members, b.members) }
		}
		fn except(&self, b: Group<T>) -> Group<T> {
			Group { members: self::except(self.members, b.members) }
		}
	}

	pub trait IndexOf<T> {
		fn index_of(&self, item: T) -> int;
	}

	impl<T:Ord+Eq> IndexOf<T> for Group<T> {
		fn index_of(&self, item: T) -> int {
			self::index_of(self.members, item)
		}
	}

	impl<T:Ord+Eq+Clone> Add<Group<T>, Group<T>> for Group<T> {
		fn add(&self, rhs: &Group<T>) -> Group<T> {
			Group { members: or(self.members, rhs.members) }
		}
	}

	impl<T:Ord+Eq+Clone> Mul<Group<T>, Group<T>> for Group<T> {
		fn mul(&self, rhs: &Group<T>) -> Group<T> {
			Group { members: and(self.members, rhs.members) }
		}
	}

	impl<T:Ord+Eq+Clone> Sub<Group<T>, Group<T>> for Group<T> {
		fn sub(&self, rhs: &Group<T>) -> Group<T> {
			Group { members: except(self.members, rhs.members) }
		}
	}

	#[test]
	fn test_or_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4]} ;
		let c = a + b;
		assert_eq!(c.members, ~[1, 2, 3, 4]);
		let d = a.or(b);
		assert_eq!(d.members, ~[1, 2, 3, 4]);
	}

	#[test]
	fn test_and_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4] };
		let c = a * b;
		assert_eq!(c.members, ~[2, 3]);
		let d = a.and(b);
		assert_eq!(d.members, ~[2, 3]);
	}

	#[test]
	fn test_except_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4] };
		let c = a - b;
		assert_eq!(c.members, ~[1]);
		let d = a.except(b);
		assert_eq!(d.members, ~[1]);
	}

	#[test]
	fn test_advanced() {
		let a = Group { members: ~[1, 5, 8] };
		let b = Group { members: ~[0, 1, 5] };
		let c = Group { members: ~[0, 1, 5, 8] };
		let d = a + b - c;
		assert_eq!(d.members, ~[]);
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

	#[test]
	fn test_and() {
		let a: ~[int] = ~[1, 2, 3];
		let b: ~[int] = ~[2, 3, 4];
		let c = and(a, b);
		assert_eq!(c, ~[2, 3]);
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

	#[test]
	fn test_exclude() {
		let a: ~[int] = ~[1, 2, 3];
		let b: ~[int] = ~[2, 3, 4];
		let c = except(a, b);
		assert_eq!(c, ~[1]);
	}

	fn index_of<T:Ord+Eq>(list: &[T], item: T) -> int {
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

			return i;
		}

		-(low + 1)
	}

	#[test]
	fn test_index_of() {
		let a: ~[int] = ~[1, 5, 10];
		let b = index_of(a, 5);
		assert_eq!(b, 1);
	}

	#[test]
	fn test_index_of_group() {
		let a = Group { members: ~[1, 5, 10] };
		let b = a.index_of(5);
		assert_eq!(b, 1);
	}

	#[test]
	fn test2_index_of() {
		let a: ~[int] = ~[1, 2, 10];
		let b = index_of(a, 0);
		assert_eq!(b, -1);
	}

	#[test]
	fn test2_index_of_group() {
		let a = Group { members: ~[1, 2, 10] };
		let b = a.index_of(0);
		assert_eq!(b, -1);
	}
}

