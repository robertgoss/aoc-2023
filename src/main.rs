#![feature(btree_cursors)]
extern crate core;

mod calibration;
mod cards;
mod io;
mod map;
mod parts;
mod race;
mod scratchcards;
mod seeds;
mod sequences;
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

    fn challenge_9() {
        let data = io::input_as_string(5);
        let almanac = seeds::Almanac::from_string(&data).unwrap();
        println!("{:?}", almanac.least_site());
    }

    fn challenge_10() {
        // TODO - Debug and fix
        let data = io::input_as_string(0);
        let almanac = seeds::Almanac::from_string(&data).unwrap();
        println!("{:?}", almanac.least_site_ranges());
    }

    fn challenge_11() {
        let data = io::input_as_lines(6);
        let races = race::Races::from_lines(&data).unwrap();
        println!("{:?}", races.score());
    }

    fn challenge_12() {
        let data = io::input_as_lines(6);
        let race = race::Race::from_lines(&data).unwrap();
        println!("{:?}", race.num_success());
    }

    fn challenge_13() {
        let data = io::input_as_lines(7);
        let game = cards::Game::from_lines(&data, false).unwrap();
        println!("{:?}", game.winnings());
    }

    fn challenge_14() {
        let data = io::input_as_lines(7);
        let game = cards::Game::from_lines(&data, true).unwrap();
        println!("{:?}", game.winnings());
    }

    fn challenge_15() {
        let data = io::input_as_string(8);
        let map = map::Map::from_string(&data).unwrap();
        println!("{:?}", map.num_steps());
    }

    fn challenge_16() {
        let data = io::input_as_string(8);
        let map = map::Map::from_string(&data).unwrap();
        println!("{:?}", map.num_steps_ghost());
    }

    fn challenge_17() {
        let data = io::input_as_lines(9);
        let gens = sequences::SequenceGens::from_lines(&data).unwrap();
        println!("{:?}", gens.next_sum());
    }

    fn challenge_18() {
        let data = io::input_as_lines(9);
        let gens = sequences::SequenceGens::from_lines(&data).unwrap();
        println!("{:?}", gens.prev_sum());
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
            9 => challenge_9(),
            10 => challenge_10(),
            11 => challenge_11(),
            12 => challenge_12(),
            13 => challenge_13(),
            14 => challenge_14(),
            15 => challenge_15(),
            16 => challenge_16(),
            17 => challenge_17(),
            18 => challenge_18(),
            _ => (),
        }
    }
}

fn main() {
    let default = "18".to_string();
    let args: Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap_or(&default).parse::<u8>().unwrap();
    challenge::challenge(ver);
}
