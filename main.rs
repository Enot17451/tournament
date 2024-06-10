struct Team<'a> {
    name: &'a str,
    games: i32,
    win: i32,
    lose: i32,
    draw: i32,
    points: i32,
}

impl Team {
    fn new(_name: &str) -> Self {
        Self {
            name: _name,
            games: 0,
            win: 0,
            lose: 0,
            draw: 0,
            points: 0,
        }
    }

    fn win(&mut self) {
        self.win += 1;
    }

    fn draw(&mut self) {
        self.draw += 1;
    }

    fn lose(&mut self) {
        self.lose += 1;
    }

    fn getAll(&mut self) {
        self.games = self.win + self.draw + self.lose;
        self.points = self.win * 3 + self.draw;
    }
}

struct Table<'a> {
    teams: Vec<Team<'a>>,
}

impl Table {
    fn new() -> Self {
        Self {
            teams: Vec::new()
        };
    }

    fn findInVec(&self, name: &str) -> Option<usize> {
        for i in 0..self.teams.len() {
            if self.teams[i].name == name {
                return Some(i);
            }
        }
        return None;
    }

    fn set(&mut self, s: &str,isFirst:bool) {
        let team: &mut Team;
        let v: Vec<&str> = s.split(';').collect();
        match self.findInVec(v[0]) {
            Some(index) => {
                team = &mut self.teams[index];
            }
            None => {
                self.teams.push(Team::new(v[0]));
                team = &mut self.teams[self.teams.len() - 1];
            }
        }
        if isFirst{
            match v[2] {
                "win" => team.win(),
                "draw" => team.draw(),
                "loss" => team.lose(),
                _ => {}
            }
        }else{
            match v[2] {
                "win" => team.lose(),
                "draw" => team.draw(),
                "loss" => team.win(),
                _ => {}
            }
        }
    }

    fn print(&self) -> String {
        let table = String::from("Team                           | MP |  W |  D |  L |  P");
        return "".to_string();
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
        table.set(s,true);
        table.set(s,false);
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
