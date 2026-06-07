
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
    ownership: PeiceType,
    level: i32 // 0-5: 0==none, 1-4==houses, 5== Hotel
}

struct Card {
    Title: String,
    command: Command,
    Def: String
}
// struct 

// Enum
enum PeiceType {
    Red,
    Blue,
    Green,
    white
}

enum Command {
    None,
    Jail,
    Move,
    MoveNoGO, // Move to a position without passing go
    MoneyManipulate,
    Morgage
}

// functions

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