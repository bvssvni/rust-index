rust-index
==========

A Boolean algebra library for indexing.  
MIT license

##Introduction

Sometimes it is useful to treat vectors as sets.
If the vector is sorted, one can use an algorithm that takes the least element from each and produces a set.
The worst case for this is O(N), but it is fast for other reasons:

1. The data is layed out sequential in memory, which makes it predictable for code optimization.  
2. Few heap allocations are required.  

###Import Namespace

    mod index;

###Example 1 - Sorted strs

    let a = ~[~"apes", ~"banana", ~"monkey"];
    let b = ~[~"banana", ~"monkey", ~"snakes"];
    let c = index::or(a, b);
    assert_eq!(c, ~[~"apes", ~"banana", ~"monkey", ~"snakes"]);

###Example 2 - Sorted ints

    let a = ~[1, 2, 3];
    let b = ~[2, 3, 4];
    let c = ~[1, 4];
    let d = index::except(index::and(a, b), c);
    assert_eq!(d, ~[2, 3]);

###Example 3 - Found

    let a = ~[1, 5, 10];
    let b = index::index_of(a, &5);
    assert_eq!(b, ::index::Found(1));

###Example 4 - FoundLarger

    let a = ~[1, 2, 10];
    let b = index::index_of(a, &0);
    assert_eq!(b, ::index::FoundLarger(0));

###To run unit tests:

    make test
    
###Requirements

* The index data must be sorted in ascending order.  
* The index type must implement std::cmp::Ord, std::cmp::Eq and std::clone::Clone.
