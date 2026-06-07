use std::collections::HashMap;

// variables
    // let mut Board = HashMap::new();
// Structs
    struct GamePeice {
    char: Character,
    money: i32,
    peiceType: Peice
    }

    struct Tile {
    id: i32,
    title: String,
    cost: i32,
    description: String,
    command: Command,
    commandWord: String,
    commandNum: i32,
    ownership: PeiceType,
    level: i32 // 0-5: 0==none, 1-4==houses, 5== Hotel
    }

    struct Card {
    Title: String,
    command: Command,
    commandWord: String,
    commandNum: i32,
    Def: String
    }

// Enum
    enum PeiceType {
        Red,
        Blue,
        Green,
        white
    }

    enum Command {
        None,
        // Jail, // just move to square 41 (40)
        Move,
        // MoveNoGO, // Move to a position without passing go
        MoneyManipulate,
        Morgage
    }

// functions
    fn innit() {

    }

    fn command(character: &mut GamePeice, command: &Command, word: String, num: i32) {
        match Command {
        Command::None => break,
        Command::Move => {
            if (word == "Jail") {
                ;
            } else if (work == "NoGo") {
                //
            } else { // 1 - 40 (0 - 39)

            }
        },
        Command::MoneyManipulate => {
            
        },
        Command::Morgage => println!{
            
        },
    }
    }

    fn gameCheck() -> bool {
        return ;
    }

    fn movePeice(spaces: i32) {

    }

    fn auction(tile: Tile, character: GamePeice) {
        
    }

    fn buy(tile: Tile, character: GamePeice) {
        cost = tile.cost;
        if (character.money < cost) {
            auction(tile)
        } else {
            
        }
    }

fn Main() {
    // 
    while () {}
}