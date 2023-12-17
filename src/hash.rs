pub struct Commands {
    commands: Vec<String>,
}

fn hash(command: &str) -> u8 {
    let mut val = 0_usize;
    for char in command.chars() {
        val += char.as_ascii().unwrap().to_u8() as usize;
        val *= 17;
        val = val % 256;
    }
    val as u8
}

impl Commands {
    pub fn from_line(line: &String) -> Commands {
        let commands = line.split(",").map(|part| part.to_string()).collect();
        Commands { commands }
    }

    pub fn hash_sum(&self) -> usize {
        self.commands
            .iter()
            .map(|command| hash(command) as usize)
            .sum()
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focus: usize,
}
pub struct Boxes {
    boxes: [Vec<Lens>; 256],
}

impl Boxes {
    pub fn from_line(line: &String) -> Boxes {
        let init: [Vec<Lens>; 256] = [const { Vec::new() }; 256];
        let mut boxes = Boxes { boxes: init };
        for command in line.split(",") {
            if let Some(label) = command.strip_suffix("-") {
                boxes.subtract(label);
            } else {
                let (label, focus_str) = command.split_once("=").unwrap();
                let focus = focus_str.parse::<usize>().unwrap();
                boxes.add(label, focus);
            }
        }
        boxes
    }

    fn subtract(&mut self, label: &str) {
        let h = hash(label) as usize;
        self.boxes[h] = self.boxes[h]
            .iter()
            .filter(|lens| lens.label != label)
            .cloned()
            .collect();
    }

    fn add(&mut self, label: &str, focus: usize) {
        let h = hash(label) as usize;
        for lens in self.boxes[h].iter_mut() {
            if lens.label == label {
                lens.focus = focus;
                return;
            }
        }
        self.boxes[h].push(Lens {
            label: label.to_string(),
            focus,
        })
    }

    pub fn focus_power_sum(&self) -> usize {
        let mut total = 0usize;
        for (i, light_box) in self.boxes.iter().enumerate() {
            for (j, lens) in light_box.iter().enumerate() {
                total += (i + 1) * (j + 1) * lens.focus;
            }
        }
        total
    }
}
