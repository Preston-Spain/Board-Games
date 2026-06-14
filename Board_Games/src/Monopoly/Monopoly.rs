use std::collections::HashMap;
use std::io;

use Util::util;

// Structs
    struct GamePiece {
        character: util::Character,
        money: i32, // Starting amount
        piece_type: PieceType,
        inventory: Vec<Card>,
        // owner: Vec<Tile>,
        last_roll: [i32; 2],
        snake_eyes_watch: i32,
        alive: bool
    }

    struct Effect {
        command: Option<Command>,
        command_word: String,
        command_num: i32,
    }

    struct Tile {
        title: String,
        cost: i32,
        description: String,
        eff: Effect,
        ownership: Option<PieceType>,
        level: i32 // 0-5: 0==none, 1-4==houses, 5== Hotel
    }

    struct Card {
        title: String,
        def: String,
        eff: Effect
    }

// Enum
    enum PieceType {
        Red,
        Blue,
        Green,
        White
    }

    enum Command {
        Auction,
        Buy,
        Jail, // just move to square 41 (40)
        Move,
        MoneyManipulate,
        Morgage,
        None,
        Roll
    }

// functions
    fn execute_command(character: &mut GamePiece, b: &HashMap <i32, Tile>, command: &Command, num: i32) -> i32 {
        match command {
            Command::Jail => {move_char(character, b, true, 41); return 0;},
            // Command::Morgage => {},
            Command::Move => {move_char(character, b, false, num); return 0;},
            Command::MoneyManipulate => {
                character.money += num;
                if !(num < 0) {
                    // command 
                }
            }//,
            // _ => {return 0;}
        }
    } // Jail, money

    fn execute_command_with_tile(character: &mut GamePiece, b: &HashMap <i32, Tile>, command: &Command, tileID: i32, num: i32) {
        match command {
            Command::Auction => {},
            Command::Buy => {
                let cost = tile.cost;
                if character.money < cost {
                    execute_command_with_tile(character, b, Command::Auction, tile, num);
                } else {
                    
                }
            },
            _ => {}
        }
    } // buy, auction

    fn morgage () {

    }

    fn game_Check(players: &mut Vec<GamePiece>) -> bool {
        let mut k: i32 = 0;
        let len = players.len();
        for i in 0..len {
            let player = &mut players[i];
            if player.money > 0 {
                k += 1;
            }
        }
        if k>1 {return true} else {return false}
    }  // TODO add morgaging in the case of a player having funds without having them currently avalible

    fn game_Clear(players: &mut Vec<GamePiece>) -> GamePiece {
        let len = players.len();
        let maxPlayer: GamePiece;
        let max: i32;
        for i in 0..len {
            let player = &mut players[i];
            if player.money > 0 {
                return player;
            } else {
                if player.money > max {
                    max = player.money;
                    maxPlayer = player;
                }
            }
        }
        return maxPlayer;
    }

    fn move_char(players: &mut GamePiece, b: &HashMap <i32, Tile>, NoGo: bool, num: i32) {
        let newPos = if players.character.x == 41 {11 + num} else {players.character.Position.X + num};
        if newPos > 40 {
            let newPos = num - 41;
            if !NoGo {
                let m: i32 = execute_command(players, b, Command::MoneyManipulate, 200);
            }
        }
        players.character.x = newPos;
    }

fn main() {
    let mut board = HashMap::new();
    // let mut characterEx: ;
    let players: Vec<GamePiece> = vec![
        GamePiece::new(PieceType::Red, character1),
        GamePiece::new(PieceType::Blue, character2),
        GamePiece::new(PieceType::Green, character3),
        GamePiece::new(PieceType::White, character4),
    ];
    
    loop {
        for i in 0..players.len() {
            let player = players[i];
            if !game_Check(players) {
                game_Clear(players);
            } else {

                let die: Vec<i32> = [util::roll(), util::roll()]; // ? [roll(); 2]

                if player.character.x == 41 {
                    // Jail
                    if player.
                    util::prompt("You're in Jail", "fgfg");
                }
            }
        }
    }
}
