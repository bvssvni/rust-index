extern mod index;

use index::Index;

#[test]
fn test_or_operator() {
	let a = Index::new( ~[1, 2, 3] );
	let b = Index::new( ~[2, 3, 4] );
	let c = a + b;
	assert_eq!(c.to_vec(), ~[1, 2, 3, 4]);
}

#[test]
fn test_and_operator() {
	let a = Index::new( ~[1, 2, 3] );
	let b = Index::new( ~[2, 3, 4] );
	let c = a * b;
	assert_eq!(c.to_vec(), ~[2, 3]);
}

#[test]
fn test_except_operator() {
	let a = Index::new( ~[1, 2, 3] );
	let b = Index::new( ~[2, 3, 4] );
	let c = a - b;
	assert_eq!(c.to_vec(), ~[1]);
}

#[test]
fn test_advanced() {
	let a = Index::new( ~[1, 5, 8] );
	let b = Index::new( ~[0, 1, 5] );
	let c = Index::new( ~[0, 1, 5, 8] );
	let d = a + b - c;
	assert_eq!(d.to_vec(), ~[]);
}

#[test]
fn test_index_of_Index() {
	let a = Index::new( ~[1, 5, 10] );
	let b = a.index_of(5);
	assert_eq!(b, ::index::Found(1));
}

#[test]
fn test2_index_of_Index() {
	let a = Index::new( ~[1, 2, 10] );
	let b = a.index_of(0);
	assert_eq!(b, ::index::FoundLarger(0));
}

#[test]
fn test_strs() {
	let a = Index::new( ~[~"apes", ~"banana", ~"monkey"] );
	let b = Index::new( ~[~"banana", ~"monkey", ~"snakes"] );
	let c = a + b;
	assert_eq!(c.to_vec(), 
		~[~"apes", ~"banana", ~"monkey", ~"snakes"]);
}

#[test]
fn test_example_2() {
	let a = Index::new( ~[1, 2, 3] );
	let b = Index::new( ~[2, 3, 4] );
	let c = Index::new( ~[1, 4] );
	let d = a * (b - c);
	assert_eq!(d.to_vec(), ~[2, 3]);
}

#[test]
fn test_new_check_order() {
	assert!(!Index::check_order( &~[1, 3, 2] ));
}

#[test]
fn test_new_check_order_empty() {
	let a: ~[int] = ~[];
	assert!(Index::check_order(&a));
}

#[test]
fn test_new_check_order_sort_retry() {
	let mut a = ~[1, 3, 2];
	let ordered = Index::check_order(&a);	
	assert!(!ordered);	
	if !ordered {
		a.swap(1, 2);
	}

	let ordered = Index::check_order(&a);
	assert!(ordered);
}

