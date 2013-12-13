rust-index
==========

A Boolean algebra library for indexing.  
MIT license

###Example

    let a = Index { ids: ~[~"apes", ~"banana", ~"monkey"] };
    let b = Index { ids: ~[~"banana", ~"monkey", ~"snakes"] };
    let c = a + b; // take the or/union operation as if the indexes were sets.
    assert_eq!(c.ids, ~[~"apes", ~"banana", ~"monkey", ~"snakes"]);

###To run unit tests:

    make test
    
###Requirements

* The indexes must be sorted in ascending order.  
* The index type must implement std::cmp::Ord, std::cmp::Eq and std::clone::Clone.

###Subtraction

Always put subtractions at the end of the expression.  
This is necessary to get correct precedence order.

    let a = b - c + e; // WRONG: c will not subtract e.
    let a = b + e - c; // RIGHT: c will subtract e.
