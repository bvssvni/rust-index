rust-index
==========

A Boolean algebra library for indexing.  
MIT license

###Import Namespace

    mod index;
    use index::Index;

###Example 1 - Sorted strs

    let a = Index::new( ~[~"apes", ~"banana", ~"monkey"] );
    let b = Index::new( ~[~"banana", ~"monkey", ~"snakes"] );
    let c = a + b;
    assert_eq!(c.to_vec(), ~[~"apes", ~"banana", ~"monkey", ~"snakes"]);

###Example 2 - Sorted ints

    let a = Index::new( ~[1, 2, 3] );
    let b = Index::new( ~[2, 3, 4] );
    let c = Index::new( ~[1, 4] );
    let d = a * (b - c);
    assert_eq!(d.to_vec(), ~[2, 3]);

###Example 3 - Found

    let a = Index::new( ~[1, 5, 10] );
    let b = a.index_of(5);
    assert_eq!(b, ::index::Found(1));

###Example 4 - FoundLarger

    let a = Index::new( ~[1, 2, 10] );
    let b = a.index_of(0);
    assert_eq!(b, ::index::FoundLarger(0));

###To run unit tests:

    make test
    
###Requirements

* The index data must be sorted in ascending order.  
* The index type must implement std::cmp::Ord, std::cmp::Eq and std::clone::Clone.

###Subtraction

Always put subtractions at the end of the expression.  
This is necessary to get correct precedence order.

    let a = b - c + e; // WRONG: c will not subtract e.
    let a = b + e - c; // RIGHT: c will subtract e.
