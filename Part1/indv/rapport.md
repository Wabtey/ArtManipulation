---
title: Record P1 DSB
tags: DSB_INF
---

Preface

---

Introduction


Nous avons choisi de représenter la 'discrimination' des artistes dans le marché de l'art.
Depuis l'aube des temps, les artistes ont eu du mal à se rémunérer et ce phénomène est d'autant plus fort dans ce monde d'hyper-concurence et néocapitaliste.

D'après les dits de cette video :
The Art Market is a Scam (And Rich People Run It)
https://www.youtube.com/watch?v=ZZ3F3zWiEmc
MINIATURE

Ce marché est très restreint, accessible uniquement par des strates très riche de nos sociétés. En effet, le prix d'entré des marchés où se déroule les ventes les plus 'lucratives' et onéreuses dépasse souvent le salaire moyen d'une vie d'un salarié (source dans un prochain épisode).

La grande majorité des ventes mondiales d'art sont concentrées à New york, Hong-kong et London.

L'art n'a pas de valeur intrisèque.
L'art se voit etre évalué par la somme *hypothétique* que les acheteurs *potentiels* sont prêt à dépenser pour l'acquérir. Il en va donc d'une enchère monétaire, de manipulation de critique, mais aussi de recherche et de travaux d'expert sur certaines oeuvres pour que celle-ci se voit identifiée, voire authentifiée, ce qui a pour cause de monter son prix.

Beaucoup tendent à dire que ce marché acceuille plus de crénacier, que d'artiste.
Les artistes se voient majoritèrement rémunérés par une réputaion plutôt qu'un salaire.

