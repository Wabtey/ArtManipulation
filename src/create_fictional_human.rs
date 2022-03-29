use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;

use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs;


static LIST_FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane", "Otto", "Jan", "Patrick", "Rudolf", "Pietra",
"Lex", "Nancy", "Waltercio", "Peter", "George", "Jotaro", "Joseph", "Jonathan", "Joe", "Maria",
"Laurence", "Emil", "Usul", "Wout", "Montse", "Bent", "Nobert", "Marcel", "Arian", "Hella"];

static LIST_LAST_NAME: &'static [&str] = &["Strömholm", "Strömholm", "Rusten", "Russomagno", "Biddle",
"Burckhardt", "Martin", "Pinney", "Neusüss", "Goldes", "Charlesworth", "Bernsten", "Appelt", "Cohen",
"McCarthy", "Wagner", "Van Toorn", "Mol", "Lynch", "Bell", "Cassell", "Schönthal", "Kammerer", "Hoppe",
"De Vringer", "Faydherbe", "Marqués", "Nobert", "Smith", "Knoll", "Rizzatto", "Tetrarc", "Wanders",
"van der Meulen", "Joestar", "Abitbol", "Zepelli", "Speedwagon"];

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

static LIST_ASSOCIATION: &'static [&str] = &["None", ""];

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
	constituent_id: i128,
	display_name: String,
	artist_bio: Option<String>,
	nationality: Option<String>,
	gender: Option<String>,
	begin_date: i16,
	end_date: i16,
	wiki_qid: Option<String>, 
	ulan: Option<String>
}

//-----------------------------------------------------------------------------

// static artworks: Vec<Artwork> = import_artworks();

// static artists: Vec<Artist> = serde_json::from_str(
//     &(fs::read_to_string("E:/Code/projects_rust/MoMA/Artist-reformed.json"))
//     .expect("Unable to read file"))
//     .unwrap();

fn import_artworks() -> Vec<Artwork>{
    let vec_artworks = serde_json::from_str(
        &(fs::read_to_string("E:/Code/projects_rust/MoMA/Artworks-reformed.json"))
            .expect("Unable to read file"))
            .unwrap();

    vec_artworks
}
fn import_artists() -> Vec<Artist>{
    let vec_artistes = serde_json::from_str(
        &(fs::read_to_string("E:/Code/projects_rust/MoMA/Artists-reformed.json"))
            .expect("Unable to read file"))
            .unwrap();

    vec_artistes
}
//-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct Artwork {
  title: String,
  artist: Vec<String>,
  constituent_id: Vec<i32>,
  artist_bio: Vec<String>,
  nationality: Vec<String>,
  begin_date: Vec<i32>,
  end_date: Vec<i32>,
  gender: Vec<String>,
  date: Option<String>,
  medium: Option<String>,
  dimensions: Option<String>,
  credit_line: Option<String>,
  accession_number: Option<String>,
  classification: Option<String>,
  department: Option<String>,
  date_acquired: Option<String>,
  cataloged: Option<String>,
  object_id: i32,
  url: Option<String>,
  thumbnail_url: Option<String>,
//   height_cm: f32,
//   width_cm: f32
}

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

/*
 * reference : 
 * https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
 */
