use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

use std::fs;

static LIST_FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane", "Otto", "Jan", "Patrick", "Rudolf", "Pietra",
"Lex", "Nancy", "Waltercio", "Peter", "George", "Jotaro", "Joseph", "Jonathan", "Joe", "Maria",
"Laurence", "Emil", "Usul", "Wout", "Montse", "Bent", "Nobert", "Marcel", "Arian", "Hella"];

static LIST_LAST_NAME: &'static [&str] = &["Strömholm", "Strömholm", "Rusten", "Russomagno", "Biddle",
"Burckhardt", "Martin", "Pinney", "Neusüss", "Goldes", "Charlesworth", "Bernsten", "Appelt", "Cohen",
"McCarthy", "Wagner", "Van Toorn", "Mol", "Lynch", "Bell", "Cassell", "Schönthal", "Kammerer", "Hoppe",
"De Vringer", "Faydherbe", "Marqués", "Nobert", "Smith", "Knoll", "Rizzatto", "Tetrarc", "Wanders",
"van der Meulen", "Joestar", "Abitbol"];

static LIST_NATIONALITY: &'static [&str] = &["nationality unknown", "American", "Spanish", "Danish", "Italian", "French", "Estonian", "Mexican", "Swedish", "Israeli", "British", 
"Finnish", "Polish", "Palestinian", "Japanese", "Guatemalan", "Colombian", "Romanian", "Russian", "German", "Argentine", "Kuwaiti", "Belgian", "Dutch", "Norwegian", 
"Chilean", "Swiss", "Costa Rican", "Czech", "Brazilian", "Austrian", "Canadian", "Australian", "Ukrainian", "Hungarian", "Haitian", "Congolese", 
"Bolivian", "Cuban", "Yugoslav", "Portuguese", "Indian", "Peruvian", "Icelandic", "Irish", "Guyanese", "Uruguayan", "Slovak", "Croatian", "Greek", "Chinese", "Venezuelan", 
"Turkish", "Panamanian", "Algerian", "Ecuadorian", "South African", "Iranian", "Korean", "Canadian Inuit", "Paraguayan", "Luxembourgish", "Nicaraguan", "Zimbabwean", 
"Moroccan", "Slovenian", "Tanzanian", "Bulgarian", "Tunisian", "Sudanese", "Taiwanese", "Ethiopian", "Scottish", "Latvian", "Senegalese", "Thai", "New Zealander", "Lithuanian", 
"Pakistani", "Bahamian", "Bosnian", "Malian", "Czechoslovakian", "Georgian", "Egyptian", "Kenyan", "Emirati", "Nigerian", "Cypriot", "Albanian", "Azerbaijani", "Ivorian", "Malaysian", 
"Serbian", "Singaporean", "Namibian", "Cambodian", "Ghanaian", "Afghan", "Native American", "Lebanese", "Kyrgyzstani", "Vietnamese", "Ugandan", "Cameroonian", "Welsh", "Macedonian", 
"Puerto Rican", "Catalan", "Filipino", "Sahrawi", "Bangladeshi", "Coptic", "Persian", "Burkinabe", "Beninese", "Sierra Leonean", "Salvadoran"];

static LIST_ARTWORK_TYPE: &'static [&str] = &["Sculture", "Paint", "Ceramics", "Graphic Art", "Illuminated Manuscripts",
"Jewellery Art", "Metalwork Art", "Mosaic Art", "Photography", "Architecture", "Religious Art", "Rock Art",
"Stained Glass Art"];

static LIST_ASSOCIATION &'static [&str] = &[""];

/*
 * cste to keep the unique propriety of id sql
 * no more than 99 human aloid for each 
 */
// static cst_comm: i32 = 0;
// static cst_expert: i32 = 100;
// static cst_critique: i32 = 200;
// static cst_rest: i32 = 300;
// static cst_creancier: i32 = 400;
// static cst_mecene: i32 = 500;
// static cst_galerie: i32 = 400;


fn create_reputation(_number_of_creation: i32, _base_reput: i32)->i32 {
    let mut rng = thread_rng();
    return rng.gen_range(1..2000);
}

fn create_name() -> String {
    let mut rng = thread_rng();

    let mut first_name: String =
        match &LIST_FIRST_NAME.choose(&mut rng) {
        Some(n) => n.to_string(),
            None => "Foo".to_string()
        };
    let last_name: String =
        match &LIST_LAST_NAME.choose(&mut rng) {
            Some(n) => n.to_string(),
            None => "Bar".to_string()
        };

    first_name.push_str(" ");
    first_name.push_str(&last_name);

    first_name
}

fn create_nationality() -> String {
    let mut rng = thread_rng();

    let nationality =
        match &LIST_NATIONALITY.choose(&mut rng) {
            Some(n) => n,
            None => "nationality unknown"
        };
    
    nationality.to_string()
}

fn create_type() -> String {
    let mut rng = thread_rng();

    let art_type: String = 
        match &LIST_ARTWORK_TYPE.choose(&mut rng) {
            Some(n) => n.to_string(),
            None => "Undefined".to_string()
        };

    art_type
}

fn create_capital() -> i128 {
    let mut rng = thread_rng();
    
    let capital: i128 = rng.gen_range(1..5000000)+1000000;

    capital
}