Le problème se pose sur le caractère spéculatif (stéril et fictif), subjectif et concentré de ce système. Ce sont dans ce genre de petit monde où le monopole contrôle tout (les prix, quand pour les ventes, combien d'oeuvre, etc) -> exemple des oeuvres d'Andy Warhol et du presque unique acheteur Jose Mugrabi.

Ces mêmes conditions qu'a permit à Mugrabi de dominer le marché des oeuvres de Warhol, à son influence de fonctionner reflète que la connivence et la corruption s'y confonde facilement.


Nous parlons même pour le marché de l'art d'une composition d'arnaques, par exemple :
Contre-intuitivement, d'ultra-riche individus peuvent gagner en rentabilité en procédant à des donations d'art.
    Pour le cas des US -se trouve également en France-, lorsqu'un de ces individus opère une donation d'art à un musée -à but non-lucratif, sinon l'arnaque irait au delas de l'imaginable-, il obtient une déduction de taxe.
        Pour un don d'une oeuvre valant 10 millions de $, la déduction d'impôt s'applique pour 10 millions, qui en théorie devrait leur profiter de 4 millions.
    Etant donné la difficulté de determiner la valeur d'oeuvre, l'IRS (Internal Revenue Service : organisme de taxation des Etats Unis vis-à-vis de l'art) se voit surpayer 38% des déductions de taxes comparement à la valeur établie plus tard. (source manquante dans la vidéo)
        En résumé, un ultra-riche pourrait acheter un pièce d'art pour 4millions.
        L'apprécier pendant quelque année, négocier pour une évaluation favorable, augmenter sa valeur, se baser sur le fait que l'IRS n'audite (ne contrôle) qu'une infime partie des oeuvres, faire la donation pour 10millions et il aura déjà atteint le seuil de rentabilité.
        
        Ce don aura également pour cause, la présence d'une piece d'une même collection dans le musée, d'augmenter le prix des autres de la collection que ce créancier possède potiellement (monopole).


La majorité des grandes fortunes se construise un empire, échappe au taxation et peuvent -s'ils ne cache pas leur fortune dans des biens 'difficilement' évaluables mais vendable/échangeable- profiter d'éxonération de taxe pour distribuer leur part impôt où elle le souhaite (par les donations).

L'art n'est plus un exercice de compétence, c'est celui d'une marque.




---
Modele conceptuel


Nous avons omis les conservateurs de musée, propriétaire de gallerie et d'autres acteurs dans l'équation de l'évaluation du prix d'une oeuvre. Les rrelations de connaisance entre les différents humains, leur contact, leur société écran, etc. Ce procédé reste complexe et non exhaustivement représenté par cette base de donnée. Il s'agit surtout d'une ébauche.

L'art est au milieu de tout ce système. 
Ce Marché se compose de plusieurs acteurs, ces acteurs vont influencer sur la valeur que l'oeuvre porte.

Une pièce d'oeuvre d'art possède un numéro d'objet (unique), un titre, un medium (type) et une cote (la valeur estimée de l'oeuvre).
Nous avons décidé de séparer le créateur de la pièce, étant donné que son créateur peut etre inconnu ou en avoir plusieurs.

-- humains

Nombreuses tables représentent des humains, pour pouvoir les différencier nous les avons séparé en plusisuers tables.
Ces humains possèdent tous ces *attributs fixes* :
    Un ID unique, un nom et une nationalité.
Certains d'entre peuevnt avoir comme attribut un capital, une spécialité, un site web ou une réputation.

    Les artistes ont un "constituent_id" unique,
puisqu'on créé le profil artiste si l'on sait qu'il existe par la seule création d'une oeuvre, celui ci peut etre complètement inconnu et ne pas avoir de nom, de site web, de réputation, d'orgine (nationalité).
    On associe les artistes avec les artworks par la relation CREE.
Un Artwork peut etre créé par personne ou plusieurs artistes. Et un artiste peut ne rien avoir créé ou avoir une multitude de création.

    Les Mecenes sont peu majoritaire dans le marche de l'art,
il s'agit surtout de créancier qui achetent directement à l'artiste. Ils possèdent les attributs humains fixes, une réputation et leur patrimoine supposé (capital).
Ces Mecènes peuvent ou pas AIDE un ou des artistes, représenté par une somme d'argent : montantAide

    Les Créanciers sont principalement des commerciaux plutôt que des collectionneurs d'art. 
Ils ont les attributs fixes et un capital.
Ils peuvent vendre de l'art, à un certain prix (prixVente), à un certain moment (dateDeVente).
Ils peuvent participer à un seul marché à la fois, dans lequel ils peuvent acheter/vendre des oeuvres répertoriée dans la relation PARTICIPE
Pour les achats de crénacier à créancier privé (sans marché public) on passe par vend et possede.

    Les Commissaires-priseurs ont seulement les attributs fixes.
Ils peuvent diriger un marché. 

    Pour qu'une oeuvre monte en valeur, et s'affiche mieux dans un immense salon, les Restaurateurs travaillent avec les Musées, les galeries et les collectionneurs d'art (créancier).
Ils sont en contact direct avec les oeuvres (relation RESTAURE: pour un certain prix (prixRestauration)).

    Les Critiques sont des personnes publiques émettant un jugement sur certaine oeuvre, mouvement d'art.
Ils jouent un rôle important dans la monter de prix des oeuvres (relation JUGE: en donnant une cote -coteDonnee-).

    Enfin les Experts sont des chercheurs, en Histoire, en Matière, en Technique.
Leur collaboration avec le corps des galeries et créanciers est essentielle pour l'authentification d'oeuvre d'art.
Prouver qu'une pièce est l'original peut faire grimper sa cote de plusieurs millions.
    En prenant l'exemple du portrait du Christ par Léonard de Vinci. 
Les experts ont determiné en analysant les différentes couches de la peinture, jusqu'à l'esquices du portrait où la position du pouce diffère de la version finale, ce qui stupide de représenter lors de la création d'une copie.
Ils ont donc determiné qu'il s'agissait de l'original.
La peinture a explosé de 200$ à 450Millions$.
L'oeuvre est passée dans les mains d'expert, de restaurateurs, de critique et de nombreux créanciers.


-- organismes

    Ce Marché devra être dirigé par un commissaire-priseur et peut avoir aucun ou plusieurs créanciers participant.
Il s'agit de galerie privée, où le prix d'entrée vise à resteindre les acheteurs (n'avoir que la même 'clientèle'). Ils sont unique et ne durent que le jour dateMarche.
Il se déroule à un certain moment, rendez vous à ne pas manquer pour repérer les fluctuations de prix et certaines oeuvres qui peuvent se rentabiliser.

    Les Galeries et les musées sont des organismes présentant des oeuvres (EXPOSE et PRET resp), elles ne peuvent pas ouvrir si ces organismes n'ont pas assez d'oeuvres à exposer.


Beaucoup de relation étaient décritent avec des cardinalités de 0 1, seulement par l'application des règles logiques, ces relations fusionnent avec les tables concernaient et perdent le caractère d'historique intéressant ici pour garder trace et décrire les évènements pour lesquels une oeuvre fluctue de prix.


----------

EXPERT : idExpert, nomExpert, typeExpert, nationaliteExpert
EXPERTISE, 1N EXPERT, 0N ART
POSSEDE, 0N CREANCIER, 0N ART : prixAchat, prixvente, dateDebut, dateFin
CREANCIER : idCreancier, nomCreancier, capitalCreancier, nationaliteCreancier
PARTICIPE, 0N CREANCIER, 0N MARCHE : OeuvreAchetee, OeuvreVendue
MARCHE : idMarche, nomMarche, dateMarche, prixEntryMarche, adresseMarche

RESTAURATEUR : idRestaurateur, nomRestaurateur, typeRestaurateur, nationaliteRestaurateur
RESTAURE, 1N RESTAURATEUR, 0N ART : prixRestauration
ART : idArt, titre, typeArt, cote, dateCreation
CREE, 0N ARTISTE, 0N ART : dureeCreation
ARTISTE : idArtiste, nomArtiste, webArtiste, reputationArtiste, nationaliteArtiste
DIRIGE, 01 COMMISSAIRES-PRISEURS, 11 MARCHE

GALERIE : idGalerie, nomGalerie, dateGalerie, prixEntryExpo, association, adresseGalerie
EXPOSE, 0N ART, NN GALERIE : dureeExpo
PRET, 0N ART, NN MUSEE : dureePret
JUGE, 0N CRITIQUE, 0N ART : coteDonnee
AIDE, 0N MECENE, 0N ARTISTE : montantAide
COMMISSAIRES-PRISEURS : idCommissaire, nomCommissaire, nationaliteCommissaire

::
MUSEE : idMusee, nomMusee, dateDeCreation, adresseMusee
CRITIQUE : idCritique, nomCritique, reputationCritique, nationaliteCritique
MECENE : idMecene, nomMecene, reputationMecene, capitalMecene, nationaliteMecene
:

--------


P1_COMMISSAIRE_PRISEUR ( idCommissaire, nomCommissaire, nationaliteCommissaire )
P1_RESTAURATEUR ( idRestaurateur, nomRestaurateur, typeRestaurateur, nationaliteRestaurateur )
P1_MUSEE ( idMusee, nomMusee, dateDeCreation, adresseMusee )
EXPOSE ( idArt, idGalerie, dureeExpo )
P1_GALERIE ( idGalerie, nomGalerie, dateGalerie, prixEntryExpo, association, adresseGalerie )
RESTAURE ( idRestaurateur, idArt, prixRestauration )
PRET ( idArt, idMusee, dureePret )
JUGE ( idCritique, idArt, coteDonnee )
P1_CRITIQUE ( idCritique, nomCritique, reputationCritique, nationaliteCritique )
P1_MARCHE ( idMarche, nomMarche, dateMarche, prixEntryMarche, adresseMarche, idCommissaire )
P1_ART ( idArt, titre, typeArt, cote, dateCreation )
EXPERTISE ( idExpert, idArt )
P1_EXPERT ( idExpert, nomExpert, typeExpert, nationaliteExpert )
P1_MECENE ( idMecene, nomMecene, reputationMecene, capitalMecene, nationaliteMecene )
PARTICIPE ( idCreancier, idMarche, OeuvreAchetee, OeuvreVendue )
P1_CREANCIER ( idCreancier, nomCreancier, capitalCreancier, nationaliteCreancier )
POSSEDE ( idCreancier, idArt, prixAchat, prixVente, dateDebut, dateFin )
CREE ( idArtiste, idArt )
P1_ARTISTE ( idArtiste, nomArtiste, webArtiste, reputationArtiste, nationaliteArtiste )
AIDE ( idMecene, idArtiste )


---


ART ( idArt, titre, typeArt, cote, dateCreation ) \newline
ARTISTE ( idArtiste, nomArtiste, webArtiste, reputationArtiste, nationaliteArtiste )\newline
EXPERT ( idExpert, nomExpert, typeExpert, nationaliteExpert )\newline
MECENE ( idMecene, nomMecene, reputationMecene, capitalMecene, nationaliteMecene )\newline
CRITIQUE ( idCritique, nomCritique, reputationCritique, nationaliteCritique )\newline
COMMISSAIR-PRISEUR ( idCommissaire, nomCommissaire, nationaliteCommissaire )\newline
RESTAURATEUR ( idRestaurateur, nomRestaurateur, typeRestaurateur, nationaliteRestaurateur )\newline
MARCHE (idMarche, nomMarche, dateMarche, prixEntryMarche, adresseMarche, idCommissaire)\newline
MUSEE ( idMusee, nomMusee, dateDeCreation, adresseMusee )\newline
GALERIE ( idGalerie, nomGalerie, dateGalerie, prixEntryExpo, association, adresseGalerie )\newline
\\
EXPERTISE ( idExpert, idArt )\newline
PARTICIPE ( idCreancier, idMarche, OeuvreAchetee, OeuvreVendue )\newline
CREANCIER ( idCreancier, nomCreancier, capitalCreancier, nationaliteCreancier )\newline
POSSEDE ( idCreancier, idArt, prixAchat, prixVente, dateDebut, dateFin )\newline
CREE ( idArtiste, idArt, dureeCreation )\newline
RESTAURE ( idRestaurateur, idArt, prixRestauration )\newline
PRET ( idArt, idMusee, dureePret )\newline
JUGE ( idCritique, idArt, coteDonnee )\newline
AIDE ( idMecene, idArtiste )\newline
EXPOSE ( idArt, idGalerie, dureeExpo )\newline


---
Modele Relationnel

a) Application règle 1

COMMISSAIRES-PRISEURS ( idCommissaire, nomCommissaire*, nationaliteCommissaire )
RESTAURATEUR ( idRestaurateur, nomRestaurateur*, typeRestaurateur*, nationaliteRestaurateur )
MUSEE ( idMusee, nomMusee*, dateDeCreation*, adresseMusee )
GALERIE ( idGalerie, nomGalerie*, dateGalerie*, prixEntryExpo, association, adresseGalerie* )
CRITIQUE ( idCritique, nomCritique*, reputationCritique, nationaliteCritique )
MARCHE ( idMarche, nomMarche*, dateMarche*, prixEntryMarche, adresseMarche*)
ART ( idArt, titre, typeArt, cote, dateCreation )
// pb : l'importation de la BDD MoMA met 0 à la place du vide pour bcp d'attrib

EXPERT ( idExpert, nomExpert*, typeExpert*, nationaliteExpert )
MECENE ( idMecene, nomMecene*, reputationMecene, capitalMecene, nationaliteMecene )
CREANCIER ( idCreancier, nomCreancier*, capitalCreancier, nationaliteCreancier )
ARTISTE ( idArtiste, nomArtiste, webArtiste, reputationArtiste, nationaliteArtiste )

b) Application règle 2 et 2bis

