use std::fs;
use serde_json::{Result};
use serde::{Deserialize, Serialize};

mod create_fictional_human;
mod html_creator;

/* Hello, this program is meant to create random people
 * to Fullfill our SQL base 'ART MARKET'
*/

//TODO : turn each of those method into a executable to avoid comment in main

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

fn convert_artists() -> Result<()>
{
    let path = "E:/Code/projects Rust/MoMA/Artists-reformed.json";
	// the file : E:/Code/projects Rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/R/art-manipulation/MoMA/Artists-reformed.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----read_.json-----");
	
    let artists: Vec<Artist> = serde_json::from_str(&content).unwrap();

	// println!("{:?}", artists);
	
	println!("--------create_request---------");

    let mut foo =
    "INSERT INTO P1_ARTISTE (idartiste, nomartiste, webartiste, reputationartiste, nationaliteartiste)
    \n VALUES ".to_string();

    for artist in artists
    {
        let artist_nationality: &str=
		match &artist.nationality {
			Some(s) => s,
			None => "",
		};

        //
        let mut artist_web = artist.display_name
            .replace(" ", ".").replace("'", "").replace(",","");
        artist_web.push_str(".org");

        let foobar =
        "\n (id,'display_name','site', reputation,'nationality')";
        let mut artist_n = foobar.replace("id", &artist.constituent_id.to_string());
        artist_n = artist_n.replace("display_name", &artist.display_name.replace("'", " "));
        artist_n = artist_n.replace("site", &artist_web);
        artist_n = artist_n.replace("reputation", &create_fictional_human::create_reputation(0,0).to_string());
        artist_n = artist_n.replace("nationality", &artist_nationality.replace("'", " "));
        foo.push_str(&artist_n);
        
        foo.push(','); // have to remove the last one
    }
    
    foo.push_str(";END"); //to end the SQL request
    foo = foo.replace(",;END",";");

    println!("--------create_.sql---------");

    // "/private/student/n/in/fepain/R/art-manipulation/RENDU/insert_artists.txt"
    // "E:/Code/projects Rust/art-manipulation/RENDU/insert_artists.txt"
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/insert_artists.sql",
			 foo)
		.expect("Unable to write file");

    Ok(())
}

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

fn convert_artworks() -> Result<()>
{
    let path = "E:/Code/projects_rust/MoMA/Artworks-reformed.json";
	// the file : E:/Code/projects_rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/R/art-manipulation/MoMA/Artists-reformed.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----read_.json-----");
	
    let artworks: Vec<Artwork> = serde_json::from_str(&content).unwrap();

	// println!("{:?}", artists);

	println!("--------create_request---------");

    let mut foo =
    "INSERT INTO P1_ART (idart, titre, typeArt, cote, dateCreation)
    \n VALUES ".to_string();

    for artwork in artworks
    {
        let artwork_medium: &str =
        match &artwork.medium {
            Some(s) => s,
            None => "",
        };

        let artwork_date: &str =
        match &artwork.date {
            Some(s) => {
                s.to_string().push_str("-01-01");
                s
            },
            None => "",
        };

        let artwork_title = artwork.title.replace("'", " ");

        //TODO : find potential ' and replace them by a space

        let foobar =
        "\n (id,'title', 'medium', cote, date)";
        let mut artwork_n = foobar.replace("id", &artwork.object_id.to_string());
        artwork_n = artwork_n.replace("title", &artwork_title.replace("\\", ""));
        // how on earth you can accept '\' in a user insertion... 
        artwork_n = artwork_n.replace("medium", &artwork_medium.replace("'", " ").replace("\\", ""));
        artwork_n = artwork_n.replace("cote", &create_fictional_human::create_reputation(0,0).to_string());
        artwork_n = artwork_n.replace("date", &artwork_date.to_string());
        foo.push_str(&artwork_n);

        foo.push(','); // have to remove the last one
    }
    
    foo.push_str(";END"); //to end the SQL request
    foo = foo.replace(",;END",";");

    println!("--------create_.sql---------");

	fs::write("E:/Code/projects_rust/art-manipulation/RENDU/insert_artworks.sql",
			 foo)
		.expect("Unable to write file");

    Ok(())
}


