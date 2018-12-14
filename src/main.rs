use std::io;

mod ck_pokemon;

use crate::ck_pokemon::Pokemon;

//use std::borrow::Borrow;
//use std::borrow::BorrowMut;

#[macro_use]
extern crate serde_derive;


extern crate reqwest;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct ZombiePokemon {
    name: String,
    url: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct PokeAPIPager {
    count: usize,
    next: Option<usize>,
    previous: Option<usize>,
    results: Vec<ZombiePokemon>,
}


struct PokeApi {}

impl PokeApi {
    fn new() -> PokeApi {
        return PokeApi {};
    }

    fn base_url(&self) -> &str {
        return "https://pokeapi.co/api/v2";
    }


    /// get a specific pokemon based on id. This method returns a string which contains the Url
    /// for the pokemon based on id
    fn pokemon(&self, id: i32) -> String {
        let base: &str = self.base_url();
        let mut modified_url = base.to_owned();
        let path_with_id = format!("/pokemon/{}", id);
        modified_url.push_str(&path_with_id.to_string());
        return modified_url;
    }

    /// returns all pokemon with name and link in the poke api database
    fn all_pokemon(&self) -> String {
        let base: &str = self.base_url();
        let mut modified_url = base.to_owned();
        modified_url.push_str("/pokemon");
        return modified_url;
    }
}


fn main() {
    println!("Hello, world!");
    let mut first_name: String = "Chris".to_owned();
    let last_name = "Karani";
    first_name.push_str(last_name);
    println!("Full name is now {}", first_name);

    get_pokemon().expect("Error decoding pokemon");
}


fn get_pokemon() -> io::Result<()> {
    let poke_api = PokeApi::new();

    let all_pokemon_url_string = poke_api.all_pokemon();

    println!("Pokemon Url {}", all_pokemon_url_string);


    // the request and result
    println!("Setting up one moment...");

    let res = reqwest::get(all_pokemon_url_string.as_str())
        .expect("failed to retrieve pokemon api")
        .text()
        .expect("failed to parse json to text");


    let resut_str_result = res.as_str();


    println!("Parsing...");
    let json_result = serde_json::from_str(resut_str_result);


    if json_result.is_ok() {
        let v: PokeAPIPager = json_result.expect("Json decode failed");
        let pokemon = v.results;
        println!("Enter a pokemon ID and I will find it");
        loop_picker(&pokemon);
    } else {
        println!("something went wrong")
    }

    Ok(())
}

fn loop_picker(pokemon: &Vec<ZombiePokemon>) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let result = input
            .trim()
            .to_string()
            .parse::<usize>();

        match result {
            Ok(i) => {
                if i <= pokemon.len() {
                    let picked_pokemon = &pokemon[i - 1];
                    println!("The pokemon you picked is {}", picked_pokemon.name.to_uppercase());
                    get_specific_pokemon(i);
                } else {
                    println!("Input out of bounds, the maximum input is {}", pokemon.len());
                }
            }
            Err(e) => println!("We Encountered an error: {}", e)
        }
    }
}


fn get_specific_pokemon(id: usize) {
    let api = PokeApi::new();
    let pokemon_url_string = api.pokemon(id as i32);
    println!("Pokemon Url: {}", &pokemon_url_string);
    let req = reqwest::get(pokemon_url_string.as_str())
        .expect(" request failed")
        .text()
        .expect("error parsing response to string");

    let json: Result<Pokemon, serde_json::Error> = serde_json::from_str(req.as_str());
    println!("{}", req);
    match json {
        Ok(_t) => println!("Successfully retrieved and decoded pokemon object!"),
        Err(e) => println!("Encountered an error while decoding Pokemon object: {}", e)
    }
//    let _p: Pokemon = json.expect("error decoding response to pokemon object");
//    println!("Done!");
}
