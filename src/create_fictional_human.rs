use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

static LIST_FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

static LIST_LAST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

fn create_human()-> String {

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
    let address = "";

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
