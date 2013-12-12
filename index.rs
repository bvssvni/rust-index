
mod index {
	pub fn or(a: ~[int], b: ~[int]) -> ~[int] {
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

	fn and(a: ~[int], b: ~[int]) -> ~[int] {
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

	fn except(a: ~[int], b: ~[int]) -> ~[int] {
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
}

