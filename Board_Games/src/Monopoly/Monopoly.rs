use std::collections::HashMap;

// variables
    // let mut Board = HashMap::new();
// Structs
    struct GamePeice {
        char: Character,
        money: i32 = 1500, // Starting amount
        peiceType: Peice,
        inventory: i32 = [],
        lastRoll: i32 = []
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
        Auction,
        Buy,
        // Jail, // just move to square 41 (40)
        Move,
        // MoveNoGO, // Move to a position without passing go
        MoneyManipulate,
        Morgage,
        None,
        Roll
    }

// functions
    fn innit() {

    }

    fn prompt(def :String, str :String) -> bool { // Will need to be replaced

    }

    fn command(character: &mut GamePeice, command: &Command, tile: Tile, num: i32) {
        match Command {
            Command::Auction => {},
            Command::Buy => {},
            Command::Move => {moveChar(character,MoneyManipulate,tile)},
            Command::MoneyManipulate => {},
            Command::Morgage => {},
            Command::None => {},
            Command::Roll => {}
        }
    }

    fn gameCheck() -> bool {
        return ;
    }

    fn rollDie() -> [i32; 2] {
        guy = [1,2];


        return guy
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

    fn moveChar(character: &mut GamePeice, tile: Tile, num: i32) {
        if (character::char::x == 41) { // jail logic
            if (true) {
                //
            } else {
                //
            }
        } else {
            if (tile::commandWord == "Jail") {
                ;
            } else if (tile::commandWord == "NoGo") {
                //
            } else { // 1 - 40 (0 - 39)
                newPos = character::char::Position::X + num;
                if (newPos > 40) {
                    newPos = num - 41;
                    command(character,MoneyManipulate,tile,200);
                }
                character::char::x = newPos;
            }
        }
    }

    

fn Main() {
    // 
    while () {}
}