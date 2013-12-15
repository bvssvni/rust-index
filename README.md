rust-index
==========

A Boolean algebra library for indexing.  
MIT license

###Example 1

    let a = ::index::new( ~[~"apes", ~"banana", ~"monkey"] );
    let b = ::index::new( ~[~"banana", ~"monkey", ~"snakes"] );
    let c = a + b;
    assert_eq!(::index::to_vec(c), ~[~"apes", ~"banana", ~"monkey", ~"snakes"]);

###Example 2

    let a = ::index::new( ~[1, 2, 3] );
    let b = ::index::new( ~[2, 3, 4] );
    let c = ::index::new( ~[1, 4] );
    let d = a * (b - c);
    assert_eq!(::index::to_vec(d), ~[2, 3]);

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
