extern mod index;

#[test]
fn test_or_operator() {
	let a = ~[1, 2, 3];
	let b = ~[2, 3, 4];
	let c = index::or(a, b);
	assert_eq!(c, ~[1, 2, 3, 4]);
}

#[test]
fn test_and_operator() {
	let a = ~[1, 2, 3];
	let b = ~[2, 3, 4];
	let c = index::and(a, b);
	assert_eq!(c, ~[2, 3]);
}

#[test]
fn test_except_operator() {
	let a = ~[1, 2, 3];
	let b = ~[2, 3, 4];
	let c = index::except(a, b);
	assert_eq!(c, ~[1]);
}

#[test]
fn test_advanced() {
	let a = ~[1, 5, 8];
	let b = ~[0, 1, 5];
	let c = ~[0, 1, 5, 8];
	let d = index::except(index::or(a, b), c);
	assert_eq!(d, ~[]);
}

#[test]
fn test_index_of() {
	let a = ~[1, 5, 10];
	let b = index::index_of(a, &5);
	assert_eq!(b, ::index::Found(1));
}

#[test]
fn test2_index_of() {
	let a = ~[1, 2, 10];
	let b = index::index_of(a, &0);
	assert_eq!(b, ::index::FoundLarger(0));
}

#[test]
fn test3_index_of() {
	let a = ~[];
	let b = index::index_of(a, &5);
	assert_eq!(b, ::index::FoundLarger(0));
}

#[test]
fn test_strs() {
	let a = ~[~"apes", ~"banana", ~"monkey"];
	let b = ~[~"banana", ~"monkey", ~"snakes"];
	let c = index::or(a, b);
	assert_eq!(c, 
		~[~"apes", ~"banana", ~"monkey", ~"snakes"]);
}

#[test]
fn test_example_2() {
	let a = ~[1, 2, 3];
	let b = ~[2, 3, 4];
	let c = ~[1, 4];
	let d = index::except(index::and(a, b), c);
	assert_eq!(d, ~[2, 3]);
}

#[test]
fn test_new_check_order() {
	assert!(!index::check_order( &~[1, 3, 2] ));
}

#[test]
fn test_new_check_order_empty() {
	let a: ~[int] = ~[];
	assert!(index::check_order(&a));
}

#[test]
fn test_new_check_order_sort_retry() {
	let mut a = ~[1, 3, 2];
	let ordered = index::check_order(&a);	
	assert!(!ordered);	
	if !ordered {
		a.swap(1, 2);
	}

	let ordered = index::check_order(&a);
	assert!(ordered);
}

#[test]
fn test_insert() {
	let mut a = ~[1, 2, 3];
	index::insert(&mut a, 4);
	assert_eq!(a, ~[1, 2, 3, 4]);
}

#[test]
fn test2_insert() {
	let mut a = ~[1, 3, 4];
	index::insert(&mut a, 2);
	assert_eq!(a, ~[1, 2, 3, 4]);
}

