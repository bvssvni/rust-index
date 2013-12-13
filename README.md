rust-index
==========

A Boolean algebra library for index/group operations.  
MIT license

###Example

    let humans = Group { members: ~[4, 5] };
    let animals = Group { members: ~[1, 2, 3] };
    let in_the_jungle = Group { members: ~[2, 3, 4] };
    // Create new groups using Boolean algebra.
    let animals_in_the_jungle = animals * in_the_jungle;
    let animals_outside_the_jungle = animals - in_the_jungle;
    let humans_or_animals = humans + animals;

To run unit tests:

    make test
    
