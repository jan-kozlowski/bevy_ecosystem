#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![deny(clippy::unwrap_used, clippy::all)]

use bevy::prelude::*;
mod map;
use map::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Map)
        .run();
}
