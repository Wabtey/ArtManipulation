/* Hello, this program is meant to create random people
 * to Fullfill our SQL base 'ART MARKET'
*/

static FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

static LAST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane"];

fn createReputation(numberOfCreation: int, baseReput: int) {
    return rng.gen_range(1..2000);
}

fn createAddress(){
    return "";
}

fn createArtist() {

    let mut rng = thread_rng();

    // let FIRST_NAME = (Adrien, Nelson, Benoit, Morgan, Florian,
    // Thomas, Maeto, Elebane, Stephane)

    println!("Art is dead !");
    let first_Name: String = rng.choose(&FIRST_NAME[..0]);
    let last_Name: String = rng.choose(&LAST_NAME[..0]);

    let address = createAddress();

    //createReputation(number of creation, base reput)
    let reputationartiste = createReputation(10, 0);

    let request =
    "INSERT INTO "+
    "<P1_ARTISTE>(nomartiste, webartiste, reputationartiste, adresseartiste)"+
    "\n VALUES ('"+first_Name+" "+last_Name+"',"+
               "'"+first_Name+"-"+last_Name+".org',"+
               "'"+reputationartiste+"',"+
               "'"+address+"',"+
               ")";

    return request;
}

fn main() {
    let request = createArtist();
    print(request)
}
