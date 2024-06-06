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
    let mut teams: HashMap<&str, [i32; 3]> = HashMap::new();
    let v: Vec<&str> = match_results.split('\n').collect();
    for s in &v {
        let parts: Vec<&str> = s.split(';').collect();
        match parts[2] {
            "win" => {
                let mut ar = teams.entry(parts[0]).or_insert([0, 0, 0]);
                ar[0] += 1;
                ar = teams.entry(parts[1]).or_insert([0, 0, 0]);
                ar[2] += 1;
            }
            "lose" => {
                let mut ar = teams.entry(parts[0]).or_insert([0, 0, 0]);
                ar[2] += 1;
                ar = teams.entry(parts[1]).or_insert([0, 0, 0]);
                ar[0] += 1;
            }
            "draw" => {
                let mut ar = teams.entry(parts[0]).or_insert([0, 0, 0]);
                ar[1] += 1;
                ar = teams.entry(parts[1]).or_insert([0, 0, 0]);
                ar[1] += 1;
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
