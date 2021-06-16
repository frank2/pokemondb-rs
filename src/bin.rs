use std::thread::sleep;
use std::time::Duration;

use pokemondb::*;
use pokemon::*;

pub fn dump_dex(game: Option<PokemonGame>) {
    let mut dex_data = get_pokedex(game).unwrap();
    dex_data.sort();

    for pkmn in dex_data {
        let mut pkmn_data_request = PokemonData::get(pkmn.as_str());

        while pkmn_data_request.is_err() {
            sleep(Duration::from_millis(1000));
            pkmn_data_request = PokemonData::get(pkmn.as_str());
        }

        let pkmn_data = pkmn_data_request.unwrap();

        println!("pub const {}: Pokemon = Pokemon {{", pkmn_data.name.to_uppercase());
        println!("    name: \"{}\",", pkmn_data.name);
        println!("    typeset: TypeData {{");
        println!("        primary: PokemonType::{:?},", pkmn_data.typedata.primary);

        if pkmn_data.typedata.secondary.is_none() {
            println!("        secondary: None,");
        }
        else {
            println!("        secondary: Some(PokemonType::{:?}),", pkmn_data.typedata.secondary.clone().unwrap());
        }

        println!("    }},");
        println!("    stats: Stats {{");
        println!("        hp: {},", pkmn_data.stats.hp);
        println!("        attack: {},", pkmn_data.stats.attack);
        println!("        defense: {},", pkmn_data.stats.defense);
        println!("        special_attack: {},", pkmn_data.stats.special_attack);
        println!("        special_defense: {},", pkmn_data.stats.special_defense);
        println!("        speed: {},", pkmn_data.stats.speed);
        println!("    }},");
        println!("}};");
    }
}

pub fn fill_dex(game: Option<PokemonGame>) {
    let mut dex_data = get_pokedex(game).unwrap();
    dex_data.sort();

    let mut dex_name = String::new();

    if game.is_some() {
        dex_name = String::from(format!("{:?}", game.clone().unwrap()));
    }
    else {
        dex_name = String::from("National");
    }

    println!("pub const {}: Pokedex = Pokedex {{", dex_name.to_uppercase());

    if game.is_some() {
        println!("    game: Some(PokemonGame::{:?}),", game.clone().unwrap());
    }
    else {
        println!("    game: None,");
    }

    let rules = match game {
        Some(PokemonGame::RedBlueYellow) => "GEN1_RULES",
        Some(PokemonGame::GoldSilverCrystal) => "GEN2_RULES",
        Some(PokemonGame::RubySapphireEmerald) => "GEN2_RULES",
        Some(PokemonGame::FireRedLeafGreen) => "GEN2_RULES",
        Some(PokemonGame::DiamondPearl) => "GEN2_RULES",
        Some(PokemonGame::Platinum) => "GEN2_RULES",
        Some(PokemonGame::HeartGoldSoulSilver) => "GEN2_RULES",
        Some(PokemonGame::BlackWhite) => "GEN2_RULES",
        Some(PokemonGame::BlackWhite2) => "GEN2_RULES",
        Some(PokemonGame::XY) => "GEN6_RULES",
        Some(PokemonGame::OmegaRubyAlphaSapphire) => "GEN6_RULES",
        Some(PokemonGame::SunMoon) => "GEN6_RULES",
        Some(PokemonGame::UltraSunUltraMoon) => "GEN6_RULES",
        Some(PokemonGame::LetsGoPikachuEevee) => "GEN6_RULES",
        Some(PokemonGame::SwordShield) => "GEN6_RULES",
        None => "GEN6_RULES"
    };

    println!("    rules: {},", rules);
    println!("    pokemon: &[");

    for pkmn in dex_data {
        println!("        {},", pkmn.to_uppercase());
    }

    println!("    ],");
    println!("}};");
}

pub fn main() {
    /*
    let type_null = PokemonData::get("type-null").unwrap();
    let mut national = get_pokedex(PokemonGame::National).unwrap();
    national.sort();
    println!("{:?}", national);
     */

    fill_dex(Some(PokemonGame::RedBlueYellow));
}
