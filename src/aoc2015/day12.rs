use json::{parse, JsonValue};

use crate::day::Day;
use crate::PuzzleResult;

const INPUT: &str = include_str!("input/12");

#[derive(Clone, Copy)]
enum Mode {
    IncludeRed,
    IgnoreRed,
}

fn solve_for(input: &str, mode: Mode) -> i64 {
    let parsed = parse(input).unwrap();

    recurse(&parsed, mode)
}

fn recurse(parsed: &JsonValue, mode: Mode) -> i64 {
    match parsed {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Array(a) => a.iter().map(|v| recurse(v, mode)).sum(),
        JsonValue::Object(o) => {
            match mode {
                Mode::IgnoreRed => {
                    if o.iter().any(|(_, v)| v == "red") {
                        return 0;
                    }
                }
                Mode::IncludeRed => {}
            }

            o.iter().map(|(_, v)| recurse(v, mode)).sum()
        }
        JsonValue::Short(_) => 0,
        _ => unreachable!("unexpected value"),
    }
}

#[test]
fn a_example() {
    assert_eq!(solve_for("[1,2,3]", Mode::IncludeRed), 6);
    assert_eq!(solve_for("[[[3]]]", Mode::IncludeRed), 3);
    assert_eq!(solve_for(r#"{"a":{"b":4},"c":-1}"#, Mode::IncludeRed), 3);
    assert_eq!(solve_for("[]", Mode::IncludeRed), 0);
    assert_eq!(solve_for("{}", Mode::IncludeRed), 0);
}

#[test]
fn b_example() {
    assert_eq!(solve_for("[1,2,3]", Mode::IgnoreRed), 6);
    assert_eq!(solve_for(r#"[1,{"c":"red","b":2},3]"#, Mode::IgnoreRed), 4);
    assert_eq!(
        solve_for(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, Mode::IgnoreRed),
        0
    );
    assert_eq!(solve_for(r#"[1,"red",5]"#, Mode::IgnoreRed), 6);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::IncludeRed), 119433);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::IgnoreRed), 68466);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::IncludeRed).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::IgnoreRed).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
