use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        use Category::*;

        match value {
            "x" => ExtremelyCool,
            "m" => Musical,
            "a" => Aerodynamic,
            "s" => Shiny,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(Category, u32),
    GreaterThan(Category, u32),
    None,
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let category = Category::from(&value[0..1]);
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
    rules: Vec<(Condition, Destination<'a>)>,
}

impl<'a> Workflow<'a> {
    fn process(&self, part: &Part) -> Destination {
        for rule in self.rules.iter() {
            if let Condition::None = rule.0 {
                return rule.1.clone();
            }

            if let Condition::LessThan(category, value) = &rule.0 {
                let passes = match category {
                    Category::ExtremelyCool => &part.x < value,
                    Category::Musical => &part.m < value,
                    Category::Aerodynamic => &part.a < value,
                    Category::Shiny => &part.s < value,
                };
                if passes {
                    return rule.1.clone();
                }
            }

            if let Condition::GreaterThan(category, value) = &rule.0 {
                let passes = match category {
                    Category::ExtremelyCool => &part.x > value,
                    Category::Musical => &part.m > value,
                    Category::Aerodynamic => &part.a > value,
                    Category::Shiny => &part.s > value,
                };
                if passes {
                    return rule.1.clone();
                }
            }
        }

        unreachable!();
    }
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

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = &value[1..value.len() - 1];
        let ratings = value
            .split(',')
            .map(|s| {
                s.split_once('=')
                    .map(|(_, v)| v.parse::<u32>().unwrap())
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Part {
            x: ratings[0],
            m: ratings[1],
            a: ratings[2],
            s: ratings[3],
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let (workflows, parts) = input
        .split_once("\n\n")
        .map(|(str_workflows, str_parts)| {
            let workflows = str_workflows
                .lines()
                .map(|l| {
                    let w = Workflow::from(l);
                    (w.name, w)
                })
                .collect::<HashMap<&str, Workflow>>();
            let parts = str_parts
                .lines()
                .map(|l| Part::from(l))
                .collect::<Vec<Part>>();
            (workflows, parts)
        })
        .unwrap();

    let mut queue: VecDeque<(&str, Part)> = VecDeque::new();
    let mut sum: u32 = 0;

    parts
        .into_iter()
        .for_each(|part| queue.push_back(("in", part)));

    while let Some((workflow_name, part)) = queue.pop_front() {
        let workflow = workflows.get(&workflow_name).unwrap();

        match workflow.process(&part) {
            Destination::Workflow(name) => queue.push_back((name, part)),
            Destination::Accepted => sum += part.total_rating(),
            Destination::Rejected => {}
        }
    }

    sum
}
