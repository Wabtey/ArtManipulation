use rand::{thread_rng, Rng};

use std::fs;
use serde_json::{Result};
use serde::{Deserialize, Serialize};

/* Hello, this program is meant to create random people
 * to Fullfill our SQL base 'ART MARKET'
*/

/*
 * reference : 
 * https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
 */

fn create_reputation(_number_of_creation: i32, _base_reput: i32)->i32 {
    let mut rng = thread_rng();
    return rng.gen_range(1..2000);
}


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
    let path = "/private/student/n/in/fepain/R/ArtManipulation/MoMA/Artists-reformed.json";
	// the file : E:/Code/projects Rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/R/ArtManipulation/MoMA/Artists-reformed.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----typed_convert-----");
	
    let artists: Vec<Artist> = serde_json::from_str(&content).unwrap();

	// println!("{:?}", artists);
	
	println!("--------marker---------");

    let mut foo =
    "INSERT INTO <P1_ARTISTE>(nomartiste, webartiste, reputationartiste, nationaliteartiste)
    \n VALUES ".to_string();

    for artist in artists
    {
        let artist_nationality: &str=
		match &artist.nationality {
			Some(s) => s,
			None => "",
		};
        let mut artist_display_name = artist.display_name
            .replace(" ", ".");
        artist_display_name.push_str(".org");

        let foobar =
        "\n ('display_name','site', 'reputation','nationality'),";
        let mut artist1 = foobar.replace("display_name", &artist.display_name);
        artist1 = artist1.replace("site", &artist_display_name);
        artist1 = artist1.replace("reputation", &create_reputation(0,0).to_string());
        artist1 = artist1.replace("nationality", artist_nationality);
        foo.push_str(&artist1);
    }
    
    foo.push(';'); //to end the SQL request

	fs::write("/private/student/n/in/fepain/R/ArtManipulation/RENDU/insert_artists.txt",
			 foo)
		.expect("Unable to write file");

    Ok(())
}

fn main() {
    println!("Art is dead !");

    convert_artists().unwrap();

    println!("--End--");
}
