#![allow(warnings)]

struct Team<'a> {
    name: &'a str,
    games: i32,
    win: i32,
    lose: i32,
    draw: i32,
    points: i32,
}

impl<'a> Team<'a> {
    fn new(_name: &'a str) -> Self {
        Self {
            name: _name,
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

struct Table<'a> {
    teams: Vec<Team<'a>>,
}

impl<'a> Table<'a> {
    fn new() -> Self {
        Self {
            teams: Vec::new()
        }
    }

    fn win(&self,team:&str){
        
    }

    fn lose(&self,team:&str){
        
    }

    fn draw(&self,team:&str){
        
    }

    fn set(&mut self, s: &'a str) {
        let team: &mut Team;
        let v:Vec<&str> = s.split(';').collect();
        println!("{:?}", v);
        match v[2] {
            "win" => {
                self.win(v[0]);
                self.lose(v[1]);
            },
            "draw" => {
                self.draw(v[0]);
                self.draw(v[1]);
            },
            "loss" => {
                self.lose(v[0]);
                self.win(v[1]);
            },
            _ => {}
        }
    }

    fn print(&self) -> String {
        let table = String::from("Team                           | MP |  W |  D |  L |  P");
        return "end".to_string();
    }
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
    let mut table = Table::new();
    let v: Vec<&str> = match_results.split('\n').collect();
    for s in &v {
        table.set(s);
    }
    return table.print();
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
