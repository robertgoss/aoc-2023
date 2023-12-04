extern crate core;

mod calibration;
mod io;
mod parts;
mod scratchcards;
mod snow_game;

mod challenge {
    use super::*;

    fn challenge_1() {
        let data = io::input_as_lines(1);
        println!("{:?}", calibration::calibration_total(&data, false));
    }

    fn challenge_2() {
        let data = io::input_as_lines(1);
        println!("{:?}", calibration::calibration_total(&data, true));
    }

    fn challenge_3() {
        let data = io::input_as_lines(2);
        let games = snow_game::Games::from_lines(&data).unwrap();
        println!("{:?}", games.sum_possible());
    }

    fn challenge_4() {
        let data = io::input_as_lines(2);
        let games = snow_game::Games::from_lines(&data).unwrap();
        println!("{:?}", games.sum_power_set());
    }

    fn challenge_5() {
        let data = io::input_as_lines(3);
        let engine = parts::Engine::from_lines(&data);
        println!("{:?}", engine.sum_part_numbers());
    }

    fn challenge_6() {
        let data = io::input_as_lines(3);
        let engine = parts::Engine::from_lines(&data);
        println!("{:?}", engine.sum_gears());
    }

    fn challenge_7() {
        let data = io::input_as_lines(4);
        let cards = scratchcards::ScratchCards::from_lines(&data).unwrap();
        println!("{:?}", cards.winnings());
    }

    fn challenge_8() {
        let data = io::input_as_lines(4);
        let cards = scratchcards::ScratchCards::from_lines(&data).unwrap();
        println!("{:?}", cards.winning_scratchcards());
    }

    pub fn challenge(num: u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            7 => challenge_7(),
            8 => challenge_8(),
            _ => (),
        }
    }
}

fn main() {
    let default = "8".to_string();
    let args: Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap_or(&default).parse::<u8>().unwrap();
    challenge::challenge(ver);
}
