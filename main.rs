use std::collections::HashMap;

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
    let mut teams: HashMap<&str, [i32; 4]> = HashMap::new();
    let v: Vec<&str> = match_results.split('\n').collect();
    for s in &v {
        let parts: Vec<&str> = s.split(';').collect();
        let t1: &str = parts[0];
        let t2: &str = parts[1];
        let result = parts[2];
        println!("{} : {} - {}", t1, t2, result);
        match result {
            "win" => {
                if teams.contains_key(t1) {
                    teams[t1][0] += 1;
                    teams[t1][1] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][1] += 1;
                }
                if teams.contains_key(t2) {
                    teams[t2][0] += 1;
                    teams[t2][3] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][1] += 1;
                }
            }
            "lose" => {
                if teams.contains_key(t1) {
                    teams[t1][0] += 1;
                    teams[t1][3] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][3] += 1;
                }
                if teams.contains_key(t2) {
                    teams[t2][0] += 1;
                    teams[t2][1] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][1] += 1;
                }
            }
            "draw" => {
                if teams.contains_key(t1) {
                    teams[t1][0] += 1;
                    teams[t1][2] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][2] += 1;
                }
                if teams.contains_key(t2) {
                    teams[t2][0] += 1;
                    teams[t2][2] += 1;
                } else {
                    teams.insert(t1,[0,0,0,0]);
                    teams[t1][0] += 1;
                    teams[t1][2] += 1;
                }
            }
            _ => {}
        }
    }
    let table = String::from("Team                           | MP |  W |  D |  L |  P");

    return table;
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