Les 0 1 et 1 1 se transforment en clé étrangères et supprime la relation qui est alors inutile.
Ici le marché étant unique, il n'est dirigé que par un seul commissaire-priseur, la relation dirige est alors incorporée dans la table marché.

MARCHE ( idMarche, nomMarche*, dateMarche*, prixEntryMarche, adresseMarche*, <u>idcomissaire</u>)

c) Application règle 3

Ces relations deviennent des tables contenant des clés étrangères pour permettre à toutes informations de pouvoir être accèder sans ambiguité.
Et les relations inutiles sont supprimées 

PARTCIPE(idMarche*, idCreancier*, OeuvreAchetee, OeuvreVendue)
// OeuvreAch, et OeuvreVend correspondent à des idArt

POSSEDE(idCreancier*, idArt*, prixAchat, prixVente, dateDebut*, dateFin)
// Les deux relation POSSEDE et VEND sont fusionné afin d'éviter les conflits
// Le prix peut etre privé (sans trop d'interet dans le milieu de la spéculation)

RESTAURE(idRestaurateur*, idArt*, prixRestauration)
PRET(idMusee*, idArt*, dureePret*)
EXPOSE(idGalerie*, idArt*, dureeExpo*)
JUGE(idCritique*, idArt*, coteDonnee*)
EXPERTISE(idExpert*, idArt*)
CREE(idArtiste*, idArt*)
AIDE(idMecene*, idArtiste*)

