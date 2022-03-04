use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

/* Hello, this program is meant to create random people
 * to Fullfill our SQL base 'ART MARKET'
*/

static LIST_FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

static LIST_LAST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

fn create_reputation(_number_of_creation: i32, _base_reput: i32)->i32 {
    let mut rng = thread_rng();
    return rng.gen_range(1..2000);
}

fn create_address()-> String {
    return "".to_string();
}

fn create_artist()-> String {

    let mut rng = thread_rng();


    let first_name =
        match &LIST_FIRST_NAME.choose(&mut rng) {
            Some(n) => n,
            None => ""
        };
    let last_name =
        match &LIST_LAST_NAME.choose(&mut rng) {
            Some(n) => n,
            None => ""
        };
    let address = create_address();

    //createReputation(number of creation, base reput)
    let reputation_artiste = create_reputation(10, 0);

    let request = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                            "INSERT INTO ",
                            "<P1_ARTISTE>(nomartiste, webartiste, reputationartiste, nationaliteartiste)",
                            "\n VALUES ('", &first_name, " ", &last_name, "',",
                            "'",&first_name,"-",&last_name,".org',",
                            "'",&reputation_artiste.to_string(),"',",
                            "'",&address,"',",
                            ")"
                         );

    return request;
}

fn import_artists() {
    
}

fn main() {
    println!("Art is dead !");
    let request = create_artist();
    print!("{}", request)
}
