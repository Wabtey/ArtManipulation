/* Hello, this program is meant to create random people
 * to Fullfill our SQL base 'ART MARKET'
*/

/**
 *create One Human base on random name, etc
*/
fn createHuman() {
    println!("Art is dead !");
    let first_Name = pickARandom("FIRST_NAME");
    let last_Name = pickARandom("LAST_NAME");

    let address = createAddress();

    //return first_Name, last_Name, address;


}

fn createArtist() {

    println!("Art is dead !");
    let first_Name = pickARandom("FIRST_NAME");
    let last_Name = pickARandom("LAST_NAME");

    let address = createAddress();

    let request =
    "INSERT INTO "+
    "<P1_ARTISTE>(nomartiste, webartiste, reputationartiste, adresseartiste)"+
    "\n VALUES"

}
