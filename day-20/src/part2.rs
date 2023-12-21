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

fn gcd(a: &u64, b: &u64) -> u64 {
    let mut a = a.clone();
    let mut b = b.clone();

    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

fn lcm(a: &u64, b: &u64) -> u64 {
    (a / gcd(a, b)) * b
}

pub fn solve(input: &str) -> u64 {
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
                let inputs = match inputs.get(&str_name[1..]) {
                    Some(inputs) => inputs.clone(),
                    None => vec![],
                };

                modules.insert(
                    str_name[1..].to_owned(),
                    Box::new(Conjunction::new(destinations, inputs)),
                );
            } else {
                unreachable!()
            }
        });

    let mut queue: VecDeque<(ModuleName, Pulse, ModuleName)> = VecDeque::new();

    // Based on the problem's input, rx only has a single conjunction input, which only
    // sends a low pulse when it remembers that the last pulse received from its inputs are
    // all high pulses.
    // All we need is to figure out at which interval these inputs send a high pulse, then
    // how many button presses it will take for them all to send a high pulse in the same
    // button press.
    // The problem's input is crafted in a way that rx's input inputs will send a single
    // pulse on each button press and in the same order, allowing us to use lcm to figure
    // out the answer.
    let rx_input = &inputs.get("rx").unwrap()[0];
    let modules_to_track = inputs.get(rx_input).unwrap();

    let mut high_pulse_at: HashMap<ModuleName, Vec<u64>> = HashMap::new();
    let mut button_presses = 0_u64;

    while button_presses < 50000 {
        button_presses += 1;
        modules
            .get_mut("broadcaster")
            .unwrap()
            .receive_pulse(Pulse::High, String::from(""))
            .iter()
            .for_each(|d| queue.push_back((d.0.clone(), d.1.clone(), String::from("broadcaster"))));

        while let Some((module, pulse, source)) = queue.pop_front() {
            if &module == rx_input
                && matches!(pulse, Pulse::High)
                && modules_to_track.contains(&source)
            {
                high_pulse_at
                    .entry(source.clone())
                    .and_modify(|v| v.push(button_presses))
                    .or_insert(vec![button_presses]);
            }

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

    high_pulse_at
        .values()
        .map(|v| v[v.len() - 1] - v[v.len() - 2])
        .reduce(|acc, i| lcm(&acc, &i))
        .unwrap()
}
