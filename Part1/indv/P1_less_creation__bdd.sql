DROP TABLE IF EXISTS POSSEDE;
DROP TABLE IF EXISTS CREE;
DROP TABLE IF EXISTS PARTICIPE;
DROP TABLE IF EXISTS EXPOSE;
DROP TABLE IF EXISTS RESTAURE;
DROP TABLE IF EXISTS PRET;
DROP TABLE IF EXISTS JUGE;
DROP TABLE IF EXISTS EXPERTISE;
DROP TABLE IF EXISTS AIDE;
DROP TABLE IF EXISTS CRITIQUE;
DROP TABLE IF EXISTS EXPERT;
DROP TABLE IF EXISTS ARTISTE;
DROP TABLE IF EXISTS MECENE;
DROP TABLE IF EXISTS RESTAURATEUR;
DROP TABLE IF EXISTS MUSEE;
DROP TABLE IF EXISTS GALERIE;
DROP TABLE IF EXISTS ART;
DROP TABLE IF EXISTS CREANCIER;
DROP TABLE IF EXISTS MARCHE;
DROP TABLE IF EXISTS COMMISSAIRE_PRISEUR;

CREATE TABLE ART (
  idart INT NOT NULL,
  titre VARCHAR(1000),
  typeArt VARCHAR(1000),
  cote INT,
  dateArt DATE,
  PRIMARY KEY (idart)
);

CREATE TABLE ARTISTE (
  idartiste INT NOT NULL,
  nomartiste VARCHAR(1000),
  webartiste VARCHAR(1000),
  reputationartiste INT,
  nationaliteartiste VARCHAR(1000),
  PRIMARY KEY (idartiste)
);

CREATE TABLE COMMISSAIRE_PRISEUR (
  idcommissaire_priseur INT NOT NULL,
  nomcommissaire_priseur VARCHAR(1000) NOT NULL,
  nationalitecommissaire_priseur VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idcommissaire_priseur)
);

CREATE TABLE MARCHE (
  idmarche INT NOT NULL,
  nommarche VARCHAR(1000) NOT NULL,
  datemarche DATE NOT NULL,
  prixmarche INT,
  adressemarche VARCHAR(1000) NOT NULL,
  idcommissaire_priseur INT NOT NULL,
  PRIMARY KEY (idmarche),
  FOREIGN KEY (idcommissaire_priseur)
  REFERENCES COMMISSAIRE_PRISEUR(idcommissaire_priseur)
);


CREATE TABLE RESTAURATEUR (
  idrestaurateur INT NOT NULL,
  nomrestaurateur VARCHAR(1000) NOT NULL,
  typerestaurateur VARCHAR(100) NOT NULL,
  nationaliterestaurateur VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idrestaurateur)
);

CREATE TABLE MUSEE (
  idmusee INT NOT NULL,
  nommusee VARCHAR(1000) NOT NULL,
  datemusee DATE NOT NULL,
  prixmusee INT,
  adressemusee VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idmusee)
);

CREATE TABLE GALERIE (
  idgalerie INT NOT NULL,
  nomgalerie VARCHAR(1000) NOT NULL,
  dategalerie DATE NOT NULL,
  prixgalerie INT,
  association VARCHAR(1000),
  adressegalerie VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idgalerie)
);

CREATE TABLE CRITIQUE (
  idcritique INT NOT NULL,
  nomcritique VARCHAR(1000) NOT NULL,
  reputationcritique INT,
  nationalitecritique VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idcritique)
);

CREATE TABLE EXPERT (
  idexpert INT NOT NULL,
  nomexpert VARCHAR(1000) NOT NULL,
  typeexpert VARCHAR(1000) NOT NULL,
  nationaliteexpert VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idexpert)
);

CREATE TABLE MECENE (
  idmecene INT NOT NULL,
  nommecene VARCHAR(1000) NOT NULL,
  reputationmecene INT,
  capitalmecene INT,
  nationalitemecene VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idmecene)
);

CREATE TABLE CREANCIER (
  idcreancier INT NOT NULL,
  nomcreancier VARCHAR(1000) NOT NULL,
  capitalcreancier INT,
  nationalitecreancier VARCHAR(1000) NOT NULL,
  PRIMARY KEY (idcreancier)
);




CREATE TABLE POSSEDE (
  idcreancier INT NOT NULL,
  idart INT NOT NULL,
  prixAchat INT,
  prixVente INT,
  datedebutPossede DATE NOT NULL,
  datefinPossede DATE,
  PRIMARY KEY (idcreancier, idart),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idcreancier)
  REFERENCES CREANCIER(idcreancier)
);

CREATE TABLE CREE (
  idartiste INT NOT NULL,
  idart INT NOT NULL,
  PRIMARY KEY (idartiste, idart),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idartiste)
  REFERENCES ARTISTE(idartiste)
);

CREATE TABLE PARTICIPE (
  idcreancier INT NOT NULL,
  idmarche INT NOT NULL,
  PRIMARY KEY (idcreancier, idmarche),
  FOREIGN KEY (idcreancier)
  REFERENCES CREANCIER(idcreancier),
  FOREIGN KEY (idcreancier)
  REFERENCES MARCHE(idmarche)
);

CREATE TABLE EXPOSE (
  idart INT NOT NULL,
  idgalerie INT NOT NULL,
  datedebutexpose DATE NOT NULL,
  datefinexpose DATE NOT NULL,
  PRIMARY KEY (idart, idgalerie),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idgalerie)
  REFERENCES GALERIE(idgalerie)
);

CREATE TABLE RESTAURE (
  idrestaurateur INT NOT NULL,
  idart INT NOT NULL,
  prixrestaure INT,
  PRIMARY KEY (idrestaurateur, idart),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idrestaurateur)
  REFERENCES RESTAURATEUR(idrestaurateur)
);
CREATE TABLE PRET (
  idart INT NOT NULL,
  idmusee INT NOT NULL,
  datedebutpret DATE NOT NULL,
  datefinpret DATE NOT NULL,
  PRIMARY KEY (idart, idmusee),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idmusee)
  REFERENCES MUSEE(idmusee)
);

CREATE TABLE JUGE (
  idcritique INT NOT NULL,
  idart INT NOT NULL,
  prixJuge INT NOT NULL,
  PRIMARY KEY (idcritique, idart),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idcritique)
  REFERENCES CRITIQUE(idcritique)
);

CREATE TABLE EXPERTISE (
  idexpert INT NOT NULL,
  idart INT NOT NULL,
  PRIMARY KEY (idexpert, idart),
  FOREIGN KEY (idart)
  REFERENCES ART(idart),
  FOREIGN KEY (idexpert)
  REFERENCES EXPERT(idexpert)
);

CREATE TABLE AIDE (
  idmecene INT NOT NULL,
  idartiste INT NOT NULL,
  prixAide INT,
  PRIMARY KEY (idmecene, idartiste),
  FOREIGN KEY (idartiste)
  REFERENCES ARTISTE(idartiste),
  FOREIGN KEY (idmecene)
  REFERENCES MECENE(idmecene)
);