pub fn create_reputation(_number_of_creation: i32, _base_reput: i32)->i32 {
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
 * turn every number less than 10 to two digit number
 * skip other number
 * cause 2016-5-12 or 2032-01-2 
 *      isn't accepted in SQL format (TODO: change date format sql ?)
 */
fn convert_to_twodigit(number: i32) -> String {
    let n = number.to_string();
    let mut temp = "".to_string();
    if n.len() == 1 {
        temp = "0".to_string();
    }
    temp.push_str(&n);
      
    temp
}
  

/**
 * @param past : 'a future date' -> false | 'a past date' -> true
 * format : AAAA-MM-JJ
 */
fn create_date(past: bool) -> String{
    let mut rng = thread_rng();

    let mut date = "year-month-day".to_string();
    let day = convert_to_twodigit(rng.gen_range(0..31)); // dc about 31/02 or 31/04
    // let day_reformed = (day).to_string().padStart(2,0);
    

    let month = convert_to_twodigit(rng.gen_range(0..12));
    let year; //= 1445;
    if past {
        year = rng.gen_range(2003..2021);
    }
    else {
        year = rng.gen_range(2022..2030);
    }
    
    date = date.replace("day", &day);
    date = date.replace("month", &month);
    date = date.replace("year", &year.to_string());

    date.to_string()
}

/**
 * 0 | 5 | 15 | 30 | 100 | 500 | 1000 | 5000 | 10 000 | 50 000
 */
fn create_price() -> i32{
    let mut rng = thread_rng();
    // placeholder
    let price: i32 = rng.gen_range(0..50000);
    price
}

fn create_association() -> String{
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
fn create_insert_humans(table_name: &str, amount: i32, art_type: bool,
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

        human_n.push_str("'nationality'),");
        human_n = human_n.replace("nationality", &nationality);
        
        request.push_str(&human_n);

    }

    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

    request

}

/**
 * if creation_date and rdv_date are true only creation_date will effect
 * you can't bassicaly have those two attributs
 */
fn create_insert_organisations(table_name: String, amount: i32, creation_date: bool,
                       rdv_date: bool, price: bool, association: bool) -> String
{
    let mut request: String =
    "INSERT INTO P1_NAME (idname, nomname, "
        .to_string();
    if creation_date {
        request.push_str("datename, ");
    }
    // careful about this else which can conflict
    // with future change on mocodoStructre
    else if rdv_date { 
        request.push_str("datename, ");
    }
    if price {
        request.push_str("prixname, ");
    }
    if association {
        request.push_str("association, ");
    }
    request.push_str("adressename");

    let mut rng = thread_rng();

    if table_name.to_lowercase() == "marche"{
        request.push_str(", idcommissaire_priseur");
    }

    request.push_str(") \n VALUES");
    request = request
        .replace("NAME", &table_name.to_uppercase())
        .replace("name", &table_name.to_lowercase());

    for i in 0..amount {

        let name = create_name();

        let country = create_nationality();

        let foobar =
        "\n (id, 'display_name', ".to_string();

        let mut orga_n = foobar.replace("id", &i.to_string());
        orga_n = orga_n.replace("display_name", &name);
        if creation_date | rdv_date {
            //if rdv_date means !creation_date
            
            orga_n.push_str("date, ");   // same care here
            orga_n = orga_n.replace("date", &create_date(creation_date));
        }
        if price{
            orga_n.push_str("price, ");
            orga_n = orga_n.replace("price", &create_price().to_string());
        }
        if association{
            orga_n.push_str("'association', ");
            orga_n = orga_n.replace("association", &create_association());
        }

        
        orga_n.push_str("'country'");

        if table_name.to_lowercase() == "marche"{
            orga_n.push_str(", idcommissaire_priseur");
            // instead of a random just pick into commissaire table
            orga_n = orga_n.replace("idcommissaire_priseur", &rng.gen_range(0..amount).to_string());
        }

        orga_n.push_str("),");
        orga_n = orga_n.replace("country", &country);
        
        request.push_str(&orga_n);
    }

    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

    request
}

/**
 * frequence: if 0 -> Err
 *            if 1 -> no condition whatsoever (no filter),
 *            if 8 -> condition 1/8 (only 1/8 will be associate)
 * don't handle insert attribute for PARTICIPE
 */
fn create_insert_relations(relation_name:String, table_name1: String,
                    table_name2: String, amount: i32, frequence: i32,
                    price: bool, duree: bool) -> String
{
    println!("{}", relation_name);
    let mut request: String =
    "INSERT INTO P1_NAME (idname1, idname2".to_string();
    
    request = request
        .replace("NAME",  &relation_name.to_uppercase())
        .replace("name1", &table_name1.to_lowercase())
        .replace("name2", &table_name2.to_lowercase());
    if price {

        if relation_name=="possede" {
            request.push_str(", prixAchat");
            request.push_str(", prixVente");
        }else {
            request.push_str(", prixNAME");
        }
    }
    if duree {
        request.push_str(", datedebutNAME, datefinNAME");
    }
    if relation_name.to_lowercase()=="marche"{
        request.push_str(", idcommissaire_priseur");
    }
    request.push_str(") \n VALUES");
    request = request.replace("NAME", &relation_name.to_lowercase());

    // TODO: import thoses two elsewhere
    let mut artists:Vec<Artist>=Vec::new();
    let mut artworks:Vec<Artwork>=Vec::new();
    if table_name2.to_lowercase() == "artiste" {
        println!("--read artists-reformed.json--");
        artists = import_artists();
    }else if table_name2.to_lowercase() == "art" {
        println!("--read artworks-reformed.json--");
        artworks = import_artworks();
    }

    for i in 0..amount {
        
        // only 1/frequence of all name1 and name2 will be associate

        let mut foo =
        "\n (idname1, idname2".to_string();


        if price {
            if relation_name=="possede" {
                foo.push_str(", prixAchat, prixVente");
                foo = foo.replace("prixAchat", &create_price().to_string());
                // I use the frequence to also choose whenever a art is still owned or selled atm : 1/1

                if i%(frequence)==0 { 
                    foo = foo.replace("prixVente", &create_price().to_string());
                }else {
                    foo = foo.replace("prixVente", " ");
                }
            }
            else {
                foo.push_str(", prix");
                foo = foo.replace("prix", &create_price().to_string());
            }
            
        }
        if duree {
            foo.push_str(", dateD, dateF");
            foo = foo.replace("dateD", &create_date(true).to_string());
            if relation_name=="possede" {
                // 1/1 des oeuvres ont déja été vendu
                if (i%frequence)==0 { 
                    // make it after dureeD but not in the FUTURE
                    foo = foo.replace("dateF", &create_date(true).to_string()); 
                }else {
                    foo = foo.replace("dateF", " ");
                }
            }else {
                foo = foo.replace("dateF", &create_date(false).to_string());
            }
        }



        // We're selecting directly from .json

        let mut rng = thread_rng();
        let mut foobar: String;

        // Select randomly a constituent_id which EXIST
        if table_name2.to_lowercase() == "artiste" {
            foobar = foo.replace("idname1", &i.to_string());
            foobar = foobar.replace("idname2", &artists[rng.gen_range(0.. artists.len())]
                                                        .constituent_id
                                                        .to_string());

        }else if table_name2.to_lowercase() == "art" {
            foobar = foo.replace("idname1", &i.to_string());
            foobar = foobar.replace("idname2", &artworks[rng.gen_range(0.. artworks.len())]
                                                        .object_id
                                                        .to_string());

        }else {
            foobar = foo.replace("idname1", &i.to_string());
            foobar = foobar.replace("idname2", &i.to_string());
        }


                    
        foobar.push_str("),");

        // After creating the whole line, we decide if we keep it based on the frequency
        if (i%frequence)==0 {
            request.push_str(&foobar);
        }
    }
    // end the request with a ';' and
    // remove the last ',' which cause error
    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

    request
}

pub fn create_requests(amount_of_each: i32) // -> Result<()>
{
    let mut request: String = "".to_string();

    let mut number_of_creation: i32 = 0;

    // Idk where to put those
    // let artists: Vec<Artist> = import_artists();
    // let artworks: Vec<Artwork> = import_artworks();


//--HUMANS------------------------------------------------------------------------------------------

    //--COMM-PRISEURS-------------------------------------------------------------------------------

    let commissaire = create_insert_humans("commissaire_priseur", amount_of_each, false, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&commissaire);

    //--MECENE--------------------------------------------------------------------------------------
    
    let mecene = create_insert_humans("mecene", amount_of_each, false, true, true);
    number_of_creation +=amount_of_each;
    request.push_str(&mecene);

    //--RESTAURATEUR--------------------------------------------------------------------------------

    let restaurateur = create_insert_humans("restaurateur", amount_of_each, true, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&restaurateur);

    //--CRITIQUE------------------------------------------------------------------------------------

    let critique = create_insert_humans("critique", amount_of_each, false, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&critique);

    //--CREANCIER-----------------------------------------------------------------------------------

    let creancier = create_insert_humans("creancier", amount_of_each, false, false, true);
    number_of_creation +=amount_of_each;
    request.push_str(&creancier);

    //--EXPERT--------------------------------------------------------------------------------------

    let expert = create_insert_humans("expert", amount_of_each, true, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&expert);

//--ORGANISATIONS-----------------------------------------------------------------------------------

    //--GALERIE-------------------------------------------------------------------------------------

    // all these galleries are past, non temporary, permanent
    let gallery = create_insert_organisations("galerie".to_string(), amount_of_each,
                                      true, false, true, true);
    // gallery.push_str(create_insert_organisations("galerie", amount_of_each/2,
    //                                      false, true, true, true));
    number_of_creation +=amount_of_each;
    request.push_str(&gallery);


    //--MARCHE--------------------------------------------------------------------------------------

//TODO: add idcommissaire into  marche atm, only 0 is ON

    let marche = create_insert_organisations("marche".to_string(), amount_of_each,
                                     false, true, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&marche);

    //--MUSEE---------------------------------------------------------------------------------------

    let museum = create_insert_organisations("musee".to_string(), amount_of_each,
                                     true, false, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&museum);
    
//--RELATIONS---------------------------------------------------------------------------------------


    //--AIDE---------------------------------------------------------------------
    let help = create_insert_relations("aide".to_string(), "mecene".to_string(),
                               "artiste".to_string(), amount_of_each, 8, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&help);

    // //--DIRIGE-------------------------------------------------------------------
    // let dirige = create_insert_relations("dirige".to_string(), "commissaire_priseur".to_string(),
    //                            "marche".to_string(), amount_of_each, 0, false, false);
    // number_of_creation +=amount_of_each;
    // request.push_str(&dirige);

    //--PARTICIPE----------------------------------------------------------------
    let participe = create_insert_relations("participe".to_string(), "creancier".to_string(),
                               "marche".to_string(), amount_of_each, 4, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&participe);

    //--POSSEDE------------------------------------------------------------------
    
    let own = create_insert_relations("possede".to_string(), "creancier".to_string(),
                             "art".to_string(), amount_of_each, 2, true, true);
    number_of_creation +=amount_of_each;
    request.push_str(&own);

    //--RESTAURE-----------------------------------------------------------------
    let restore = create_insert_relations("restaure".to_string(), "restaurateur".to_string(),
                               "art".to_string(), amount_of_each, 18, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&restore);

    //--PRET---------------------------------------------------------------------
    let loan = create_insert_relations("pret".to_string(), "musee".to_string(),
                               "art".to_string(), amount_of_each, 20, false, true);
    number_of_creation +=amount_of_each;
    request.push_str(&loan);

    //--EXPOSE-------------------------------------------------------------------
    let expose = create_insert_relations("expose".to_string(), "galerie".to_string(),
                               "art".to_string(), amount_of_each, 25, false, true);
    number_of_creation +=amount_of_each;
    request.push_str(&expose);

    //--EXPERTISE----------------------------------------------------------------
    let expertise = create_insert_relations("expertise".to_string(), "expert".to_string(),
                                    "art".to_string(), amount_of_each, 4, false, false);
    number_of_creation +=amount_of_each;
    request.push_str(&expertise);

    //--JUGE---------------------------------------------------------------------
    let juge = create_insert_relations("juge".to_string(), "critique".to_string(),
                               "art".to_string(), amount_of_each, 1, true, false);
    number_of_creation +=amount_of_each;
    request.push_str(&juge);


//--WRITING------------------------------------------------------------------------------------
    
    println!("inserts created: {}", number_of_creation);
    println!("--------create_.sql---------");

    // /private/student/n/in/fepain/R/art-manipulation/RENDU/humans.txt
    // E:/Code/projects_rust/art-manipulation/Part1/indv/insertFictivs.sql
	fs::write("E:/Code/projects_rust/art-manipulation/Part1/indv/insertFictivs.sql",
			  request)
		.expect("Unable to write file");
}