---

IV. Schéma physique

CREATE DATABASE IF NOT EXISTS `MERISE` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;
USE `MERISE`;

DROP TABLE IF EXISTS POSSEDE;
DROP TABLE IF EXISTS CREE;
DROP TABLE IF EXISTS PARTICPE;
DROP TABLE IF EXISTS EXPOSE;
DROP TABLE IF EXISTS RESTAURE;
DROP TABLE IF EXISTS PRET;
DROP TABLE IF EXISTS JUGE;
DROP TABLE IF EXISTS EXPERTISE;
DROP TABLE IF EXISTS AIDE;
DROP TABLE IF EXISTS P1_ART;
DROP TABLE IF EXISTS P1_ARTISTE;
DROP TABLE IF EXISTS P1_COMMISSAIRE_PRISEUR;
DROP TABLE IF EXISTS P1_MARCHE;
DROP TABLE IF EXISTS P1_RESTAURATEUR;
DROP TABLE IF EXISTS P1_MUSEE;
DROP TABLE IF EXISTS P1_GALERIE;
DROP TABLE IF EXISTS P1_CRITIQUE;
DROP TABLE IF EXISTS P1_EXPERT;
DROP TABLE IF EXISTS P1_MECENE;
DROP TABLE IF EXISTS P1_CREANCIER;


