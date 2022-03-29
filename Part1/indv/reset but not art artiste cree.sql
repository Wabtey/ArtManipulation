DROP TABLE IF EXISTS P1_POSSEDE;
DROP TABLE IF EXISTS P1_PARTICIPE;
DROP TABLE IF EXISTS P1_EXPOSE;
DROP TABLE IF EXISTS P1_RESTAURE;
DROP TABLE IF EXISTS P1_PRET;
DROP TABLE IF EXISTS P1_JUGE;
DROP TABLE IF EXISTS P1_EXPERTISE;
DROP TABLE IF EXISTS P1_AIDE;
DROP TABLE IF EXISTS P1_CRITIQUE;
DROP TABLE IF EXISTS P1_EXPERT;
DROP TABLE IF EXISTS P1_MECENE;
DROP TABLE IF EXISTS P1_RESTAURATEUR;
DROP TABLE IF EXISTS P1_MUSEE;
DROP TABLE IF EXISTS P1_GALERIE;
DROP TABLE IF EXISTS P1_CREANCIER;
DROP TABLE IF EXISTS P1_MARCHE;
DROP TABLE IF EXISTS P1_COMMISSAIRE_PRISEUR;


CREATE TABLE P1_COMMISSAIRE_PRISEUR (
  idcommissaire_priseur INT NOT NULL,
  nomcommissaire_priseur VARCHAR(1000) NOT NULL,
  nationalitecommissaire_priseur VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idcommissaire_priseur)
);

CREATE TABLE P1_MARCHE (
  idmarche INT NOT NULL,
  nommarche VARCHAR(1000) NOT NULL,
  datemarche DATE NOT NULL,
  prixmarche VARCHAR(1000),
  adressemarche VARCHAR(1000) NOT NULL,
  idcommissaire_priseur INT NOT NULL,
  PRIMARY KEY (idmarche),
  FOREIGN KEY (idcommissaire_priseur)
  REFERENCES P1_COMMISSAIRE_PRISEUR(idcommissaire_priseur)
);


CREATE TABLE P1_RESTAURATEUR (
  idrestaurateur INT NOT NULL,
  nomrestaurateur VARCHAR(1000) NOT NULL,
  typerestaurateur VARCHAR(100) NOT NULL,
  nationaliterestaurateur VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idrestaurateur)
);

CREATE TABLE P1_MUSEE (
  idmusee INT NOT NULL,
  nommusee VARCHAR(1000) NOT NULL,
  datemusee DATE NOT NULL,
  prixmusee INT,
  adressemusee VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idmusee)
);

CREATE TABLE P1_GALERIE (
  idgalerie INT NOT NULL,
  nomgalerie VARCHAR(1000) NOT NULL,
  dategalerie DATE NOT NULL,
  prixgalerie INT,
  association VARCHAR(1000),
  adressegalerie VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idgalerie)
);

CREATE TABLE P1_CRITIQUE (
  idcritique INT NOT NULL,
  nomcritique VARCHAR(1000) NOT NULL,
  reputationcritique INT,
  nationalitecritique VARCHAR(1000),
  PRIMARY KEY (idcritique)
);

CREATE TABLE P1_EXPERT (
  idexpert INT NOT NULL,
  nomexpert VARCHAR(1000) NOT NULL,
  typeexpert VARCHAR(1000) NOT NULL,
  nationaliteexpert VARCHAR(1000),
  PRIMARY KEY (idexpert)
);

CREATE TABLE P1_MECENE (
  idmecene INT NOT NULL,
  nommecene VARCHAR(1000) NOT NULL,
  reputationmecene INT,
  capitalmecene INT,
  nationalitemecene VARCHAR(1000),
  PRIMARY KEY (idmecene)
);

CREATE TABLE P1_CREANCIER (
  idcreancier INT NOT NULL,
  nomcreancier VARCHAR(1000) NOT NULL,
  capitalcreancier INT,
  nationalitecreancier VARCHAR(1000),
  PRIMARY KEY (idcreancier)
);




CREATE TABLE P1_POSSEDE (
  idcreancier INT NOT NULL,
  idart INT NOT NULL,
  prixAchat INT,
  prixVente INT,
  datedebutPossede DATE NOT NULL,
  datefinPossede DATE,
  PRIMARY KEY (idcreancier, idart),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_CREANCIER(idcreancier)
);

CREATE TABLE P1_PARTICIPE (
  idcreancier INT NOT NULL,
  idmarche INT NOT NULL,
  PRIMARY KEY (idcreancier, idmarche),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_CREANCIER(idcreancier),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_MARCHE(idmarche)
);

CREATE TABLE P1_EXPOSE (
  idart INT NOT NULL,
  idgalerie INT NOT NULL,
  datedebutexpose DATE NOT NULL,
  datefinexpose DATE NOT NULL,
  PRIMARY KEY (idart, idgalerie),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idgalerie)
  REFERENCES P1_GALERIE(idgalerie)
);

CREATE TABLE P1_RESTAURE (
  idrestaurateur INT NOT NULL,
  idart INT NOT NULL,
  prixrestaure INT,
  PRIMARY KEY (idrestaurateur, idart),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idrestaurateur)
  REFERENCES P1_RESTAURATEUR(idrestaurateur)
);
CREATE TABLE P1_PRET (
  idart INT NOT NULL,
  idmusee INT NOT NULL,
  datedebutpret DATE NOT NULL,
  datefinpret DATE NOT NULL,
  PRIMARY KEY (idart, idmusee),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idmusee)
  REFERENCES P1_MUSEE(idmusee)
);

CREATE TABLE P1_JUGE (
  idcritique INT NOT NULL,
  idart INT NOT NULL,
  prixJuge INT NOT NULL,
  PRIMARY KEY (idcritique, idart),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idcritique)
  REFERENCES P1_CRITIQUE(idcritique)
);

CREATE TABLE P1_EXPERTISE (
  idexpert INT NOT NULL,
  idart INT NOT NULL,
  PRIMARY KEY (idexpert, idart),
  FOREIGN KEY (idart)
  REFERENCES P1_ART(idart),
  FOREIGN KEY (idexpert)
  REFERENCES P1_EXPERT(idexpert)
);

CREATE TABLE P1_AIDE (
  idmecene INT NOT NULL,
  idartiste INT NOT NULL,
  prixAide INT,
  PRIMARY KEY (idmecene, idartiste),
  FOREIGN KEY (idartiste)
  REFERENCES P1_ARTISTE(idartiste),
  FOREIGN KEY (idmecene)
  REFERENCES P1_MECENE(idmecene)
);
