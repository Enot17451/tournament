#![allow(warnings)]

struct Team<'a> {
    name:&'a str,
    games: i32,
    win: i32,
    draw: i32,
    lose: i32,
    points: i32,
}

impl<'a> Team<'a> {
    fn new(_name:&'a str) -> Self {
        Self {
            name:_name,
            games: 0,
            win: 0,
            draw: 0,
            lose: 0,
            points: 0,
        }
    }

    fn getAll(&mut self) {
        self.games = self.win + self.draw + self.lose;
        self.points = self.win * 3 + self.draw;
    }
}

fn addTeam(name:&str,result: &str,table:&mut Vec<Team>){

}

fn print(table:&Vec<Team>) -> String {
    let out = String::from("Team                           | MP |  W |  D |  L |  P");
    return out;
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
    let matchStrings: Vec<&str> = match_results.split('\n').collect();
    let mut table = Vec::with_capacity(matchStrings.len()/2);
    for ms in matchStrings {
        let string: Vec<&str> = ms.split(";").collect();
    }
    return print(&table);
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
