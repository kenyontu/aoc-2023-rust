use std::collections::{HashMap, VecDeque};

type ModuleName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct FlipFlop<'a> {
    destinations: &'a Vec<ModuleName>,
    on: bool,
}

impl<'a> FlipFlop<'a> {
    fn new(destinations: &'a Vec<ModuleName>) -> Self {
        Self {
            destinations,
            on: false,
        }
    }
}

#[derive(Debug)]
struct Conjunction<'a> {
    destinations: &'a Vec<ModuleName>,
    received: HashMap<ModuleName, Pulse>,
}

impl<'a> Conjunction<'a> {
    fn new(destinations: &'a Vec<ModuleName>, received_vec: &'a Vec<ModuleName>) -> Self {
        let received = received_vec.iter().fold(HashMap::new(), |mut map, module| {
            map.insert(module.clone(), Pulse::Low);
            map
        });

        Self {
            received,
            destinations,
        }
    }
}

#[derive(Debug)]
struct Broadcaster<'a> {
    destinations: &'a Vec<ModuleName>,
}

impl<'a> Broadcaster<'a> {
    fn new(destinations: &'a Vec<ModuleName>) -> Self {
        Self { destinations }
    }
}

enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcaster(Broadcaster<'a>),
}

impl<'a> Module<'a> {
    fn receive_pulse(
        &mut self,
        pulse: &Pulse,
        source: ModuleName,
    ) -> Option<Vec<(ModuleName, Pulse)>> {
        if let Module::FlipFlop(f) = self {
            return match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    f.on = !f.on;
                    let pulse_to_send = if f.on { Pulse::High } else { Pulse::Low };
                    Some(
                        f.destinations
                            .iter()
                            .map(|module| (module.clone(), pulse_to_send.clone()))
                            .collect::<Vec<_>>(),
                    )
                }
            };
        }

        if let Module::Conjunction(c) = self {
            c.received
                .entry(source)
                .and_modify(|p| *p = pulse.clone())
                .or_insert(pulse.clone());

            let pulse_to_send = if c.received.values().all(|p| p == &Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            };

            return Some(
                c.destinations
                    .iter()
                    .map(|module| (module.clone(), pulse_to_send.clone()))
                    .collect::<Vec<_>>(),
            );
        }

        if let Module::Broadcaster(b) = self {
            return Some(
                b.destinations
                    .iter()
                    .map(|module| (module.clone(), Pulse::Low))
                    .collect::<Vec<_>>(),
            );
        }

        unreachable!()
    }
}

pub fn solve(input: &str) -> u32 {
    let mod_and_dest = input
        .lines()
        .flat_map(|l| {
            l.split_once(" -> ").map(|(str_name, str_destinations)| {
                (
                    str_name,
                    str_destinations
                        .split(", ")
                        .map(|s| s.to_owned())
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect::<Vec<_>>();

    let inputs: HashMap<ModuleName, Vec<ModuleName>> =
        mod_and_dest
            .iter()
            .fold(HashMap::new(), |mut map, (str_name, destinations)| {
                let name = str_name.replace(['&', '%'], "");
                destinations.iter().for_each(|m| {
                    map.entry(m.to_owned())
                        .and_modify(|v| v.push(name.clone()))
                        .or_insert(vec![name.clone()]);
                });

                map
            });

    let mut modules: HashMap<ModuleName, Module> =
        mod_and_dest
            .iter()
            .fold(HashMap::new(), |mut map, (str_name, destinations)| {
                if str_name == &"broadcaster" {
                    map.insert(
                        str_name[..].to_owned(),
                        Module::Broadcaster(Broadcaster::new(destinations)),
                    );
                } else if str_name.as_bytes()[0] == b'%' {
                    map.insert(
                        str_name[1..].to_owned(),
                        Module::FlipFlop(FlipFlop::new(destinations)),
                    );
                } else if str_name.as_bytes()[0] == b'&' {
                    map.insert(
                        str_name[1..].to_owned(),
                        Module::Conjunction(Conjunction::new(
                            destinations,
                            inputs.get(&str_name[1..]).unwrap(),
                        )),
                    );
                } else {
                    unreachable!()
                }

                map
            });

    let mut queue: VecDeque<(ModuleName, Pulse, ModuleName)> = VecDeque::new();

    let mut low_count = 1000;
    let mut high_count = 0;
    let broadcaster = String::from("broadcaster");

    for _ in 0..1000 {
        modules
            .get_mut(&broadcaster)
            .unwrap()
            .receive_pulse(&Pulse::High, broadcaster.clone())
            .unwrap()
            .into_iter()
            .for_each(|d| queue.push_back((d.0, d.1, broadcaster.clone())));

        while let Some((module, pulse, source)) = queue.pop_front() {
            match &pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            };

            if !modules.contains_key(&module) {
                continue;
            }

            if let Some(dest) = modules
                .get_mut(&module)
                .unwrap()
                .receive_pulse(&pulse, source)
            {
                dest.into_iter().for_each(|d| {
                    queue.push_back((d.0, d.1, module.clone()));
                });
            }
        }
    }

    low_count * high_count
}
