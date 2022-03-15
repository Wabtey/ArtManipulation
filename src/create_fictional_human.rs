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

fn create_request_insertion_v2(name: String, number: i128
                               rest_type: Boolean, specialite: Boolean,
                               capital: Boolean, reputation: Boolean)
{
    let mut request =
    "INSERT INTO P1_NAME (idname, nomname, nationalitename)
    \n VALUES"
        .to_string()
        .replace("NAME", name.to_uppercase())
        .replace("name", name.to_lowercase());

    let mut i: i32 =0;
    for i in 0..number {

        let name = create_name();

        let nationality = create_nationality();

        let mut foobar =
        "\n (id, 'display_name',";
        if(reputation)
            foobar.push_str("'reputation', ");
        if(specialite | rest_type) 
            foobar.push_str("'type', ");

        foobar.push_str("'nationality')");

        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);

        if(specialite) {
            let specialite_type = create_type();
            human_n = human_n.replace("type", &specialite_type);
        }

        human_n = human_n.replace("nationality", &nationality);
        request.push_str(&human_n);
        request.push_str(",");

        number_of_creation+=1;

    }

    request.push_str(";END");
    request = request.replace(",;END","; \n \n");

}

pub fn create_human() // -> Result<()>
{

    let mut rng = thread_rng();

    let mut request: String = "".to_string();

//--COMM-PRISEURS-------------------------------------------------------------------------------

    let mut insert_comm =
    "INSERT INTO P1_COMMISSAIRES-PRISEURS (idcommisaire, nomcommisaire, nationalitecommisaire)
    \n VALUES ".to_string();

    let mut i: i32 =0;
    let mut number_of_creation = 0;

    for i in 0..50 {

        let name = create_name();

        let nationality = create_nationality();
        
        let foobar =
        "\n (id, 'display_name', 'nationality')";
        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);
        human_n = human_n.replace("nationality", &nationality);
        insert_comm.push_str(&human_n);
        insert_comm.push_str(",");

        number_of_creation+=1;
        
    }

    insert_comm.push_str(";END");
    insert_comm = insert_comm.replace(",;END","; \n \n");

    request.push_str(&insert_comm);

//--MECENE--------------------------------------------------------------------------------------

    let mut insert_mecene =
    "INSERT INTO P1_MECENE (idmecene, nommecene, reputationmecene, capitalmecene, nationalitemecene)
    \n VALUES ".to_string();

    for i in 0..50 {

        let name = create_name();

        let capital = create_capital();

        let nationality = create_nationality();
        
        let foobar =
        "\n (id, 'display_name', reput, capital, 'nationality')";
        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);
        human_n = human_n.replace("reput", &create_reputation(0,0).to_string());
        human_n = human_n.replace("capital", &capital.to_string());
        human_n = human_n.replace("nationality", &nationality);
        insert_mecene.push_str(&human_n);
        insert_mecene.push_str(",");

        number_of_creation+=1;
        
    }

    insert_mecene.push_str(";END");
    insert_mecene = insert_mecene.replace(",;END","; \n \n");

    request.push_str(&insert_mecene);

//--RESTAURATEUR------------------------------------------------------------------------------------

    let mut insert_restaurateur =
    "INSERT INTO P1_RESTAURATEUR (idrestaurateur, nomrestaurateur, type, nationaliterestaurateur)
    \n VALUES ".to_string();

    for i in 0..50 {

        let name = create_name();

        let rest_type = create_type();

        let nationality = create_nationality();
        
        let foobar =
        "\n (id, 'display_name', 'type', 'nationality')";
        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);
        human_n = human_n.replace("type", &rest_type);
        human_n = human_n.replace("nationality", &nationality);
        insert_restaurateur.push_str(&human_n);
        insert_restaurateur.push_str(",");

        number_of_creation+=1;
        
    }

    insert_restaurateur.push_str(";END");
    insert_restaurateur = insert_restaurateur.replace(",;END","; \n \n");

    request.push_str(&insert_restaurateur);

//--CRITIQUE------------------------------------------------------------------------------------

    let mut insert_critique =
        "INSERT INTO P1_CRITIQUE (idcritique, nomcritique, reputationcritique, nationalitecritique)
        \n VALUES ".to_string();

        for i in 0..50 {

            let name = create_name();

            let reputation = create_reputation(0,0);

            let nationality = create_nationality();

            let foobar =
            "\n (id, 'display_name', reput, 'nationality')";
            let mut human_n = foobar.replace("id", &i.to_string());
            human_n = human_n.replace("display_name", &name);
            human_n = human_n.replace("reput", &reputation.to_string());
            human_n = human_n.replace("nationality", &nationality);
            insert_critique.push_str(&human_n);
            insert_critique.push_str(",");

            number_of_creation+=1;

        }

        insert_critique.push_str(";END");
        insert_critique = insert_critique.replace(",;END","; \n \n");

        request.push_str(&insert_critique);

//--CREANCIER-----------------------------------------------------------------------------------

    let mut insert_creancier =
        "INSERT INTO P1_CREANCIER (idcreancier, nomcreancier, capitalcreancier, nationalitecreancier)
        \n VALUES ".to_string();

        for i in 0..50 {

            let name = create_name();

            let capital = create_capital();

            let nationality = create_nationality();

            let foobar =
            "\n (id, 'display_name', capital, 'nationality')";
            let mut human_n = foobar.replace("id", &i.to_string());
            human_n = human_n.replace("display_name", &name);
            human_n = human_n.replace("capital", &capital.to_string());
            human_n = human_n.replace("nationality", &nationality);
            insert_creancier.push_str(&human_n);
            insert_creancier.push_str(",");

            number_of_creation+=1;

        }

        insert_creancier.push_str(";END");
        insert_creancier = insert_creancier.replace(",;END","; \n \n");

    request.push_str(&insert_creancier);

//--EXPERT--------------------------------------------------------------------------------------

    let mut insert_expert =
    "INSERT INTO P1_EXPERT (idexpert, nomexpert, specialiteexpert, nationaliteexpert)
    \n VALUES ".to_string();

    for i in 0..50 {

        let name = create_name();

        let specialite = create_type();

        let nationality = create_nationality();

        let foobar =
        "\n (id, 'display_name', 'type', 'nationality')";
        let mut human_n = foobar.replace("id", &i.to_string());
        human_n = human_n.replace("display_name", &name);
        human_n = human_n.replace("type", &specialite);
        human_n = human_n.replace("nationality", &nationality);
        insert_expert.push_str(&human_n);
        insert_expert.push_str(",");

        number_of_creation+=1;

    }

    insert_expert.push_str(";END");
    insert_expert = insert_expert.replace(",;END","; \n \n");

    request.push_str(&insert_expert);

//--GALERIE-------------------------------------------------------------------------------------
//--MARCHE--------------------------------------------------------------------------------------
//--GALERIE-------------------------------------------------------------------------------------

//--RELATIONS-------------------------------------------------------------------------------------

    println!("humans created: {}", number_of_creation);

    

    println!("--------create_.txt---------");

    // /private/student/n/in/fepain/R/art-manipulation
    // E:/Code/projects Rust/art-manipulation/RENDU/humans.txt
	fs::write("/private/student/n/in/fepain/R/art-manipulation/RENDU/humans.txt",
			  request)
		.expect("Unable to write file");
}


