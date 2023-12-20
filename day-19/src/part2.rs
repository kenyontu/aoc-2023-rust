use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

#[derive(Debug)]
enum Condition<'a> {
    LessThan(&'a str, u32),
    GreaterThan(&'a str, u32),
    None,
}

impl<'a> From<&'a str> for Condition<'a> {
    fn from(value: &'a str) -> Condition<'a> {
        let category = &value[0..1];
        let operator = value.chars().nth(1).unwrap();
        let val = &value[2..].parse::<u32>().unwrap();

        if operator == '>' {
            Self::GreaterThan(category, *val)
        } else if operator == '<' {
            Self::LessThan(category, *val)
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug, Clone)]
enum Destination<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

impl<'a> From<&'a str> for Destination<'a> {
    fn from(value: &'a str) -> Destination<'a> {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            s => Self::Workflow(s),
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<(Condition<'a>, Destination<'a>)>,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Workflow<'a> {
        value
            .split_once("{")
            .map(|(name, str_rules)| {
                let rules = &str_rules[0..str_rules.len() - 1];
                let rules = rules
                    .split(',')
                    .map(|rule| {
                        let s = rule.split(':').collect::<Vec<_>>();
                        if s.len() == 2 {
                            (Condition::from(s[0]), Destination::from(s[1]))
                        } else {
                            (Condition::None, Destination::from(rule))
                        }
                    })
                    .collect::<Vec<_>>();

                Workflow { name, rules }
            })
            .unwrap()
    }
}

type PartRange<'a> = HashMap<&'a str, Range<u32>>;

pub fn solve(input: &str) -> u64 {
    let workflows = input
        .split_once("\n\n")
        .map(|(str_workflows, _)| {
            let workflows = str_workflows
                .lines()
                .map(|l| {
                    let w = Workflow::from(l);
                    (w.name, w)
                })
                .collect::<HashMap<&str, Workflow>>();
            workflows
        })
        .unwrap();

    let mut queue: VecDeque<(Destination, PartRange)> = VecDeque::new();
    let mut sum: u64 = 0;

    let mut initial_pr = PartRange::new();
    initial_pr.insert("x", 1..4001);
    initial_pr.insert("m", 1..4001);
    initial_pr.insert("a", 1..4001);
    initial_pr.insert("s", 1..4001);

    queue.push_back((Destination::Workflow("in"), initial_pr));

    while let Some((destination, pr)) = queue.pop_front() {
        if let Destination::Accepted = destination {
            sum += pr["x"].len() as u64
                * pr["m"].len() as u64
                * pr["a"].len() as u64
                * pr["s"].len() as u64;
            continue;
        }

        if let Destination::Rejected = destination {
            continue;
        }

        let Destination::Workflow(workflow_name) = destination else {
            unreachable!();
        };

        let workflow = workflows.get(&workflow_name).unwrap();

        let mut pr = pr;

        for (condition, destination) in workflow.rules.iter() {
            match condition {
                Condition::LessThan(category, value) => {
                    let range = &pr[*category];
                    let start_range = range.start..*value;

                    if start_range.len() > 0 {
                        let mut pr = pr.clone();
                        pr.entry(category).and_modify(|r| *r = start_range);
                        queue.push_back((destination.clone(), pr));
                    }

                    let end_range = *value..range.end;
                    if end_range.len() > 0 {
                        pr.entry(category).and_modify(|r| *r = end_range);
                    } else {
                        break;
                    }
                }
                Condition::GreaterThan(category, value) => {
                    let range = &pr[*category];

                    let end_range = value + 1..range.end;
                    if end_range.len() > 0 {
                        let mut pr = pr.clone();
                        pr.entry(category).and_modify(|r| *r = end_range);
                        queue.push_back((destination.clone(), pr));
                    }

                    let start_range = range.start..value + 1;
                    if start_range.len() > 0 {
                        pr.entry(category).and_modify(|r| *r = start_range);
                    } else {
                        break;
                    }
                }
                Condition::None => {
                    queue.push_back((destination.clone(), pr));
                    break;
                }
            }
        }
    }

    sum
}
