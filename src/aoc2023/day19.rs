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
    fn ranges(&self, mut range: Range) -> (Vec<WorkflowRange>, Vec<Range>) {
        let mut ranges = vec![];
        let mut outputs = vec![];

        for rule in &self.rules {
            match rule.operator {
                Operator::LessThan => {
                    let (min, max) = match rule.category {
                        Category::X => (range.x_min, range.x_max),
                        Category::M => (range.m_min, range.m_max),
                        Category::A => (range.a_min, range.a_max),
                        Category::S => (range.s_min, range.s_max),
                    };
                    if max < rule.threshold {
                        match &rule.action {
                            Action::Accept => {
                                outputs.push(range);
                            }
                            Action::Reject => {}
                            Action::Workflow(w) => {
                                ranges.push(WorkflowRange {
                                    workflow: w.clone(),
                                    range,
                                });
                            }
                        }
                        return (ranges, outputs);
                    } else if min <= rule.threshold {
                        match rule.category {
                            Category::X => range.x_max = rule.threshold - 1,
                            Category::M => range.m_max = rule.threshold - 1,
                            Category::A => range.a_max = rule.threshold - 1,
                            Category::S => range.s_max = rule.threshold - 1,
                        }

                        match &rule.action {
                            Action::Accept => {
                                outputs.push(range);
                            }
                            Action::Reject => {}
                            Action::Workflow(w) => {
                                ranges.push(WorkflowRange {
                                    workflow: w.clone(),
                                    range,
                                });
                            }
                        }

                        match rule.category {
                            Category::X => {
                                range.x_max = max;
                                range.x_min = rule.threshold;
                            }
                            Category::M => {
                                range.m_max = max;
                                range.m_min = rule.threshold;
                            }
                            Category::A => {
                                range.a_max = max;
                                range.a_min = rule.threshold;
                            }
                            Category::S => {
                                range.s_max = max;
                                range.s_min = rule.threshold;
                            }
                        }
                    }
                }
                Operator::MoreThan => {
                    let (min, max) = match rule.category {
                        Category::X => (range.x_min, range.x_max),
                        Category::M => (range.m_min, range.m_max),
                        Category::A => (range.a_min, range.a_max),
                        Category::S => (range.s_min, range.s_max),
                    };
                    if min > rule.threshold {
                        match &rule.action {
                            Action::Accept => {
                                outputs.push(range);
                            }
                            Action::Reject => {}
                            Action::Workflow(w) => {
                                ranges.push(WorkflowRange {
                                    workflow: w.clone(),
                                    range,
                                });
                            }
                        }
                        return (ranges, outputs);
                    } else if max >= rule.threshold {
                        match rule.category {
                            Category::X => range.x_min = rule.threshold + 1,
                            Category::M => range.m_min = rule.threshold + 1,
                            Category::A => range.a_min = rule.threshold + 1,
                            Category::S => range.s_min = rule.threshold + 1,
                        }

                        match &rule.action {
                            Action::Accept => {
                                outputs.push(range);
                            }
                            Action::Reject => {}
                            Action::Workflow(w) => {
                                ranges.push(WorkflowRange {
                                    workflow: w.clone(),
                                    range,
                                });
                            }
                        }

                        match rule.category {
                            Category::X => {
                                range.x_min = min;
                                range.x_max = rule.threshold;
                            }
                            Category::M => {
                                range.m_min = min;
                                range.m_max = rule.threshold;
                            }
                            Category::A => {
                                range.a_min = min;
                                range.a_max = rule.threshold;
                            }
                            Category::S => {
                                range.s_min = min;
                                range.s_max = rule.threshold;
                            }
                        }
                    }
                }
            };
        }

        match &self.otherwise {
            Action::Accept => outputs.push(range),
            Action::Reject => {}
            Action::Workflow(w) => {
                ranges.push(WorkflowRange {
                    workflow: w.clone(),
                    range,
                });
            }
        }

        (ranges, outputs)
    }
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
