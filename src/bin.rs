use std::thread::sleep;
use std::time::Duration;

use pokemondb::*;

pub fn main() {
    // let flabebe = PokemonData::get("flabebe").unwrap();
    
    let mut dex_data = get_pokedex(PokemonGame::National).unwrap();
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
