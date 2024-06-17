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
    fn new(teamName: &'a str) -> Self {
        Self {
            name: teamName,
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

    pub fn add(&mut self, teamName: &'a str, result: &str) {
        let mut existTeam: Option<&mut Team> = None;
        for team in &mut self.teams {
            if team.name == teamName {
                existTeam = Some(team);
                break;
            }
        }
        match existTeam {
            Some(t) => {
                match result {
                    "win" => t.win += 1,
                    "lose" => t.lose += 1,
                    _ => t.draw += 1
                }
            }
            None => {
                self.teams.push(Team::new(teamName));
                let t = self.teams.last_mut().unwrap();
                match result {
                    "win" => t.win += 1,
                    "lose" => t.lose += 1,
                    _ => t.draw += 1
                }
            }
        }
    }


    pub fn print(&self) -> String {
        let out = String::from("Team                           | MP |  W |  D |  L |  P");
        return out;
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

fn resultForSecondTeam(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        _ => "draw"
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = Table::new();
    let results: Vec<&str> = match_results.split('\n').collect();
    for result in results {
        let line: Vec<&str> = result.split(';').collect();
        table.add(line[0], line[2]);
        table.add(line[0], resultForSecondTeam(line[2]));
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
