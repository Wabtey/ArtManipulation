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

static LIST_ASSOCIATION: &'static [&str] = &["", ""];

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
 * @param past : 'a future date' -> false | 'a past date' -> true
 * format : AAAA-MM-JJ
 */
fn create_date(past: bool) -> String{
    let mut rng = thread_rng();

    let mut date = "year-month-day".to_string();
    let day = rng.gen_range(0..31); // dc about 31/02 or 31/04
    let month = rng.gen_range(0..12);
    let year; //= 1445;
    if past {
        year = rng.gen_range(2003..2021);
    }
    else {
        year = rng.gen_range(2022..2030);
    }
    
    date = date.replace("day", &day.to_string());
    date = date.replace("month", &month.to_string());
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
        request.push_str("datedecreation, ");
    }
    // careful about this else which can conflict
    // with future change on mocodoStructre
    else if rdv_date { 
        request.push_str("datename, ");
    }
    if price {
        request.push_str("prixentryname, ");
    }
    request.push_str("adressename) \n VALUES");
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
            orga_n.push_str("association, ");
            orga_n = orga_n.replace("association", &create_association());
        }
        orga_n.push_str("'country'),");
        orga_n = orga_n.replace("country", &country);
        
        request.push_str(&orga_n);
    }

    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

    request
}

/**
 * frequence: if 0 -> no condition (no filter),
 *            if 8 -> condition 1/8 (only 1/8 will be associate)
 */
fn create_insert_relations(relation_name:String, table_name1: String,
                   table_name2: String, amount: i32, frequence: i32) -> String
{
    let mut request: String =
    "INSERT INTO P1_NAME (idname1, idname2) \n VALUES".to_string();
    request = request
        .replace("NAME",  &relation_name.to_uppercase())
        .replace("name1", &table_name1.to_lowercase())
        .replace("name2", &table_name2.to_lowercase());
    for i in 0..amount {
        
        if frequence != 0{
        // only 1/frequence of all name1 and name2 will be associate
            if i%frequence==0 {
                let foo =
                "\n (idname1, idname2),".to_string();
                let mut foobar = foo.replace("idname1", &i.to_string());
                foobar = foobar.replace("idname2", &i.to_string());
                request.push_str(&foobar);
            }
        }
        else {
            let foo =
            "\n (idname1, idname2),".to_string();
            let mut foobar = foo.replace("idname1", &i.to_string());
            foobar = foobar.replace("idname2", &i.to_string());
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

//--HUMANS------------------------------------------------------------------------------------------

    //--COMM-PRISEURS-------------------------------------------------------------------------------

    let commissaire = create_insert_humans("commissaires-priseurs", amount_of_each, false, false, false);
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
                               "artist".to_string(), amount_of_each, 8);
    number_of_creation +=amount_of_each;
    request.push_str(&help);

    //--DIRIGE-------------------------------------------------------------------
    let dirige = create_insert_relations("dirige".to_string(), "commissaires-priseurs".to_string(),
                               "marche".to_string(), amount_of_each, 0);
    number_of_creation +=amount_of_each;
    request.push_str(&dirige);

    //--PARTICIPE----------------------------------------------------------------
    let participe = create_insert_relations("participe".to_string(), "creancier".to_string(),
                               "marche".to_string(), amount_of_each, 4);
    number_of_creation +=amount_of_each;
    request.push_str(&participe);

    //--VEND---------------------------------------------------------------------
    
    // TODO : CONFLICT with possede
    let sell = create_insert_relations("vend".to_string(), "creancier".to_string(),
                               "art".to_string(), amount_of_each, 9);
    number_of_creation +=amount_of_each;
    request.push_str(&sell);

    //--POSSEDE------------------------------------------------------------------
    
    // TODO : CONFLICT with vend
    let own = create_insert_relations("possede".to_string(), "creancier".to_string(),
                               "art".to_string(), amount_of_each, 2);
    number_of_creation +=amount_of_each;
    request.push_str(&own);

    //--RESTAURE-----------------------------------------------------------------
    let restore = create_insert_relations("restaure".to_string(), "restaurateur".to_string(),
                               "art".to_string(), amount_of_each, 18);
    number_of_creation +=amount_of_each;
    request.push_str(&restore);

    //--PRET---------------------------------------------------------------------
    let loan = create_insert_relations("pret".to_string(), "musee".to_string(),
                               "art".to_string(), amount_of_each, 20);
    number_of_creation +=amount_of_each;
    request.push_str(&loan);

    //--EXPOSE-------------------------------------------------------------------
    let expose = create_insert_relations("expose".to_string(), "galerie".to_string(),
                               "art".to_string(), amount_of_each, 25);
    number_of_creation +=amount_of_each;
    request.push_str(&expose);

    //--EXPERTISE----------------------------------------------------------------
    let expertise = create_insert_relations("expertise".to_string(), "expert".to_string(),
                                    "art".to_string(), amount_of_each, 4);
    number_of_creation +=amount_of_each;
    request.push_str(&expertise);

    //--JUGE---------------------------------------------------------------------
    let juge = create_insert_relations("juge".to_string(), "critique".to_string(),
                               "art".to_string(), amount_of_each, 2);
    number_of_creation +=amount_of_each;
    request.push_str(&juge);


//--WRITING------------------------------------------------------------------------------------
    
    println!("humans created: {}", number_of_creation);
    println!("--------create_.txt---------");

    // /private/student/n/in/fepain/R/art-manipulation/RENDU/humans.txt
    // E:/Code/projects Rust/art-manipulation/RENDU/humans.txt
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/humansV2.txt",
			  request)
		.expect("Unable to write file");
}