CREATE TABLE `P1_ART` (
  `idart` INT NOT NULL,
  `titre` VARCHAR(42),
  `type` VARCHAR(42),
  `cote` VARCHAR(42),
  `date` VARCHAR(42),
  PRIMARY KEY (`idart`)
);

CREATE TABLE `P1_ARTISTE` (
  `idartiste` INT NOT NULL,
  `nomartiste` VARCHAR(42),
  `webartiste` VARCHAR(42),
  `reputationartiste` INT,
  `nationaliteartiste` VARCHAR(42),
  PRIMARY KEY (`idartiste`)
);

CREATE TABLE `P1_COMMISSAIRE_PRISEUR` (
  `idcommissaire` INT NOT NULL,
  `nomcommissaire` VARCHAR(42) NOT NULL,
  `nationalitecommissaire` VARCHAR(42) NOT NULL,
  PRIMARY KEY (`idcommissaire`)
);

CREATE TABLE `P1_MARCHE` (
  `idmarche` INT NOT NULL,
  `nommarche` VARCHAR(42) NOT NULL,
  `datemarche` DATE NOT NULL,
  `prixentrymarche` VARCHAR(42),
  `adressemarche` VARCHAR(42) NOT NULL,
  PRIMARY KEY (`idmarche`),
  FOREIGN KEY (`idcommissaire`)
  REFERENCES COMMISSAIRE_PRISEUR(idcommissaire)
);


CREATE TABLE `P1_RESTAURATEUR` (
  `idrestaurateur` INT NOT NULL,
  `nomrestaurateur` VARCHAR(42) NOT NULL,
  `typerestaurateur` VARCHAR(100) NOT NULL,
  `nationaliterestaurateur` VARCHAR(42) NOT NULL,
  PRIMARY KEY (`idrestaurateur`)
);

CREATE TABLE `P1_MUSEE` (
  `idmusee` INT NOT NULL,
  `nommusee` VARCHAR(42) NOT NULL,
  `datedecreation` DATE NOT NULL,
  `adressemusee` VARCHAR(42) NOT NULL,
  PRIMARY KEY (`idmusee`)
);

CREATE TABLE `P1_GALERIE` (
  `idgalerie` INT NOT NULL,
  `nomgalerie` VARCHAR(42) NOT NULL,
  `dategalerie` DATE NOT NULL,
  `prixentryexpo` INT,
  `association` VARCHAR(42),
  `adressegalerie` VARCHAR(42) NOT NULL,
  PRIMARY KEY (`idgalerie`)
);

CREATE TABLE `P1_CRITIQUE` (
  `idcritique` INT NOT NULL,
  `nomcritique` VARCHAR(42) NOT NULL,
  `reputationcritique` INT,
  `nationalitecritique` VARCHAR(42),
  PRIMARY KEY (`idcritique`)
);

CREATE TABLE `P1_EXPERT` (
  `idexpert` INT NOT NULL,
  `nomexpert` VARCHAR(42) NOT NULL,
  `typeexpert` VARCHAR(42) NOT NULL,
  `nationaliteexpert` VARCHAR(42),
  PRIMARY KEY (`idexpert`)
);

CREATE TABLE `P1_MECENE` (
  `idmecene` INT NOT NULL,
  `nommecene` VARCHAR(42) NOT NULL,
  `reputationmecene` INT,
  `capitalmecene` INT,
  `nationalitemecene` VARCHAR(42),
  PRIMARY KEY (`idmecene`)
);

