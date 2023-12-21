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
    fn receive_pulse(&mut self, pulse: &Pulse, source: ModuleName) -> Vec<(ModuleName, Pulse)> {
        if let Module::FlipFlop(f) = self {
            return match pulse {
                Pulse::High => vec![],
                Pulse::Low => {
                    f.on = !f.on;
                    let pulse_to_send = if f.on { Pulse::High } else { Pulse::Low };
                    f.destinations
                        .iter()
                        .map(|module| (module.clone(), pulse_to_send.clone()))
                        .collect::<Vec<_>>()
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

            return c
                .destinations
                .iter()
                .map(|module| (module.clone(), pulse_to_send.clone()))
                .collect::<Vec<_>>();
        }

        if let Module::Broadcaster(b) = self {
            return b
                .destinations
                .iter()
                .map(|module| (module.clone(), Pulse::Low))
                .collect::<Vec<_>>();
        }

        unreachable!()
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
    let broadcaster = String::from("broadcaster");

    while button_presses < 50000 {
        button_presses += 1;
        modules
            .get_mut(&broadcaster)
            .unwrap()
            .receive_pulse(&Pulse::High, String::from(""))
            .into_iter()
            .for_each(|d| queue.push_back((d.0, d.1, broadcaster.clone())));

        while let Some((module, pulse, source)) = queue.pop_front() {
            if !modules.contains_key(&module) {
                continue;
            }

            if &module == rx_input
                && matches!(pulse, Pulse::High)
                && modules_to_track.contains(&source)
            {
                high_pulse_at
                    .entry(source.clone())
                    .and_modify(|v| v.push(button_presses))
                    .or_insert(vec![button_presses]);
            }

            modules
                .get_mut(&module)
                .unwrap()
                .receive_pulse(&pulse, source)
                .into_iter()
                .for_each(|d| {
                    queue.push_back((d.0, d.1, module.clone()));
                });
        }
    }

    high_pulse_at
        .values()
        .map(|v| v[v.len() - 1] - v[v.len() - 2])
        .reduce(|acc, i| lcm(&acc, &i))
        .unwrap()
}
