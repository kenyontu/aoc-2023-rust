use std::collections::HashMap;

#[derive(Debug)]
enum Operation<'a> {
    Remove(u8, &'a str),
    Add(u8, &'a str, u8),
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, focal_length: u8) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

fn hash_str(s: &str) -> u8 {
    s.chars().fold(0_u32, |mut acc, c| {
        acc += c as u32;
        acc *= 17;
        acc %= 256;
        acc
    }) as u8
}

pub fn solve(input: &str) -> u32 {
    let operations = input
        .split(',')
        .map(|s| {
            let s = s.trim();

            if s.chars().nth(s.len() - 1).unwrap() == '-' {
                let label = &s[..s.len() - 1];
                let hash = hash_str(&label);
                Operation::Remove(hash, label)
            } else {
                let (label, focal_length) = s.split_once('=').unwrap();
                let hash = hash_str(&label);
                Operation::Add(hash, label, focal_length.parse().unwrap())
            }
        })
        .collect::<Vec<Operation>>();

    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();
    for operation in operations.iter() {
        match operation {
            Operation::Add(box_i, label, focal_length) => {
                boxes
                    .entry(*box_i)
                    .and_modify(|b: &mut Vec<Lens>| {
                        let new_lens = Lens::new(label, *focal_length);
                        if let Some(replace_i) = b.iter().position(|lens| &lens.label == label) {
                            b[replace_i] = new_lens;
                        } else {
                            b.push(new_lens);
                        }
                    })
                    .or_insert(vec![Lens::new(label, *focal_length)]);
            }
            Operation::Remove(box_i, label) => {
                boxes.entry(*box_i).and_modify(|b| {
                    if let Some(remove_i) = b.iter().position(|lens| &lens.label == label) {
                        b.remove(remove_i);
                    }
                });
            }
        };
    }

    boxes
        .iter()
        .map(|(box_i, b)| {
            b.iter()
                .enumerate()
                .map(|(i, lens)| (*box_i as u32 + 1) * (i as u32 + 1) * lens.focal_length as u32)
                .sum::<u32>()
        })
        .sum()
}
