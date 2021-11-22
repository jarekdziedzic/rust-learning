mod vectors;
mod strings;
mod hashmaps;
mod exercises;

use crate::vectors::*;
use crate::strings::*;
use crate::hashmaps::*;

fn main() {
    vector_create();
    vector_refs();

    str_concat();

    insert_example();

    exercises::statistics();
}
