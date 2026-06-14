use std::io;
use rand::Rng;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};

//
fn roll_percice(max: i32) -> i32 {
    // rand::thread_rng().gen_range(1..=max);
    6
}

fn innit() {

}

fn roll() -> i32 {
    // rand::thread_rng().gen_range(1..=6);
    6
}

fn prompt_text(title :String, def :String) -> bool { // Will need to be replaced with actual interface
        println!("{}", title);
        println!("{}", def);
        println!("Y/N");

        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");
        return (ans.trim().to_uppercase() == "Y");

        // ? return ((io::stdin()
        // ?     .read_line()
        // ?     .expect("Failed to read line"))
        // ?     .trim().to_uppercase() == "Y");
    }

    fn prompt_options(title :String, def :String, options: Vec<String>) -> bool { // Will need to be replaced with actual interface
        println!("{}", title);
        println!("{}", def);
        for i in 0..options.len() {
            print!("{}", i);
            print!(": {}", options[i]);
        }
        println!("Y/N");

        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");
        return (ans.trim().to_uppercase() == "Y");

        // ? return ((io::stdin()
        // ?     .read_line()
        // ?     .expect("Failed to read line"))
        // ?     .trim().to_uppercase() == "Y");
    }

struct Character {
    Name: String,
    x: i32,
    y: i32
}

impl EventHandler {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Game logic here
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Drawing code here
        Ok(())
    }
}

fn main() -> GameResult {
    // event::run(ContextBuilder::new("my_game", "author")
    //     .build()?, state)
}