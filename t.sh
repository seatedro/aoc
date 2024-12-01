#!/bin/bash

for i in {2..25}; do
    echo 'pub fn solve() {
    println!("day'$i' code");
}' > src/day$i.rs
done
