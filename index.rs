
mod index {
	pub fn or(a: &[int], b: &[int]) -> ~[int] {
		let mut i = 0;
		let mut j = 1;
		let an = a.len();
		let bn = b.len();
		let mut result: ~[int] = ~[];
		while i < an || j < an {
			let av = 
				if i < an {
					a[i]
				} else {
					::std::int::max_value
				};
			let bv = 
				if j < bn {
					b[j]
				} else {
					::std::int::max_value
				};
			let min = 
				if av < bv {
					 av
				} else {
					bv
				};
			if min == av {
				i += 1;
			}
			if min == bv {
				j += 1;
			}

			result.push(min);
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

	pub struct Group {
		members: ~[int],
	}

	impl Add<Group, Group> for Group {
		fn add(&self, rhs: &Group) -> Group {
			Group { members: or(self.members, rhs.members) }
		}
	}

	impl Mul<Group, Group> for Group {
		fn mul(&self, rhs: &Group) -> Group {
			Group { members: and(self.members, rhs.members) }
		}
	}

	impl Sub<Group, Group> for Group {
		fn sub(&self, rhs: &Group) -> Group {
			Group { members: except(self.members, rhs.members) }
		}
	}

	#[test]
	fn test_or_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4]} ;
		let c = a + b;
		assert_eq!(c.members, ~[1, 2, 3, 4]);
	}

	#[test]
	fn test_and_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4] };
		let c = a * b;
		assert_eq!(c.members, ~[2, 3]);
	}

	#[test]
	fn test_except_operator() {
		let a = Group { members: ~[1, 2, 3] };
		let b = Group { members: ~[2, 3, 4] };
		let c = a - b;
		assert_eq!(c.members, ~[1]);
	}

	#[test]
	fn test_advanced() {
		let a = Group { members: ~[1, 5, 8] };
		let b = Group { members: ~[0, 1, 5] };
		let c = Group { members: ~[0, 1, 5, 8] };
		let d = a + b - c;
		assert_eq!(d.members, ~[]);
	}

	fn and(a: &[int], b: &[int]) -> ~[int] {
		let mut i = 0;
		let mut j = 0;
		let an = a.len();
		let bn = b.len();
		let mut result: ~[int] = ~[];
		while i < an && j < bn {
			let av = a[i];
			let bv = b[j];
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
				result.push(min);
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

	fn except(a: &[int], b: &[int]) -> ~[int] {
		let mut i = 0;
		let mut j = 0;
		let an = a.len();
		let bn = b.len();
		let mut result: ~[int] = ~[];
		while i < an {
			let av = a[i];
			let bv =
				if j < bn {
					b[j]
				} else {
					::std::int::max_value
				};
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
			if isAv && !isBv {
				result.push(min);
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

	fn indexOf(list: &[int], item: int) -> int {
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
	fn test_indexOf() {
		let a: ~[int] = ~[1, 5, 10];
		let b = indexOf(a, 5);
		assert_eq!(b, 1);
	}

	#[test]
	fn test2_indexOf() {
		let a: ~[int] = ~[1, 2, 10];
		let b = indexOf(a, 0);
		assert_eq!(b, -1);
	}
}

