#![allow(warnings)]

use std::collections::HashMap;

struct Team {
    games: i32,
    win: i32,
    lose: i32,
    draw: i32,
    points: i32,
}

impl Team {
    fn new() -> Self {
        Self {
            games: 0,
            win: 0,
            lose: 0,
            draw: 0,
            points: 0,
        }
    }

    fn getAll(&mut self) {
        self.games = self.win + self.draw + self.lose;
        self.points = self.win * 3 + self.draw;
    }
}

fn print() -> String {
    let table = String::from("Team                           | MP |  W |  D |  L |  P");
    return "end".to_string();
}

fn main() {
    let input: &[&str] = &[
        "Allegoric Alaskans;Blithering Badgers;win",
        "Devastating Donkeys;Courageous Californians;draw",
        "Devastating Donkeys;Allegoric Alaskans;win",
        "Courageous Californians;Blithering Badgers;loss",
        "Blithering Badgers;Devastating Donkeys;loss",
        "Allegoric Alaskans;Courageous Californians;win",
    ];
    let input = input.join("\n");
    tally(&input);
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<&str, Team> = HashMap::new();
    let results: Vec<&str> = match_results.split('\n').collect();
    for result in results {
        for res in result.split(";").collect() {
            let o: Option<&mut Team> = table.get_mut(res[0]);
            match o {
                Some(t) => {
                    match res[2] {
                        "win" => t.win += 1,
                        "draw" => t.draw += 1,
                        "loss" => t.lose += 1,
                        _ => {}
                    }
                }
                None => {}
            }
            let o: Option<&mut Team> = table.get_mut(res[1]);
            match o {
                Some(t) => {
                    match res[2] {
                        "win" => t.lose += 1,
                        "draw" => t.draw += 1,
                        "loss" => t.win += 1,
                        _ => {}
                    }
                }
                None => {}
            }
        }
    }
    return print();
}

#[test]
fn typical_input() {
    let input: &[&str] = &[
        "Allegoric Alaskans;Blithering Badgers;win",
        "Devastating Donkeys;Courageous Californians;draw",
        "Devastating Donkeys;Allegoric Alaskans;win",
        "Courageous Californians;Blithering Badgers;loss",
        "Blithering Badgers;Devastating Donkeys;loss",
        "Allegoric Alaskans;Courageous Californians;win",
    ];
    let input = input.join("\n");
    let output = tally(&input);
    let expected = [
        "Team                           | MP |  W |  D |  L |  P",
        "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
        "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
        "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
        "Courageous Californians        |  3 |  0 |  1 |  2 |  1",
    ]
        .join("\n");
    assert_eq!(output, expected);
}
