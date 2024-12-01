use aocf::Aoc;

pub fn input_for(year: i32, day: u32) -> String {
    let mut input = Aoc::new()
        .parse_cli(false)
        .year(Some(year))
        .day(Some(day))
        .init()
        .unwrap()
        .get_input(false)
        .unwrap();

    let ch = input.pop().unwrap();
    if ch != '\n' {
        input.push(ch);
    }
    input
}