CREATE TABLE `P1_CREANCIER` (
  `idcreancier` INT NOT NULL,
  `nomcreancier` VARCHAR(42) NOT NULL,
  `capitalcreancier` INT,
  `nationalitecreancier` VARCHAR(42),
  PRIMARY KEY (`idcreancier`)
);




CREATE TABLE `POSSEDE` (
  `idcreancier` INT,
  `idart` INT NOT NULL,
  `prixAchat` INT,
  `prixVente` INT,
  `datedebut` DATE NOT NULL,
  `datefin` DATE,
  PRIMARY KEY (`idcreancier`, `idart`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_CREANCIER(idcreancier)
);

CREATE TABLE `CREE` (
  `idartiste` INT NOT NULL,
  `idart` INT NOT NULL,
  PRIMARY KEY (`idartiste`, `idart`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idartiste)
  REFERENCES P1_ARTISTE(idartiste)
);

CREATE TABLE `PARTICIPE` (
  `idcreancier` INT NOT NULL,
  `idmarche` INT NOT NULL,
  `oeuvreachetee` INT,
  `oeuvrevendue` INT,
  PRIMARY KEY (`idcreancier`, `idmarche`),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_CREANCIER(idcreancier),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_MARCHE(idmarche),
    FOREIGN KEY (oeuvreachetee)
  REFERENCES P1_ART(idart),
    FOREIGN KEY (oeuvrevendue)
  REFERENCES P1_ART(idart)
);

CREATE TABLE `EXPOSE` (
  `idart` INT NOT NULL,
  `idgalerie` INT NOT NULL,
  `dureeexpo` DATE NOT NULL,
  PRIMARY KEY (`idart`, `idgalerie`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idgalerie)
  REFERENCES P1_GALERIE(idgalerie)
);

CREATE TABLE `RESTAURE` (
  `idrestaurateur` INT NOT NULL,
  `idart` INT NOT NULL,
  `prixrestauration` INT,
  PRIMARY KEY (`idrestaurateur`, `idart`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idrestaurateur)
  REFERENCES P1_RESTAURATEUR(idrestaurateur)
);
CREATE TABLE `PRET` (
  `idart` INT NOT NULL,
  `idmusee` INT NOT NULL,
  `dureepret` DATE NOT NULL,
  PRIMARY KEY (`idart`, `idmusee`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idmusee)
  REFERENCES P1_MUSEE(idmusee)
);

CREATE TABLE `JUGE` (
  `idcritique` INT NOT NULL,
  `idart` INT NOT NULL,
  `cotedonnee` INT NOT NULL,
  PRIMARY KEY (`idcritique`, `idart`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idcritique)
  REFERENCES P1_CRITIQUE(idcritique)
);

CREATE TABLE `EXPERTISE` (
  `idexpert` INT NOT NULL,
  `idart` INT NOT NULL,
  PRIMARY KEY (`idexpert`, `idart`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idexpert)
  REFERENCES P1_EXPERT(idexpert)
);

CREATE TABLE `AIDE` (
  `idmecene` INT NOT NULL,
  `idartiste` INT NOT NULL,
  PRIMARY KEY (`idmecene`, `idartiste`),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idmecene)
  REFERENCES P1_MECENE(idmecene)
);

---

V. Peuplement des Tables uwu

Nous avons utilisé seulement quelque informations contenues dans la base de données MoMA :
	- Pour les artistes :
		- DisplayName (leur nom)
		- Nationality (leur origine)
  - Pour les oeuvres d art :
    - Title
    - Artist
    - Medium

Toutes les autres informations contenues dans notre base de données
n est que pure supposition et imagination (cote d une oeuvre / reputation d un artiste / etc) et n'a pas rapport à MoMA.

Pour l'importation des .json en sql j'ai codé cette partie avec `serde` en rust :

**use and dependencies**
```rust
use std::fs;
use serde_json::{Result};
use serde::{Deserialize, Serialize};
```
```toml
[dependencies]
serde = { version = "1.0.104", features = ["derive"] }
serde_json = "1.0.48"
rand = "0.8.4"
```

**ARTISTES**
```rust
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
            .replace(" ", ".").replace("'", "");
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

    // "/private/student/n/in/fepain/R/art-manipulation/RENDU/insert_artists.sql"
    // "E:/Code/projects Rust/art-manipulation/RENDU/insert_artists.sql"
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/insert_artists.sql",
			 foo)
		.expect("Unable to write file");

    Ok(())
}
```

