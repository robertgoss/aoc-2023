#![feature(btree_cursors)]
#![feature(ascii_char)]
#![feature(inline_const)]
extern crate core;

mod beams;
mod bricks;
mod calibration;
mod cards;
mod cave;
mod crucible;
mod galaxy;
mod hail;
mod hash;
mod io;
mod lake;
mod map;
mod mirrors;
mod parts;
mod pipes;
mod race;
mod scratchcards;
mod seeds;
mod sequences;
mod signals;
mod snow_game;
mod steps;
mod utils;
mod walk;
mod workflow;

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
        let data = io::input_as_string(5);
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

    fn challenge_19() {
        let data = io::input_as_grid(10);
        let pipes = pipes::Pipes::from_grid(&data);
        println!("{:?}", pipes.max_distance());
    }

    fn challenge_20() {
        let data = io::input_as_grid(0);
        let pipes = pipes::Pipes::from_grid(&data);
        println!("{:?}", pipes.enclosed());
    }

    fn challenge_21() {
        let data = io::input_as_grid(11);
        let galaxy = galaxy::Galaxy::from_grid(2, &data);
        println!("{:?}", galaxy.sum_dist());
    }

    fn challenge_22() {
        let data = io::input_as_grid(11);
        let galaxy = galaxy::Galaxy::from_grid(1000000, &data);
        println!("{:?}", galaxy.sum_dist());
    }

    fn challenge_25() {
        let data = io::input_as_grids(13);
        let caves = cave::Caves::from_grids(&data);
        println!("{:?}", caves.score());
    }

    fn challenge_26() {
        let data = io::input_as_grids(13);
        let caves = cave::Caves::from_grids(&data);
        println!("{:?}", caves.score_smudge());
    }

    fn challenge_27() {
        let data = io::input_as_grid(14);
        let mut beam = beams::Beam::from_grid(&data);
        println!("{:?}", beam.score_north());
    }

    fn challenge_28() {
        let data = io::input_as_grid(14);
        let mut beam = beams::Beam::from_grid(&data);
        println!("{:?}", beam.score_spin(1000000000));
    }

    fn challenge_29() {
        let data = io::input_as_string(15);
        let commands = hash::Commands::from_line(&data);
        println!("{:?}", commands.hash_sum());
    }

    fn challenge_30() {
        let data = io::input_as_string(15);
        let boxes = hash::Boxes::from_line(&data);
        println!("{:?}", boxes.focus_power_sum());
    }

    fn challenge_31() {
        let data = io::input_as_grid(16);
        let cave = mirrors::MirrorCave::from_grid(&data);
        println!("{:?}", cave.simulate_excited());
    }

    fn challenge_32() {
        let data = io::input_as_grid(16);
        let cave = mirrors::MirrorCave::from_grid(&data);
        println!("{:?}", cave.max_simulate_excited());
    }

    fn challenge_33() {
        let data = io::input_as_grid(17);
        let city = crucible::City::from_grid(&data);
        println!("{:?}", city.least_path(1, 3));
    }

    fn challenge_34() {
        let data = io::input_as_grid(17);
        let city = crucible::City::from_grid(&data);
        println!("{:?}", city.least_path(4, 10));
    }

    fn challenge_35() {
        let data = io::input_as_lines(18);
        let path = lake::Path::from_lines(&data).unwrap();
        println!("{:?}", path.area());
    }

    fn challenge_37() {
        let data = io::input_as_string(19);
        let pile = workflow::Pile::from_string(&data).unwrap();
        println!("{:?}", pile.accepted_ratings());
    }

    fn challenge_38() {
        let data = io::input_as_string(19);
        let pile = workflow::Pile::from_string(&data).unwrap();
        println!("{:?}", pile.accepted_combos());
    }

    fn challenge_39() {
        let data = io::input_as_lines(20);
        let mut network = signals::Network::from_lines(&data).unwrap();
        println!("{:?}", network.count_signals(1000));
    }

    fn challenge_40() {
        let data = io::input_as_lines(20);
        let mut network = signals::Network::from_lines(&data).unwrap();
        println!("{:?}", network.count_presses());
    }

    fn challenge_41() {
        let data = io::input_as_grid(21);
        let map = steps::Map::from_grid(&data);
        println!("{:?}", map.steps(64));
    }

    fn challenge_43() {
        let data = io::input_as_lines(22);
        let mut bricks = bricks::Bricks::from_lines(&data).unwrap();
        println!("{:?}", bricks.disintergrate_count());
    }

    fn challenge_44() {
        let data = io::input_as_lines(22);
        let mut bricks = bricks::Bricks::from_lines(&data).unwrap();
        println!("{:?}", bricks.maximum_chain());
    }

    fn challenge_45() {
        let data = io::input_as_grid(23);
        let woods = walk::Woods::from_grid(&data);
        println!("{:?}", woods.maximum_path(false));
    }

    fn challenge_46() {
        let data = io::input_as_grid(23);
        let woods = walk::Woods::from_grid(&data);
        println!("{:?}", woods.maximum_path(true));
    }

    fn challenge_47() {
        let data = io::input_as_lines(24);
        let storm = hail::Storm::from_lines(&data).unwrap();
        println!(
            "{:?}",
            storm.intersect_in_area(200000000000000, 400000000000000 + 1)
        );
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
            19 => challenge_19(),
            20 => challenge_20(),
            21 => challenge_21(),
            22 => challenge_22(),
            25 => challenge_25(),
            26 => challenge_26(),
            27 => challenge_27(),
            28 => challenge_28(),
            29 => challenge_29(),
            30 => challenge_30(),
            31 => challenge_31(),
            32 => challenge_32(),
            33 => challenge_33(),
            34 => challenge_34(),
            35 => challenge_35(),
            37 => challenge_37(),
            38 => challenge_38(),
            39 => challenge_39(),
            40 => challenge_40(),
            41 => challenge_41(),
            43 => challenge_43(),
            44 => challenge_44(),
            45 => challenge_45(),
            46 => challenge_46(),
            47 => challenge_47(),
            _ => (),
        }
    }
}

fn main() {
    let default = "47".to_string();
    let args: Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap_or(&default).parse::<u8>().unwrap();
    challenge::challenge(ver);
}
