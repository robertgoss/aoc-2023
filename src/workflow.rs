use std::collections::HashMap;

enum Property {
    X,
    M,
    A,
    S,
}

enum Op {
    LT,
    GT,
}

#[derive(Clone)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

struct Rule {
    prop: Property,
    op: Op,
    val: i64,
    res: Destination,
}

struct Workflow {
    rules: Vec<Rule>,
    default: Destination,
}

pub struct Pile {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

impl Pile {
    pub fn from_string(string: &str) -> Option<Pile> {
        let (workflow_str, part_str) = string.split_once("\n\n")?;
        let workflows = workflow_str
            .lines()
            .map(|line| Workflow::from_line(&line))
            .collect::<Option<HashMap<String, Workflow>>>()?;
        let parts = part_str
            .lines()
            .map(|line| Part::from_line(&line))
            .collect::<Option<Vec<Part>>>()?;
        Some(Pile { workflows, parts })
    }

    pub fn accepted_ratings(&self) -> i64 {
        self.parts
            .iter()
            .filter(|part| self.accept(part))
            .map(|part| part.rating())
            .sum()
    }

    pub fn accepted_combos(&self) -> i64 {
        self.accepted_ranges()
            .iter()
            .map(|range| range.size())
            .sum()
    }

    fn accepted_ranges(&self) -> Vec<PartRange> {
        let mut to_check: Vec<(String, PartRange)> = vec![("in".to_string(), PartRange::new())];
        let mut ranges: Vec<PartRange> = Vec::new();
        while !to_check.is_empty() {
            let (workflow_name, range) = to_check.pop().unwrap();
            let workflow = self.workflows.get(&workflow_name).unwrap();
            for (destination, processed_range) in workflow.process(&range) {
                match destination {
                    Destination::Accept => ranges.push(processed_range),
                    Destination::Reject => {}
                    Destination::Workflow(next) => to_check.push((next, processed_range)),
                }
            }
        }
        ranges
    }

    fn accept(&self, part: &Part) -> bool {
        let mut workflow_name = "in".to_string();
        loop {
            let workflow = self.workflows.get(&workflow_name).unwrap();
            let destination = workflow.send(part);
            match destination {
                Destination::Accept => return true,
                Destination::Reject => return false,
                Destination::Workflow(next) => workflow_name = next.clone(),
            }
        }
    }
}
impl Part {
    fn from_line(line: &str) -> Option<Part> {
        let strip_start = line.strip_prefix("{")?;
        let inner = strip_start.strip_suffix("}")?;
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for prop in inner.split(",") {
            let (p, v_s) = prop.split_once("=")?;
            let v = v_s.parse::<i64>().ok()?;
            match p {
                "x" => part.x = v,
                "m" => part.m = v,
                "a" => part.a = v,
                "s" => part.s = v,
                _ => return None,
            }
        }
        Some(part)
    }

    fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }

    fn value(&self, prop: &Property) -> i64 {
        match prop {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
}

impl Workflow {
    fn from_line(line: &str) -> Option<(String, Workflow)> {
        let (name, rule_trail) = line.split_once("{")?;
        let rules_str = rule_trail.strip_suffix("}")?;
        let mut rule_strings: Vec<&str> = rules_str.split(",").collect();
        let default_str = rule_strings.pop()?;
        let default = Destination::from_string(default_str);
        let rules = rule_strings
            .iter()
            .map(|string| Rule::from_string(*string))
            .collect::<Option<Vec<Rule>>>()?;
        Some((name.to_string(), Workflow { rules, default }))
    }

    fn send(&self, part: &Part) -> Destination {
        self.rules
            .iter()
            .filter_map(|rule| rule.apply(part))
            .next()
            .unwrap_or(self.default.clone())
    }

    fn process(&self, in_range: &PartRange) -> Vec<(Destination, PartRange)> {
        let mut current_range = in_range.clone();
        let mut processed = Vec::new();
        for rule in &self.rules {
            let (applies, next) = current_range.limit(&rule.prop, &rule.op, rule.val);
            if let Some(applies_range) = applies {
                processed.push((rule.res.clone(), applies_range))
            }
            if let Some(next_range) = next {
                current_range = next_range;
            } else {
                return processed;
            }
        }
        processed.push((self.default.clone(), current_range));
        processed
    }
}

impl Destination {
    fn from_string(string: &str) -> Destination {
        match string {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            _ => Destination::Workflow(string.to_string()),
        }
    }
}

impl Rule {
    fn from_string(string: &str) -> Option<Rule> {
        let (cond_str, dest_str) = string.split_once(":")?;
        let destination = Destination::from_string(dest_str);
        if let Some((prop_str, val_str)) = cond_str.split_once("<") {
            let val = val_str.parse::<i64>().ok()?;
            let prop = Property::from_string(prop_str)?;
            Some(Rule {
                prop,
                op: Op::LT,
                val,
                res: destination,
            })
        } else if let Some((prop_str, val_str)) = cond_str.split_once(">") {
            let val = val_str.parse::<i64>().ok()?;
            let prop = Property::from_string(prop_str)?;
            Some(Rule {
                prop,
                op: Op::GT,
                val,
                res: destination,
            })
        } else {
            None
        }
    }

    fn apply(&self, part: &Part) -> Option<Destination> {
        let pv = part.value(&self.prop);
        match self.op {
            Op::LT => pv < self.val,
            Op::GT => pv > self.val,
        }
        .then_some(self.res.clone())
    }
}

impl Property {
    fn from_string(string: &str) -> Option<Property> {
        match string {
            "x" => Some(Property::X),
            "m" => Some(Property::M),
            "a" => Some(Property::A),
            "s" => Some(Property::S),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    fn limit(&self, prop: &Property, op: &Op, val: i64) -> (Option<PartRange>, Option<PartRange>) {
        let (min, max) = self.range_prop(prop);
        match op {
            Op::LT => (self.restrict(prop, min, val), self.restrict(prop, val, max)),
            Op::GT => (
                self.restrict(prop, val + 1, max),
                self.restrict(prop, min, val + 1),
            ),
        }
    }

    fn range_prop(&self, prop: &Property) -> (i64, i64) {
        match prop {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn restrict(&self, prop: &Property, min: i64, max: i64) -> Option<PartRange> {
        if min > max {
            None
        } else {
            let mut rest = self.clone();
            match prop {
                Property::X => rest.x = (min, max),
                Property::M => rest.m = (min, max),
                Property::A => rest.a = (min, max),
                Property::S => rest.s = (min, max),
            };
            Some(rest)
        }
    }

    fn size(&self) -> i64 {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }
}