**ARTWORKS**
```rust
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
  let path = "E:/Code/projects Rust/MoMA/Artworks-reformed.json";
	// the file : E:/Code/projects Rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/R/art-manipulation/MoMA/Artists-reformed.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	println!("-----read_.json-----");
	
    let artworks: Vec<Artwork> = serde_json::from_str(&content).unwrap();

	println!("--------create_request---------");

    let mut foo =
    "INSERT INTO P1_ART (idart, titre, type, cote)
    \n VALUES ".to_string();

    for artwork in artworks
    {
        let artwork_medium: &str =
        match &artwork.medium {
            Some(s) => s,
            None => "",
        };

        let artwork_title = artwork.title.replace("'", " ");

        let foobar =
        "\n (id,'title', 'medium', cote)";
        let mut artwork_n = foobar.replace("id", &artwork.object_id.to_string());
        artwork_n = artwork_n.replace("title", &artwork_title);
        artwork_n = artwork_n.replace("medium", &artwork_medium.replace("'", " "));
        artwork_n = artwork_n.replace("cote", &create_fictional_human::create_reputation(0,0).to_string());
        foo.push_str(&artwork_n);

        foo.push(','); // have to remove the last one
    }
    
    foo.push_str(";END"); //to end the SQL request
    foo = foo.replace(",;END",";");

    println!("--------create_.sql---------");

    // "E:/Code/projects Rust/art-manipulation/RENDU/insert_artworks.sql"
	fs::write("E:/Code/projects Rust/art-manipulation/RENDU/insert_artworks.sql",
			 foo)
		.expect("Unable to write file");

    Ok(())
}
```

**CREE**
```rust
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
```

**execute**
```rust
fn main() {
    println!("Art is dead !");

    println!("--artists now--");

    convert_artists().unwrap();

    println!("--artworks now--");

    convert_artworks().unwrap();

    println!("--associations now--");

    convert_association().unwrap();

    println!("--End--");
}
```

Maintenant la partie de création ultra fictive d'humain

J'ai créé des tableaux de string primaire pour remplir les champs de mes futures humains :
```rust
static LIST_FIRST_NAME: &'static [&str] = &["Adrien", "Nelson", "Benoit", "Morgan", "Florian",
"Thomas", "Maeto", "Clementine", "Stephane", "Otto", "Jan", "Patrick", "Rudolf", "Pietra",
"Lex", "Nancy", "Waltercio", "Peter", "George", "Jotaro", "Joseph", "Jonathan", "Joe", "Maria",
"Laurence", "Emil", "Usul", "Wout", "Montse", "Bent", "Nobert", "Marcel", "Arian", "Hella"];

static LIST_LAST_NAME: &'static [&str] = &["Strömholm", "Strömholm", "Rusten", "Russomagno", "Biddle",
"Burckhardt", "Martin", "Pinney", "Neusüss", "Goldes", "Charlesworth", "Bernsten", "Appelt", "Cohen",
"McCarthy", "Wagner", "Van Toorn", "Mol", "Lynch", "Bell", "Cassell", "Schönthal", "Kammerer", "Hoppe",
"De Vringer", "Faydherbe", "Marqués", "Nobert", "Smith", "Knoll", "Rizzatto", "Tetrarc", "Wanders",
"van der Meulen", "Joestar", "Abitbol", "Zepelli", "Speedwagon"];

static LIST_ARTWORK_TYPE: &'static [&str] = &["Sculture", "Paint", "Ceramics", "Graphic Art", "Illuminated Manuscripts",
"Jewellery Art", "Metalwork Art", "Mosaic Art", "Photography", "Architecture", "Religious Art", "Rock Art",
"Stained Glass Art"];

// unlucky art market world is not kind :/
static LIST_ASSOCIATION: &'static [&str] = &["", ""];
```

