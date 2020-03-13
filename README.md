# tag-vec
A rust crate for a vector of tags

## Examples
```rust
// Create a vector of tags with the tags 
// "healthy"(healthy food), "junk"(junk food) and "veg"(vegetarian)
let foods: TagVec<String> = TagVec::new();
foods.push(vec!["healthy", "veg"]);
foods.push(vec!["junk"]);
foods.push(vec!["healthy", "veg"]);
foods.push(vec!["junk", "veg"]);
foods.push(vec!["junk"]);

// We don't want healthy or vegetarian food,
// we're not plebs who care about the planet, are we!?
use expressions::*;
let mut cool_foods = tags.query(and(not(tag("veg")), not(tag("healthy"))));
for food in cool_foods {
	println!("{:?}", food);
}

// Expected output: (It only outputs the indices of the matching elements, 
// this is a vec, so the order matters)
// 1
// 3
```

## Features to come:
* Changing the tags of an element
* Removing elements(ugh, this is going to be an expensive operation)
* Popping elements(not expensive at all!)
