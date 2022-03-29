DROP TABLE IF EXISTS P1_PARTICIPE;
DROP TABLE IF EXISTS P1_MARCHE;


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

CREATE TABLE P1_PARTICIPE (
  idcreancier INT NOT NULL,
  idmarche INT NOT NULL,
  PRIMARY KEY (idcreancier, idmarche),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_CREANCIER(idcreancier),
  FOREIGN KEY (idcreancier)
  REFERENCES P1_MARCHE(idmarche)
);
