use std::io;
use rand::Rng;

//
fn roll(max: i32) -> i32 {
    return rand::thread_rng().gen_range(1..=max);
}

fn roll() -> i32 {
    return rand::thread_rng().gen_range(1..=6);
}

struct Character {
    Name: String,
    x: i32,
    y: i32
}