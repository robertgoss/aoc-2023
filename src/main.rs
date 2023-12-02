mod calibration;
mod io;

mod challenge {
    use super::calibration;
    use super::io;

    fn challenge_1() {
        let data = io::input_as_lines(1);
        println!("{:?}", calibration::calibration_total(&data, false));
    }

    fn challenge_2() {
        let data = io::input_as_lines(1);
        println!("{:?}", calibration::calibration_total(&data, true));
    }

    pub fn challenge(num: u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            _ => (),
        }
    }
}

fn main() {
    let default = "2".to_string();
    let args: Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap_or(&default).parse::<u8>().unwrap();
    challenge::challenge(ver);
}
