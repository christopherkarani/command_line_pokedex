


#[derive(Serialize, Deserialize, Debug)]
struct NamedAPIResource {
    name: String,
    url: String
}


#[derive(Serialize, Deserialize, Debug)]
struct PokemonAbility {
    ability: NamedAPIResource,
    is_hidden: bool,
    slot: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionGameIndex {
    game_index: i32,
    version: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionDetails {
    rarity: i32,
    version: NamedAPIResource
}

#[derive(Serialize, Deserialize, Debug)]
struct HeldItem {
    item: NamedAPIResource,
    version_details: Vec<VersionDetails>
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionGroupDetails {
    level_learned_at: i32,
    move_learn_method: NamedAPIResource,
    version_group: NamedAPIResource
}

#[derive(Serialize, Deserialize, Debug)]
struct PokemonMoves {
    #[serde(rename = "move")]
    mov: NamedAPIResource,
    version_group_details: Vec<VersionGroupDetails>
}

#[derive(Serialize, Deserialize, Debug)]
struct PokemonSprite {
    back_default: String,
    back_female: Option<String>,
    back_shiny: String,
    back_shiny_female: Option<String>,
    front_default: String,
    front_female: Option<String>,
    front_shiny: String,
    front_shiny_female: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PokemonStat {
    base_stat: i32,
    effort: i32,
    stat: NamedAPIResource
}

#[derive(Serialize, Deserialize, Debug)]
struct PokemonType {
    slot: i32,
    #[serde(rename = "type")]
    typ: NamedAPIResource
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    id: i32,
    name: String,
    base_experience: i32,
    height: i32,
    is_default: bool,
    order: i32,
    weight: i32,
    abilities: Vec<PokemonAbility>,
    forms: Vec<NamedAPIResource>,
    game_indices: Vec<VersionGameIndex>,
    held_items: Vec<HeldItem>,
    location_area_encounters: String,
    moves: Vec<PokemonMoves>,
    species: NamedAPIResource,
    sprites: PokemonSprite,
    stats: Vec<PokemonStat>,
    types: Vec<PokemonType>,
}