j'ai importé cette liste depuis artists.json de MoMA
```rust
static LIST_NATIONALITY: &'static [&str] = &["nationality unknown", "American", "Spanish", "Danish", "Italian", "French", "Estonian", "Mexican", "Swedish", "Israeli", "British", 
"Finnish", "Polish", "Palestinian", "Japanese", "Guatemalan", "Colombian", "Romanian", "Russian", "German", "Argentine", "Kuwaiti", "Belgian", "Dutch", "Norwegian", 
"Chilean", "Swiss", "Costa Rican", "Czech", "Brazilian", "Austrian", "Canadian", "Australian", "Ukrainian", "Hungarian", "Haitian", "Congolese", 
"Bolivian", "Cuban", "Yugoslav", "Portuguese", "Indian", "Peruvian", "Icelandic", "Irish", "Guyanese", "Uruguayan", "Slovak", "Croatian", "Greek", "Chinese", "Venezuelan", 
"Turkish", "Panamanian", "Algerian", "Ecuadorian", "South African", "Iranian", "Korean", "Canadian Inuit", "Paraguayan", "Luxembourgish", "Nicaraguan", "Zimbabwean", 
"Moroccan", "Slovenian", "Tanzanian", "Bulgarian", "Tunisian", "Sudanese", "Taiwanese", "Ethiopian", "Scottish", "Latvian", "Senegalese", "Thai", "New Zealander", "Lithuanian", 
"Pakistani", "Bahamian", "Bosnian", "Malian", "Czechoslovakian", "Georgian", "Egyptian", "Kenyan", "Emirati", "Nigerian", "Cypriot", "Albanian", "Azerbaijani", "Ivorian", "Malaysian", 
"Serbian", "Singaporean", "Namibian", "Cambodian", "Ghanaian", "Afghan", "Native American", "Lebanese", "Kyrgyzstani", "Vietnamese", "Ugandan", "Cameroonian", "Welsh", "Macedonian", 
"Puerto Rican", "Catalan", "Filipino", "Sahrawi", "Bangladeshi", "Coptic", "Persian", "Burkinabe", "Beninese", "Sierra Leonean", "Salvadoran"];
```



---

VI. Requetes SQL

a) Projection :

	--Les noms des oeuvres d'art des Artistes avec un grand A

SELECT titre
FROM P1_ART Join P1_ARTISTE on nomArtiste
WHERE reputaionArtiste > 1200
;

	--Les oeuvres d'art dont on ignore le createur
 
SELECT titre
FROM P1_ART
WHERE nomArtiste IS NULL
;

b) Jointure : Les Mecenes qui achetent aussi de l'art

SELECT *
FROM P1_MECENE Natural Join P1_CREANCIER on nomCreancier=nomMecene
;

c) Moyenne sur l'integralite d'un attribut : La moyenne du capital des créanciers

SELECT AVG(capitalCreancier) as capitalCreancier.MOY
FROM P1_CREANCIER
;

d) Un regroupement par calcul : 

	--Les prix d'entree au Marche d'art les plus bas, actuel (qui ne sont pas expirée)
	
SELECT MIN(prixEntryMarche)
FROM P1_MARCHE
WHERE dateMarche > CURDATE()
;
			  
	--Les Marches d'art dont leur billet d'entree sont en dessous de la moyenne
	
SELECT nomMarche
FROM P1_MARCHE
WHERE dateMarche > CURDATE() AND
      prixEntryMarche < (SELECT AVG(prixEntryMarche)
			                   FROM P1_MARCHE
			                   WHERE dateMarche > CURDATE())
;

e) Une Différence : Les artistes qui n'ont rien créé

SELECT nomArtiste -- DISTINCT non nécéssaire
FROM P1_ARTISTE
WHERE nomArtiste NOT IN (SELECT DISTINCT nomArtiste
                         FROM P1_ART)
;

f) Une Division : Noms des Creanciers ayant possedé toutes les oeuvres d'art
	--peut faire la même chose avec les Musees (Création d'une nouvelle table
	--pour se rappeler par quels musées une oeuvre est passée : relation pret works)

SELECT nomCreancier
FROM P1_CREANCIER
WHERE (SELECT COUNT(DISTINCT *)
       FROM P1_POSSEDE
       WHERE P1_POSSEDE.idART = P1_ART.idArt --P1_POSSEDE.#idArt
       AND   P1_POSSEDE.idCreancier = P1_CREANCIER.idCreancier)
      = (SELECT COUNT(DISTINCT *) from P1_ART)
;

g) Un Group By :

	-- Nombre d'artistes par ville

SELECT villeArtiste, COUNT(*)
FROM P1_ARTISTE
GROUP BY villeArtiste
;

	-- Donner pour chaque artiste sa cote moyenne (de ses oeuvres)
	
SELECT nomArtiste, AVG(cote)
FROM P1_ARTISTE Join P1_ART
GROUP BY nomArtiste
;