/**
 * past : post==false | pre==true
 */
fn create_date(past: bool) -> String{
    let mut rng = thread_rng();

    let mut date = "month/year".to_string();
    let month = rng.gen_range(0..12);
    let mut year = 1445;
    if past {
        year = rng.gen_range(2003..2021);
    }
    else {
        year = rng.gen_range(2022..2030);
    }
    date = date.replace("month", &month.to_string());
    date = date.replace("year", &year.to_string());

    date.to_string()
}

/**
 * 0 | 5 | 15 | 30 | 100 | 500 | 1000 | 5000 | 10 000 | 50 000
 */
fn create_price() {
    
}

fn create_association(){
    let mut rng = thread_rng();

    let association: String = 
        match &LIST_ASSOCIATION.choose(&mut rng) {
            Some(n) => n.to_string(),
            None => "Undefined".to_string()
        };

    association
}

/**
 * create a sql request  to insert a good amount of data in your database
 * respect my current sql structure
 * should be great to import a example of struct with all table to auto all of this
 */
fn create_request(table_name: &str, amount: i32, art_type: bool,
                  reputation: bool, capital: bool) -> String 
{
    let mut request: String =
    "INSERT INTO P1_NAME (idname, nomname, "
        .to_string();
    if art_type {
        request.push_str("typename, ");
    }
    if reputation {
        request.push_str("reputationname, ");
    }
    if capital {
        request.push_str("capitalname, ");
    }
    
    request.push_str("nationalitename) \n VALUES");
    request = request
        .replace("NAME", &table_name.to_uppercase())
        .replace("name", &table_name.to_lowercase());

    let mut i: i32 =0;
    for i in 0..amount {

        let name = create_name();

        let nationality = create_nationality();

        let foobar =
        "\n (id, 'display_name', ".to_string();

        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);

        if art_type{
            human_n.push_str("'type', ");
            human_n = human_n.replace("type", &create_type());
        }
        if reputation{
            human_n.push_str("reputation, ");
            human_n = human_n.replace("reputation", &create_reputation(0,0).to_string());
        }
        if capital{
            human_n.push_str("capital, ");
            human_n = human_n.replace("capital", &create_capital().to_string());
        }

        human_n.push_str("'nationality')");
        human_n = human_n.replace("nationality", &nationality);
        
        request.push_str(&human_n);
        request.push_str(",");

    }

    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

    request

}

fn create_organisation() {

}

pub fn create_humans(amount_of_each: i32) // -> Result<()>
{

    let mut rng = thread_rng();

    let mut request: String = "".to_string();

    let mut i: i32 =0;
    let mut number_of_creation: i32 = 0;

//--COMM-PRISEURS-------------------------------------------------------------------------------

    let commissaire = create_request("commissaires-priseurs", amount_of_each, false, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&commissaire);

//--MECENE--------------------------------------------------------------------------------------
    
    let mecene = create_request("mecene", amount_of_each, false, true, true);
    number_of_creation +=amount_of_each;
    request.push_str(&mecene);

//--RESTAURATEUR--------------------------------------------------------------------------------

    let restaurateur = create_request("restaurateur", amount_of_each, true, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&restaurateur);

//--CRITIQUE------------------------------------------------------------------------------------

    let critique = create_request("critique", amount_of_each, false, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&critique);

//--CREANCIER-----------------------------------------------------------------------------------

    let creancier = create_request("creancier", amount_of_each, false, false, true);
    number_of_creation +=amount_of_each;
    request.push_str(&creancier);

//--EXPERT--------------------------------------------------------------------------------------

    let expert = create_request("expert", amount_of_each, true, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&expert);

    /*

//--GALERIE-------------------------------------------------------------------------------------

    let mut insert_galerie =
    "INSERT INTO P1_GALERIE (idgalerie, nomgalerie, dateexpo, prixentryexpo, association, adressegalerie)
    \n VALUES ".to_string();

    for i in 0..50 {

        let name = create_name();

        let date = create_date();

        //0 | 5 | 15 | 30 | 100 | 500 | 1000 | 5000
        let price = create_price();

        let association = create_association();

        let nationality = create_nationality();

        let foobar =
        "\n (id, 'display_name', dateexpo, price, 'association', 'country')";
        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);
        human_n = human_n.replace("dateexpo", &date);
        human_n = human_n.replace("price", &price);
        human_n = human_n.replace("association", &association);
        human_n = human_n.replace("country", &nationality);
        insert_galerie.push_str(&human_n);
        insert_galerie.push_str(",");

        number_of_creation+=1;

    }

    insert_galerie.push_str(";END");
    insert_galerie = insert_galerie.replace(",;END","; \n \n");

    request.push_str(&insert_galerie);

//--MARCHE--------------------------------------------------------------------------------------
//--MUSEE-------------------------------------------------------------------------------------

    */

    
//--RELATIONS-----------------------------------------------------------------------------------

    

    

//--WRITING------------------------------------------------------------------------------------
    
    println!("humans created: {}", number_of_creation);
    println!("--------create_.txt---------");

    // /private/student/n/in/fepain/R/art-manipulation/RENDU/humans.txt
    // E:/Code/projects Rust/art-manipulation/RENDU/humans.txt
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/humansV2.txt",
			  request)
		.expect("Unable to write file");
}


