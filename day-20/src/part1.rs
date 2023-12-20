use std::collections::{HashMap, VecDeque};

type ModuleName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

trait Module {
    fn receive_pulse(&mut self, pulse: Pulse, source: ModuleName) -> Vec<(ModuleName, Pulse)>;
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    destinations: Vec<ModuleName>,
}

impl FlipFlop {
    fn new(destinations: Vec<ModuleName>) -> Self {
        FlipFlop {
            on: false,
            destinations,
        }
    }
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse, _source: ModuleName) -> Vec<(ModuleName, Pulse)> {
        match pulse {
            Pulse::High => vec![],
            Pulse::Low => {
                self.on = !self.on;
                let pulse_to_send = if self.on { Pulse::High } else { Pulse::Low };
                self.destinations
                    .iter()
                    .map(|module| (module.clone(), pulse_to_send.clone()))
                    .collect::<Vec<_>>()
            }
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    received: HashMap<ModuleName, Pulse>,
    destinations: Vec<ModuleName>,
}

impl Conjunction {
    fn new(destinations: Vec<ModuleName>, received_vec: Vec<ModuleName>) -> Self {
        let received = received_vec
            .into_iter()
            .fold(HashMap::new(), |mut map, module| {
                map.insert(module, Pulse::Low);
                map
            });

        Conjunction {
            received,
            destinations,
        }
    }
}

impl Module for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse, source: ModuleName) -> Vec<(ModuleName, Pulse)> {
        self.received
            .entry(source)
            .and_modify(|p| *p = pulse.clone())
            .or_insert(pulse);

        let pulse_to_send = if self.received.values().all(|p| p == &Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.destinations
            .iter()
            .map(|module| (module.clone(), pulse_to_send.clone()))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Broadcaster {
    destinations: Vec<ModuleName>,
}

impl Module for Broadcaster {
    fn receive_pulse(&mut self, _pulse: Pulse, _source: ModuleName) -> Vec<(ModuleName, Pulse)> {
        self.destinations
            .iter()
            .map(|module| (module.clone(), Pulse::Low))
            .collect::<Vec<_>>()
    }
}

pub fn solve(input: &str) -> u32 {
    let mut modules: HashMap<ModuleName, Box<dyn Module>> = HashMap::new();
    let mut inputs: HashMap<ModuleName, Vec<ModuleName>> = HashMap::new();

    input
        .lines()
        .map(|l| {
            l.split_once(" -> ")
                .map(|(str_name, str_destinations)| {
                    let destinations = str_destinations
                        .split(", ")
                        .map(|s| {
                            inputs
                                .entry(s.to_owned())
                                .and_modify(|i| i.push(str_name[1..].to_owned()))
                                .or_insert(vec![str_name[1..].to_owned()]);

                            s.to_owned()
                        })
                        .collect::<Vec<_>>();

                    (str_name, destinations)
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(str_name, destinations)| {
            if str_name == "broadcaster" {
                modules.insert(
                    String::from("broadcaster"),
                    Box::new(Broadcaster { destinations }),
                );
            } else if str_name.chars().nth(0).unwrap() == '%' {
                modules.insert(
                    str_name[1..].to_owned(),
                    Box::new(FlipFlop::new(destinations)),
                );
            } else if str_name.chars().nth(0).unwrap() == '&' {
                let inputs = inputs.remove(&str_name[1..]).unwrap_or(vec![]);

                modules.insert(
                    str_name[1..].to_owned(),
                    Box::new(Conjunction::new(destinations, inputs)),
                );
            } else {
                unreachable!()
            }
        });

    let mut queue: VecDeque<(ModuleName, Pulse, ModuleName)> = VecDeque::new();

    let mut low_count = 1000;
    let mut high_count = 0;

    for _ in 0..1000 {
        modules
            .get_mut("broadcaster")
            .unwrap()
            .receive_pulse(Pulse::High, String::from(""))
            .iter()
            .for_each(|d| queue.push_back((d.0.clone(), d.1.clone(), String::from("broadcaster"))));

        while let Some((module, pulse, source)) = queue.pop_front() {
            match &pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            };

            if !modules.contains_key(&module) {
                continue;
            }

            modules
                .get_mut(&module)
                .unwrap()
                .receive_pulse(pulse, source)
                .iter()
                .for_each(|d| {
                    queue.push_back((d.0.clone(), d.1.clone(), module.clone()));
                });
        }
    }

    low_count * high_count
}
