use std::fs;
use serde_json::{Result};
use serde::{Deserialize, Serialize};

use crate::create_fictional_human;

/* Hello, this program is meant to create HTML page
 * to represent our SQL base 'ART MARKET'
*/

//TODO : turn each of those methods into a executable to avoid comment in main

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
	constituent_id: i32,
	display_name: String,
	artist_bio: Option<String>,
	nationality: Option<String>,
	gender: Option<String>,
	begin_date: i16,
	end_date: i16,
	wiki_qid: Option<String>, 
	ulan: Option<String>
}

pub fn create_table_artists() -> Result<()>
{
    let path = "/private/student/n/in/fepain/R/art-manipulation/MoMA/Artists-reformed.json";
	// the file : E:/Code/projects Rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/R/art-manipulation/MoMA/Artists-reformed.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----read_.json-----");
	
    let artists: Vec<Artist> = serde_json::from_str(&content).unwrap();

	// println!("{:?}", artists);
	
	println!("--------create_request---------");

    let mut foo =
    "<a href=\"http://testcgi.istic.univ-rennes1.fr/~fepain/artist.html\">HOME PAGE</a>
    <table>
    \n    <tr> 
    \n         <th>Constituent ID</th> 
    \n         <th>Nom</th> 
    \n         <th>Site Web</th> 
    \n         <th>Reputation</th> 
    \n         <th>Nationalite</th> 
    \n    </tr>".to_string();

    for artist in artists
    {
        let artist_nationality: &str=
		match &artist.nationality {
			Some(s) => s,
			None => "",
		};

        let mut artist_web = artist.display_name
            .replace(" ", "-")
            .replace("'", "")
            .replace(",", "")
            .replace(";", "")
            .replace("(", "")
            .replace(")", "");
        artist_web.push_str(".org");

        let foobar =
        "\n <tr> 
        \n     <td>id</td>
        \n     <td>display_name</td>
        \n     <td>site</td>
        \n     <td>reputation</td>
        \n     <td>nationality</td>
        \n </tr>";
        let mut artist_n = foobar.replace("id", &artist.constituent_id.to_string());
        artist_n = artist_n.replace("display_name", &artist.display_name.replace("'", " "));
        artist_n = artist_n.replace("site", &artist_web);
        artist_n = artist_n.replace("reputation", &create_fictional_human::create_reputation(0,0).to_string());
        artist_n = artist_n.replace("nationality", &artist_nationality.replace("'", " "));
        foo.push_str(&artist_n);
    }
    
    foo.push_str("\n</table>");

    println!("--------create_.html---------");

    // "/private/student/n/in/fepain/R/art-manipulation/RENDU/insert_artists.txt"
    // "E:/Code/projects Rust/art-manipulation/RENDU/insert_artists.txt"
	fs::write("/private/student/n/in/fepain/public_html/artists.html",
			 foo)
		.expect("Unable to write file");

    Ok(())
}