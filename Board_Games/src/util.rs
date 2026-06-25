use std::io;
use rand::Rng;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};

//
fn roll_percice(max: i32) -> i32 {
    return (rand::thread_rng().gen_range(1..=max) as i32);
}

fn innit() {

}

pub fn roll() -> i32 {
    return (rand::thread_rng().gen_range(1..=6) as i32);
}

pub fn murder(k: &mut Vec<i32>, x: i32) {
    k.retain(|&item| item != x);
}

pub fn prompt(title :String, def :String) { // Will need to be replaced with actual interface
    println!("{}", title);
    println!("{}", def);

    // ok button
}

pub fn prompt_yn(title :String, def :String) -> bool { // Will need to be replaced with actual interface
    println!("{}", title);
    println!("{}", def);
    println!("Y/N");

    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");
    return ans.trim().to_uppercase() == "Y";

    // ? return ((io::stdin()
    // ?     .read_line()
    // ?     .expect("Failed to read line"))
    // ?     .trim().to_uppercase() == "Y");
}

pub fn prompt_options(title :String, def :String, options: Vec<String>) -> i32 { // Will need to be replaced with actual interface
    println!("{}", title);
    println!("{}", def);
    for i in 0..options.len() {
        print!("{}", i);
        print!(": {}", options[i]);
        println!(".");
    }
    println!("Pick a number from 0 to {}", options.len());

    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");
    return (ans.trim()).parse::<i32>().unwrap();
}

pub struct Character {
    pub(crate) name: String,
    pub(crate) x: i32,
    pub(crate) y: i32
}

struct MyGame {
    
}

impl dyn EventHandler {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Game logic here
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Drawing code here
        Ok(())
    }
}

fn run_game() -> GameResult {
   // Uncommented and fixed the ggez event loop
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("my_game", "author")
        .build()?;
    let mut my_game = MyGame {};
    event::run(ctx, event_loop, my_game)
}