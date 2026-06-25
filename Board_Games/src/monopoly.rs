use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::{self, Character};

// Structs
    struct GamePiece {
        character: util::Character,
        money: i32,
        piece_type: PieceType,
        // inventory: Vec<Card>,
        inventory: i32,
        jail_stay: i32,
        // owner: Vec<Tile>,
        last_roll: [i32; 2],
        snake_eyes_watch: i32,
        is_alive: bool
    }

    impl GamePiece {
        fn new(piece_type: PieceType, character: util::Character) -> Self {
            GamePiece {
                character,
                money: 1500,
                piece_type,
                inventory: 0,
                jail_stay: 0,
                last_roll: [0, 0],
                snake_eyes_watch: 0,
                is_alive: true
            }
        }
    }

    struct Effect {
        command: Option<Command>,
        command_word: String,
        command_num: i32,
    }

    struct Tile {
        title: String,
        group: i32, // Rail roads are (0)
        cost: i32,
        landing_cost: i32,
        description: String,
        eff: Effect,
        ownership: Option<PieceType>,
        is_morgaged: bool,
        level: i32 // 0-5: 0==none, 1-4==houses, 5== Hotel houses cost 0.3* the cost of a tile
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
        White,
        None
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
            Command::MoneyManipulate => {character.money = num + (character.money as i32);},
            _ => {return 0;}
        }
    } // Jail, money

    fn execute_command_with_tile(character: &mut GamePiece, b: &HashMap <i32, Tile>, command: &Command, tile_ID: i32, num: i32) {
        let tile = b.get(&tile_ID);
        match command {
            Command::Auction => {},
            Command::Buy => {
                let cost = tile.unwrap().cost;
                if character.money < cost {
                    let u: i32 = execute_command_with_tile(character, b, &Command::Auction, b.get(tile).unwrap(), num);
                } else {
                    
                }
            },
            _ => {}
        }
    } // buy, auction

    fn morgage (player: &mut GamePiece, b: &HashMap<i32, Tile>) -> bool {
        let mut IDs: Vec<i32>;
        for i in 0..b.len() {
            if &b.get(&(i as i32)).unwrap().ownership == player.piece_type {
                if !b.get(&(i as i32)).unwrap().is_morgaged {
                    IDs.push(i as i32);
                }
            }
        }
        if IDs.len() > 0 {
            let mut ez_string: Vec<String>;
            for i in 0..IDs.len() {
                ez_string.push(
                        b.get(&(i as i32)).unwrap().title +
                        " " + 
                        &(((b.get(&(i as i32)).unwrap().cost) as f32 *
                            0.7).round()
                        ).to_string()
                    );
            }
            let user_input = util::prompt_options("Morgage".to_string(), "What property would you like to morgage?".to_string(),ez_string);
            player.money =  
                (b.get(&user_input).unwrap().cost as f32 * 0.7 *
                 b.get(&user_input).unwrap().level as f32 * 0.2
                ).round() as i32;
            b.get(&user_input).unwrap().level == 0;
            b.get(&user_input).unwrap().is_morgaged == true;
            return true;
        } else {
            return false;
        }
    }

    fn unmorgage (player: &mut GamePiece, b: &HashMap<i32, Tile>) -> bool {
        let mut IDs: Vec<i32>;
        for i in 0..b.len() {
            let index = i as i32;
            if b.get(&index).unwrap().ownership == player.piece_type {
                if !b.get(&index).unwrap().is_morgaged {
                    IDs.push(index);
                }
            }
        }
        if IDs.len() > 0 {
            let mut ez_string: Vec<String>;
            for i in 0..IDs.len() {
                let index = i as i32;
                ez_string.push(b.get(&index).unwrap().title + 
                                    " " + 
                                    &(((b.get(&index).unwrap().cost as f32 *
                                        0.7).round()
                                    ) as i32
                                    ).to_string());
            }
            // actually deal with it
            let user_input: i32 = util::prompt_options("unmorgage".to_string(), "What property would you like to unmorgage?".to_string(), ez_string);
            player.money = player.money +
                            ((b.get(&user_input).unwrap().cost as f32 *
                                0.7 *
                                b.get(&user_input).unwrap().level as f32 * 
                                0.2
                            ).round() as i32);
            b.get(&user_input).unwrap().is_morgaged == false;
            return true;
        } else {
            return false;
        }
    }

    fn find_tile_ID(tile: Tile, b: &HashMap<i32, Tile>) -> i32 {
        for i in 0..b.len() {
            let index = i as i32;
            if b.get(&index) == tile {
                return index;
            }
        }
    }

    fn find_group(group_num: i32, b: &HashMap<i32, Tile>) -> Vec<i32> {
        let mut IDs: Vec<i32>;
        for i in 0..b.len() {
            let index = i as i32;
            if b.get(&index).unwrap().group == group_num {
                IDs.push(index);
            }
        }
        return IDs;
    }

    fn find_rest_group(id: i32, b: &HashMap<i32, Tile>) -> Vec<i32> {
        let mut IDs: Vec<i32>;
        for i in 0..b.len() {
            if b.get(&(i as i32)).unwrap().group == b.get(&id).unwrap().group {
                if !(i == id.try_into().unwrap()) {IDs.push(i as i32)}
            }
        }
        return IDs;
    }

    fn find_group_owner(group: i32, b: &HashMap<i32, Tile>, players: &mut Vec<GamePiece>) -> GamePiece {
        let grp /*: Vec<i32>*/ = find_group(group, b);
        let mut owner: PieceType = grp.get(0).unwrap().ownership;

        for i in 1..grp.len() {
            if !(owner == grp.get((i as i32)).unwrap().ownership) {return GamePiece::None;}
        }

        for (i, player) in players.iter().enumerate() {
            if owner == player.piece_type {
                return players[i]; // Or return player.clone() if needed
            }
        }
    }

    fn build (player: &mut GamePiece, b: &HashMap<i32, Tile>, players: &mut Vec<GamePiece>) {
        let mut owner: PieceType = player.unwrap().ownership;
        let mut tile_IDs: Vec<i32>;
        let mut group_IDs: Vec<i32>;
        let mut buildable: Vec<i32>;
        let mut build_map: HashMap<i32, i32>;

        for i in 0..b.len() {
            if b.get(&(i as i32)).unwrap().ownership == player.piece_type {
                tile_IDs.push(i as i32); // IDs ???
            }
        }

        for i in 0..tile_IDs.len() {
            if owner == find_group_owner(find_group(i as i32, &b), b, players) {
                group_IDs.push((i as i32));
            }
        }

        if !(group_IDs.len() < 1) { 
            let mut k: Vec<i32>; // TODO combine into one {
            for i in 0..group_IDs.len() {
                let f = find_group(group_IDs[i], b);
                for j in 0..f.len() {
                    k.push(f[i])
                }
            }

            for i in 0..k.len() {
                if (k.get(i as i32).unwrap().group == 0) {
                    util::murder(k, k[i]);
                }
            }

            for i in 0..k.len() {
                let f = find_rest_group(k.get(i as i32));
                if (k.get(i as i32).unwrap().level <= f[0] && k[i].level <= f[1]) {
                    buildable.push(f[i]);
                }
            } // TODO end of former comment }

            for i in 0..buildable.len() {
                // cost*0.3
                if b.get(&buildable[i]).unwrap().cost as f32 * 0.3 > player.money as f32 {
                    buildable.remove(i as i32);
                } else {
                    build_map.insert(buildable[i], b.get(&buildable[i]).unwrap().cost as f32 * 0.3).round();
                }
            }
            
            if build_map.len() < 0 {
                let user_input = util::prompt_options("You can build on these properties".to_string(), "What property would you like to build on?".to_string(), build_map.get(i).title + " " + (build_map.get(i).cost * 0.3).to_string());
                player.money = ((player.money as f32 - (b.get(buildable[user_input]).unwrap().cost as f32 * 0.3)) as i32).round();
                b.get(buildable[user_input]).unwrap().level += 1;
            }
        }
    }

    fn game_Check(players: &mut Vec<GamePiece>) -> bool {
        let mut k: i32 = 0;
        let len = players.len();
        for i in 0..len {
            let player: &mut GamePiece = &mut players[i];
            if player.money > 0 {
                k += 1;
            }
        }
        if k>1 {return true} else {return false}
    }  // TODO add morgaging in the case of a player having funds without having them currently avalible

    fn game_Clear(players: &mut Vec<GamePiece>) -> &mut GamePiece {
        let len: usize = players.len();
        let mut maxPlayer: GamePiece;
        let max: i32;
        for i in 0..len {
            let player: &mut GamePiece = &mut players[i];
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

    fn move_char(players: &mut GamePiece, b: &HashMap <i32, Tile>, no_go: bool, num: i32) {
        let new_pos: i32 = if players.character.x == 41 {11 + num} else {players.character.x + num};
        if new_pos > 40 {
            let new_pos: i32 = num - 41;
            if no_go {
                let _m: i32 = execute_command(players, b, &Command::MoneyManipulate, 200);
            }
        }
        players.character.x = new_pos;
    }

    fn board_innit(b: &HashMap <i32, Tile>) {

        let file = File::open("D:\\Project\\StarDance\\Board-Games\\Board_Games\\src\\csv\\Monopoly.csv");
        let reader = BufReader::new(file);
        
        for (i, current_line) in reader.lines().enumerate() {

            let parts: Vec<&str> = current_line.split(',').collect(); // Order: title, group, cost, landing cost, description, (eff: command, command_word, command_num)
            

            let helper_eff = Effect {
                command: match parts[5] {
                    "Jail" => Some(Command::Jail),
                    "MoneyManipulate" => Some(Command::MoneyManipulate),
                    "None" => Some(Command::None)
                },
                command_word: parts[6].to_owned(),
                command_num: parts[7].parse::<i32>().unwrap_or(0)
            };
            let helper_tile = Tile {
                title: parts[0].to_owned(),
                landing_cost: parts[1].parse::<i32>().unwrap_or(0),
                cost: parts[2].parse::<i32>().unwrap_or(0),
                group: parts[3].parse::<i32>().unwrap_or(0),
                description: parts[4].to_owned(),
                eff: helper_eff,
                ownership: Some(PieceType::None),
                level: 0,
                is_morgaged: false,
            };
            b.insert(i.try_into().unwrap(), helper_tile);
        }
    }

fn main() {
    let mut board = HashMap::new();
    board_innit(&board);
    let chh = util::Character {
        name: "z".to_owned(),
            x: 0,
            y: 0
        };
    // let characterEx: GamePiece = GamePiece {
    //     character: chh,
    //     money: 1500,
    //     piece_type: PieceType::None,
    //     inventory: 0,
    //     jail_stay: 0,
    //     last_roll: [0, 0],
    //     snake_eyes_watch: 0,
    //     is_alive: true
    // };
    let players: Vec<GamePiece> = vec![
        GamePiece::new(PieceType::Red, util::Character),
        GamePiece::new(PieceType::Blue, util::Character),
        GamePiece::new(PieceType::Green, util::Character),
        GamePiece::new(PieceType::White, util::Character),
    ];
    
    loop {
        for i in 0..players.len() {
            let player = players[i];
            if !game_Check(&mut players) {
                let helper = game_Clear(&mut players).piece_type;
                let color: String = match helper{
                    PieceType::Red   => {"Red".to_string()},
                    PieceType::Blue  => {"Blue".to_string()},
                    PieceType::Green => {"Green".to_string()},
                    PieceType::White => {"White".to_string()}
                    PieceType::None  => {"None".to_string()}
                };
                util::prompt(String::from("Game over"), String::from(color.to_string() + &" won!".to_string()));
            } else {
                let die = [util::roll(), util::roll()]; // ? [roll(); 2]

                if player.character.x == 41 {
                    // Jail
                    if player.inventory > 0 {
                        if util::prompt_yn("You're in Jail".to_string(), "Use your \"Get out of jail free card\"?".to_string()) {
                            player.character.x == 11;
                        }
                    }
                    if util::prompt_yn("Pay to leave".to_string(), "$150".to_string()) {
                        execute_command(&mut player, &board, &Command::MoneyManipulate, -150);
                    }
                    util::prompt("You're in Jail".to_string(), "Roll doubles to leave.".to_string());
                }
            }
        }
    }
}
