rust-index
==========

A Boolean algebra library on sorted vectors of ints.
MIT license

##Why?

A sorted vector of ints can represent a selection of another vector.  
With Boolean algebra you can combine the sorted vectors with the set operations:

    index::or (union)
    index::and (intersect)
    index::except (difference)

###Example

    let waiting = // get the customers waiting
    let bored = // get the bored customers
    let recently_arrived = // get the recently arrived customers
    let waiting_and_bored = index::and(waiting, bored);
    let impatient = index::except(waiting_and_bored, recently_arrived);

To run unit tests:

    make test
    
