use std::collections::HashMap;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/19_example");
const INPUT: &str = include_str!("input/19");

enum Category {
    X,
    M,
    A,
    S,
}

enum Operator {
    MoreThan,
    LessThan,
}

enum Action {
    Workflow(String),
    Reject,
    Accept,
}

struct Rule {
    category: Category,
    operator: Operator,
    threshold: u64,
    action: Action,
}

struct Workflow {
    rules: Vec<Rule>,
    otherwise: Action,
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

struct WorkflowRange {
    workflow: String,
    range: Range,
}

#[derive(Clone, Copy)]
struct Range {
    x_min: u64,
    x_max: u64,
    m_min: u64,
    m_max: u64,
    a_min: u64,
    a_max: u64,
    s_min: u64,
    s_max: u64,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> &Action {
        for rule in &self.rules {
            let left = match rule.category {
                Category::X => part.x,
                Category::M => part.m,
                Category::A => part.a,
                Category::S => part.s,
            };
            if match rule.operator {
                Operator::LessThan => left < rule.threshold,
                Operator::MoreThan => left > rule.threshold,
            } {
                return &rule.action;
            }
        }
        &self.otherwise
    }

    fn ranges(&self, mut range: Range) -> (Vec<WorkflowRange>, Vec<Range>) {
        let mut ranges = vec![];
        let mut outputs = vec![];

        for rule in &self.rules {
            if match rule.operator {
                Operator::LessThan => handle_less_than(rule, &mut range, &mut outputs, &mut ranges),
                Operator::MoreThan => handle_more_than(rule, &mut range, &mut outputs, &mut ranges),
            } {
                return (ranges, outputs);
            }
        }

        self.otherwise.act(&range, &mut outputs, &mut ranges);

        (ranges, outputs)
    }
}

impl Range {
    fn min_max_for(&self, category: &Category) -> (u64, u64) {
        match category {
            Category::X => (self.x_min, self.x_max),
            Category::M => (self.m_min, self.m_max),
            Category::A => (self.a_min, self.a_max),
            Category::S => (self.s_min, self.s_max),
        }
    }

    fn set_min(&mut self, category: &Category, value: u64) {
        match category {
            Category::X => self.x_min = value,
            Category::M => self.m_min = value,
            Category::A => self.a_min = value,
            Category::S => self.s_min = value,
        }
    }

    fn set_max(&mut self, category: &Category, value: u64) {
        match category {
            Category::X => self.x_max = value,
            Category::M => self.m_max = value,
            Category::A => self.a_max = value,
            Category::S => self.s_max = value,
        }
    }
}

impl Action {
    fn act(&self, range: &Range, outputs: &mut Vec<Range>, ranges: &mut Vec<WorkflowRange>) {
        match self {
            Action::Accept => {
                outputs.push(*range);
            }
            Action::Reject => {}
            Action::Workflow(w) => {
                ranges.push(WorkflowRange {
                    workflow: w.clone(),
                    range: *range,
                });
            }
        }
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut lines = input.lines();

    let mut workflows = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (name, rest) = line.split_once('{').unwrap();
        let mut rules: Vec<_> = rest[0..rest.len() - 1].split(',').collect();

        let otherwise = match rules.pop().unwrap() {
            "R" => Action::Reject,
            "A" => Action::Accept,
            workflow => Action::Workflow(workflow.to_string()),
        };

        let rules = rules
            .into_iter()
            .map(|r| {
                let (category, rest) = r.split_at(1);
                let (operator, rest) = rest.split_at(1);
                let (threshold, action) = rest.split_once(':').unwrap();

                Rule {
                    category: match category {
                        "x" => Category::X,
                        "m" => Category::M,
                        "a" => Category::A,
                        "s" => Category::S,
                        ch => panic!("unexpected category '{ch}'"),
                    },
                    operator: match operator {
                        "<" => Operator::LessThan,
                        ">" => Operator::MoreThan,
                        ch => panic!("unexpected operator '{ch}'"),
                    },
                    threshold: threshold.parse().unwrap(),
                    action: match action {
                        "R" => Action::Reject,
                        "A" => Action::Accept,
                        workflow => Action::Workflow(workflow.to_string()),
                    },
                }
            })
            .collect();

        workflows.insert(name.to_string(), Workflow { rules, otherwise });
    }

    let parts = lines
        .map(|line| {
            let mut values = line.split('=');
            values.next();
            let (x, _) = values.next().unwrap().split_once(',').unwrap();
            let (m, _) = values.next().unwrap().split_once(',').unwrap();
            let (a, _) = values.next().unwrap().split_once(',').unwrap();
            let (s, _) = values.next().unwrap().split_once('}').unwrap();

            Part {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            }
        })
        .collect();

    (workflows, parts)
}

fn solve_a_for(input: &str) -> u64 {
    let (workflows, parts) = parse(input);

    parts
        .into_iter()
        .map(|part| {
            let mut workflow = workflows.get("in").unwrap();

            loop {
                let action = workflow.evaluate(&part);
                match action {
                    Action::Accept => return part.x + part.m + part.a + part.s,
                    Action::Reject => return 0,
                    Action::Workflow(w) => workflow = workflows.get(w).unwrap(),
                }
            }
        })
        .sum()
}

fn solve_b_for(input: &str) -> u64 {
    let (workflows, _) = parse(input);

    let mut queue = vec![WorkflowRange {
        workflow: "in".to_string(),
        range: Range {
            x_min: 1,
            x_max: 4000,
            m_min: 1,
            m_max: 4000,
            a_min: 1,
            a_max: 4000,
            s_min: 1,
            s_max: 4000,
        },
    }];

    let mut output = vec![];

    while let Some(range) = queue.pop() {
        let workflow = workflows.get(&range.workflow).unwrap();
        let (ranges, outputs) = workflow.ranges(range.range);
        queue.extend(ranges);
        output.extend(outputs);
    }

    output
        .into_iter()
        .map(|o| {
            (o.x_max - o.x_min + 1)
                * (o.m_max - o.m_min + 1)
                * (o.a_max - o.a_min + 1)
                * (o.s_max - o.s_min + 1)
        })
        .sum()
}

fn handle_more_than(
    rule: &Rule,
    range: &mut Range,
    outputs: &mut Vec<Range>,
    ranges: &mut Vec<WorkflowRange>,
) -> bool {
    let (min, max) = range.min_max_for(&rule.category);
    if min > rule.threshold {
        rule.action.act(range, outputs, ranges);
        return true;
    } else if max >= rule.threshold {
        range.set_min(&rule.category, rule.threshold + 1);
        rule.action.act(range, outputs, ranges);

        range.set_min(&rule.category, min);
        range.set_max(&rule.category, rule.threshold);
    }
    false
}

fn handle_less_than(
    rule: &Rule,
    range: &mut Range,
    outputs: &mut Vec<Range>,
    ranges: &mut Vec<WorkflowRange>,
) -> bool {
    let (min, max) = range.min_max_for(&rule.category);
    if max < rule.threshold {
        rule.action.act(range, outputs, ranges);
        return true;
    } else if min <= rule.threshold {
        range.set_max(&rule.category, rule.threshold - 1);
        rule.action.act(range, outputs, ranges);

        range.set_max(&rule.category, max);
        range.set_min(&rule.category, rule.threshold);
    }
    false
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 19114);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 449531);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 167409079868000);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 122756210763577);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
