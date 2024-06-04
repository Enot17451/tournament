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

struct Team {
    name: String,
    mp: i32,
    w: i32,
    d: i32,
    l: i32,
    p: i32,
}

impl Team {}

struct Teams {
    teams: Vec<Team>,
}

impl Teams {
    fn getResult() -> String {
        todo!()
    }
}

pub fn tally(match_results: &str) -> String {
    let strings: Vec<&str> = match_results.split('\n').collect();
    for s in strings {
        let parts: Vec<&str> = s.split(';').collect();
        let t1: &str = parts[0];
        let t2: &str = parts[1];
        let result = parts[2];
        println!("{} : {} - {}", t1, t2, result);
    }
    let table = String::from("Team                           | MP |  W |  D |  L |  P");

    return table;
}

fn fetchResult(result: &str) {}

#[test]
fn a_win_is_three_points_a_loss_is_zero_points() {
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
        "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
        "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
    ]
        .join("\n");
    assert_eq!(output, expected);
}