/**
 * create association between artists and artworks,
 * based on the Artworks-reformed.json which for
 * each artwork have 0 or several artists stored
 * in constituent_id
 * 
 * for each artwork we going through the constituent_id: Vec<i32>
 * if there is more than one constituent_id (id_artiste)
 * if the artwork had none, it go over to the next artwork
 *      create a new line to associate it with the current
 *      artwork_id
 */
fn convert_association() -> Result<()>
{
    let path_artworks = "E:/Code/projects Rust/MoMA/Artworks-reformed.json";

	let content_artworks = fs::read_to_string(path_artworks)
		.expect("Unable to read file");

	println!("-----read_.json-----");

    let artworks: Vec<Artwork> = serde_json::from_str(&content_artworks).unwrap();

	// println!("{:?}", artists);

    println!("--association_artists_artworks--");

    let mut association =
    "INSERT INTO CREE (idartiste, idart)
    \n VALUES ".to_string();

    println!("--idArt now--");

    for artwork in artworks
    {
        
        /* for each artwork we going through the constituent_id: Vec<i32>
         * if the artwork had none, it go over to the next artwork
         * if there is more than one constituent_id (id_artiste)
         *      create a new line to associate it with the current
         *      artwork_id
         */
        for artists_id in artwork.constituent_id{
            let foobar =
            "\n (idartiste, idart)";
            let mut cree_n = foobar.replace("idartiste", &artists_id.to_string());
            cree_n = cree_n.replace("idart", &artwork.object_id.to_string());
            association.push_str(&cree_n);
            association.push(',');
        }
        

    }
    association.push_str(";END"); //to end the SQL request
    association = association.replace(",;END",";");

    println!("--------create_.sql---------");

    // "E:/Code/projects Rust/art-manipulation/RENDU/insert_artworks.txt"
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/insert_cree.sql",
			 association)
		.expect("Unable to write file");

    Ok(())
}


/**
 * create a semi data base of all nationality into a Vec<&str>
 * from the artists.json
 */
fn create_vec_nationality() -> Result<()>
{

    let path = "E:/Code/projects Rust/MoMA/Artists-reformed.json";
	
	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----read_.json-----");
	
    let artists: Vec<Artist> = serde_json::from_str(&content).unwrap();
	
	println!("--------create_request---------");

    let mut foo =
    "static LIST_NATIONALITY: &'static [&str] = &[".to_string();

    //pb : doublon
    for artist in artists
    {
        let artist_nationality: &str=
		match &artist.nationality {
			Some(s) => s,
			None => "",
		};

        
        if !foo.contains(&artist_nationality){
            let foobar = "nationality";
            let nat_n = foobar.replace("nationality", &artist_nationality.replace("'", " "));
            foo.push_str(&nat_n);
            foo.push_str(", "); // have to remove the last one
        }
        
    }
    
    foo.push_str(";END"); //to end the Vec<&str>
    foo = foo.replace(", ;END","];");

    println!("--------create_.txt---------");

	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/nationality.txt",
			 foo)
		.expect("Unable to write file");

    Ok(())

}




fn main() {
    println!("Art is dead !");

    println!("--artists now--");

    // convert_artists().unwrap();

    println!("--artworks now--");

    convert_artworks().unwrap();

    println!("--associations now--");

    // convert_association().unwrap();

    println!("--nationality data base--");

    // create_vec_nationality().unwrap();

    println!("--creation_human--");

    // create_fictional_human::create_requests(500);

    println!("--create arstist table--");

    // html_creator::create_table_artists().unwrap();

    println!("--End--");
}
