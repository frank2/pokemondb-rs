use pokemon::types::{PokemonType, TypeData};
use pokemon::Stats;

use regex::Regex;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PokemonData {
    pub name: String,
    pub typedata: TypeData,
    pub stats: Stats,
}
impl PokemonData {
    pub fn get(name: &str) -> Result<Self, &'static str> {
        let request = reqwest::blocking::get(format!("https://pokemondb.net/pokedex/{}", name));
        if request.is_err() { return Err("couldn't get PokemonDB data"); }
        
        let body = request.unwrap().text();
        if body.is_err() { return Err("no Pokemon data to parse"); }

        let pkmn_data = body.unwrap();

        println!("[+] Got {} data.", name);
        
        let name_regex = Regex::new("<h1>([A-Za-z\\-' .é]+)</h1>").unwrap();
        let type_regex = Regex::new("<th>Type</th>\n<td>\n<a class=\"type-icon type-[a-z]+\" href=\"/type/(?P<primary>[a-z]+)\">[A-Za-z]+</a>(?: <a class=\"type-icon type-[a-z]+\" href=\"/type/(?P<secondary>[a-z]+)\">[A-Za-z]+</a>)?").unwrap();
        let hp_regex = Regex::new("<th>HP</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();
        let attack_regex = Regex::new("<th>Attack</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();
        let defense_regex = Regex::new("<th>Defense</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();
        let sp_atk_regex = Regex::new("<th>Sp. Atk</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();
        let sp_def_regex = Regex::new("<th>Sp. Def</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();
        let speed_regex = Regex::new("<th>Speed</th>\n<td class=\"cell-num\">([0-9]+)</td>").unwrap();

        let name_results = name_regex.captures(pkmn_data.as_str());
        if name_results.is_none() { return Err("couldn't get name of Pokemon"); }

        let name = name_results.unwrap().get(1).unwrap().as_str();

        let type_results = type_regex.captures(pkmn_data.as_str());
        if type_results.is_none() { return Err("couldn't get Pokemon type"); }

        let type_captures = type_results.unwrap();
        let primary_type = type_captures.name("primary").unwrap().as_str();
        let mut secondary_type: Option<&str> = None;
        let secondary_capture = type_captures.name("secondary");

        if secondary_capture.is_some() {
            secondary_type = Some(secondary_capture.unwrap().as_str());
        }

        let hp_results = hp_regex.captures(pkmn_data.as_str());
        if hp_results.is_none() { return Err("couldn't get Pokemon HP"); }

        let hp = hp_results.unwrap().get(1).unwrap().as_str();

        let attack_results = attack_regex.captures(pkmn_data.as_str());
        if attack_results.is_none() { return Err("couldn't get Pokemon attack"); }

        let attack = attack_results.unwrap().get(1).unwrap().as_str();

        let defense_results = defense_regex.captures(pkmn_data.as_str());
        if defense_results.is_none() { return Err("couldn't get Pokemon defense"); }

        let defense = defense_results.unwrap().get(1).unwrap().as_str();

        let sp_atk_results = sp_atk_regex.captures(pkmn_data.as_str());
        if sp_atk_results.is_none() { return Err("couldn't get Pokemon special attack"); }

        let sp_atk = sp_atk_results.unwrap().get(1).unwrap().as_str();

        let sp_def_results = sp_def_regex.captures(pkmn_data.as_str());
        if sp_def_results.is_none() { return Err("couldn't get Pokemon special defense"); }

        let sp_def = sp_def_results.unwrap().get(1).unwrap().as_str();

        let speed_results = speed_regex.captures(pkmn_data.as_str());
        if speed_results.is_none() { return Err("couldn't get Pokemon speed"); }

        let speed = speed_results.unwrap().get(1).unwrap().as_str();

        let primary_type = PokemonType::from_string(&String::from(primary_type));
        if primary_type.is_err() { return Err(primary_type.err().unwrap()); }

        let primary_type_id = primary_type.unwrap();
        let mut secondary_type_id: Option<PokemonType> = None;

        if secondary_type.is_some() {
            let type_result = PokemonType::from_string(&String::from(secondary_type.unwrap()));
            if type_result.is_err() { return Err(type_result.err().unwrap()); }

            secondary_type_id = Some(type_result.unwrap());
        }

        Ok(PokemonData {
                name: String::from(name),
                typedata: TypeData {
                    primary: primary_type_id,
                    secondary: secondary_type_id
                },
                stats: Stats {
                    hp: hp.parse().unwrap(),
                    attack: attack.parse().unwrap(),
                    defense: defense.parse().unwrap(),
                    special_attack: sp_atk.parse().unwrap(),
                    special_defense: sp_def.parse().unwrap(),
                    speed: speed.parse().unwrap()
                }
        })
                
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum PokemonGame {
    RedBlueYellow,
    GoldSilverCrystal,
    RubySapphireEmerald,
    FireRedLeafGreen,
    DiamondPearl,
    Platinum,
    HeartGoldSoulSilver,
    BlackWhite,
    BlackWhite2,
    XY,
    OmegaRubyAlphaSapphire,
    SunMoon,
    UltraSunUltraMoon,
    LetsGoPikachuEevee,
    SwordShield,
    National,
}

pub fn get_pokedex(game: PokemonGame) -> Result<Vec<String>, &'static str> {
    let dex_url_chunk = match game {
        PokemonGame::RedBlueYellow => "game/red-blue-yellow",
        PokemonGame::GoldSilverCrystal => "game/gold-silver-crystal",
        PokemonGame::RubySapphireEmerald => "game/ruby-sapphire-emerald",
        PokemonGame::FireRedLeafGreen => "game/firered-leafgreen",
        PokemonGame::DiamondPearl => "game/diamond-pearl",
        PokemonGame::Platinum => "game/platinum",
        PokemonGame::HeartGoldSoulSilver => "game/heartgold-soulsilver",
        PokemonGame::BlackWhite => "game/black-white",
        PokemonGame::BlackWhite2 => "game/black-white-2",
        PokemonGame::XY => "game/x-y",
        PokemonGame::OmegaRubyAlphaSapphire => "game/omega-ruby-alpha-sapphire",
        PokemonGame::SunMoon => "game/sun-moon",
        PokemonGame::UltraSunUltraMoon => "game/ultra-sun-ultra-moon",
        PokemonGame::LetsGoPikachuEevee => "game/lets-go-pikachu-eevee",
        PokemonGame::SwordShield => "game/sword-shield",
        PokemonGame::National => "national",
    };
    let request = reqwest::blocking::get(format!("https://pokemondb.net/pokedex/{}", dex_url_chunk));
    if request.is_err() { return Err("couldn't get Pokedex data"); }

    let body = request.unwrap().text();
    if body.is_err() { return Err("no Pokedex data to parse"); }

    let dex_data = body.unwrap();
    let mon_regex = Regex::new("<span class=\"infocard-lg-img\"><a href=\"/pokedex/([a-z]+)\">").unwrap();
    let mut result = Vec::<String>::new();

    for capture in mon_regex.captures_iter(dex_data.as_str()) {
        result.push(String::from(capture.get(1).unwrap().as_str()));
    }

    Ok(result)
}
            
