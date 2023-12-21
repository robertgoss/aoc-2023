use std::collections::{HashMap, VecDeque};

enum State {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Module {
    state: State,
    outputs: Vec<String>,
}

pub struct Network {
    modules: HashMap<String, Module>,
}

impl Network {
    pub fn from_lines(lines: &Vec<String>) -> Option<Network> {
        let modules = lines
            .iter()
            .map(|line| Module::from_line(line))
            .collect::<Option<HashMap<String, Module>>>()?;
        let mut network = Network { modules };
        network.set_con_inputs();
        Some(network)
    }

    pub fn count_signals(&mut self, num: usize) -> usize {
        let mut count_low: usize = 0;
        let mut count_high: usize = 0;
        for _ in 0..num {
            let (l, h, _) = self.count_signal();
            count_high += h;
            count_low += l;
        }
        count_low * count_high
    }

    pub fn count_presses(&mut self) -> usize {
        for i in 0_usize.. {
            if i % 1024 == 0 {
                println!("{}", i);
            }
            let (_, _, fin) = self.count_signal();
            if fin {
                return i + 1;
            }
        }
        0
    }

    fn count_signal(&mut self) -> (usize, usize, bool) {
        let mut count_low: usize = 1;
        let mut count_high: usize = 0;
        let mut fin = false;
        let mut to_send: VecDeque<(String, String, bool)> = VecDeque::new();
        to_send.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while !to_send.is_empty() {
            let (from, module_name, pulse) = to_send.pop_front().unwrap();
            if module_name == "rx" && !pulse {
                fin = true;
            }
            if let Some(module) = self.modules.get_mut(&module_name) {
                let outputs = module.send(&from, pulse);
                count_low += outputs.iter().filter(|(_, pulse)| !*pulse).count();
                count_high += outputs.iter().filter(|(_, pulse)| *pulse).count();
                for output in outputs {
                    to_send.push_back((module_name.clone(), output.0, output.1))
                }
            }
        }
        (count_low, count_high, fin)
    }

    fn set_con_inputs(&mut self) {
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for (name, module) in self.modules.iter() {
            for out_name in module.outputs.iter() {
                inputs
                    .entry(out_name.to_string())
                    .and_modify(|l| l.push(name.to_string()))
                    .or_insert(vec![name.to_string()]);
            }
        }
        for (name, module) in self.modules.iter_mut() {
            if let State::Conjunction(map) = &mut module.state {
                for input in inputs.get(name).unwrap() {
                    map.insert(input.to_string(), false);
                }
            }
        }
    }
}

impl Module {
    fn from_line(line: &str) -> Option<(String, Module)> {
        let (name_dec, output_str) = line.split_once(" -> ")?;
        let outputs: Vec<String> = output_str.split(", ").map(|out| out.to_string()).collect();
        if name_dec == "broadcaster" {
            Some((
                name_dec.to_string(),
                Module {
                    state: State::Broadcast,
                    outputs,
                },
            ))
        } else if let Some(name) = name_dec.strip_prefix("%") {
            Some((
                name.to_string(),
                Module {
                    state: State::FlipFlop(false),
                    outputs,
                },
            ))
        } else if let Some(name) = name_dec.strip_prefix("&") {
            Some((
                name.to_string(),
                Module {
                    state: State::Conjunction(HashMap::new()),
                    outputs,
                },
            ))
        } else {
            None
        }
    }

    fn send(&mut self, from: &str, pulse: bool) -> Vec<(String, bool)> {
        if let Some(out_pulse) = self.state.signal(from, pulse) {
            self.outputs
                .iter()
                .map(|out| (out.to_string(), out_pulse))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl State {
    fn signal(&mut self, from: &str, pulse: bool) -> Option<bool> {
        match self {
            State::Broadcast => Some(pulse),
            State::FlipFlop(state) => {
                if pulse {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            State::Conjunction(map) => {
                map.insert(from.to_string(), pulse);
                Some(!map.values().all(|v| *v))
            }
        }
    }
}
