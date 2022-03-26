CREATE DATABASE IF NOT EXISTS `MERISE` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;
USE `MERISE`;

CREATE TABLE `P1_COMMISSAIRES-PRISEURS` (
  `idcommissaire` VARCHAR(42),
  `nomcommissaire` VARCHAR(42),
  `nationalitecommissaire` VARCHAR(42),
  PRIMARY KEY (`idcommissaire`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_RESTAURATEUR` (
  `idrestaurateur` VARCHAR(42),
  `nomrestaurateur` VARCHAR(42),
  `typerestaurateur` VARCHAR(42),
  `nationaliterestaurateur` VARCHAR(42),
  PRIMARY KEY (`idrestaurateur`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_MUSEE` (
  `idmusee` VARCHAR(42),
  `nommusee` VARCHAR(42),
  `datedecreation` VARCHAR(42),
  `adressemusee` VARCHAR(42),
  PRIMARY KEY (`idmusee`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `EXPOSE` (
  `idart` VARCHAR(42),
  `idgalerie` VARCHAR(42),
  `dureeexpo` VARCHAR(42),
  PRIMARY KEY (`idart`, `idgalerie`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_GALERIE` (
  `idgalerie` VARCHAR(42),
  `nomgalerie` VARCHAR(42),
  `dategalerie` VARCHAR(42),
  `prixentryexpo` VARCHAR(42),
  `association` VARCHAR(42),
  `adressegalerie` VARCHAR(42),
  PRIMARY KEY (`idgalerie`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `RESTAURE` (
  `idrestaurateur` VARCHAR(42),
  `idart` VARCHAR(42),
  `prixrestauration` VARCHAR(42),
  PRIMARY KEY (`idrestaurateur`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `PRET` (
  `idart` VARCHAR(42),
  `idmusee` VARCHAR(42),
  `dureepret` VARCHAR(42),
  PRIMARY KEY (`idart`, `idmusee`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `JUGE` (
  `idcritique` VARCHAR(42),
  `idart` VARCHAR(42),
  `cotedonnee` VARCHAR(42),
  PRIMARY KEY (`idcritique`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_CRITIQUE` (
  `idcritique` VARCHAR(42),
  `nomcritique` VARCHAR(42),
  `reputationcritique` VARCHAR(42),
  `nationalitecritique` VARCHAR(42),
  PRIMARY KEY (`idcritique`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_MARCHE` (
  `idmarche` VARCHAR(42),
  `nommarche` VARCHAR(42),
  `datemarche` VARCHAR(42),
  `prixentrymarche` VARCHAR(42),
  `adressemarche` VARCHAR(42),
  `idcommissaire` VARCHAR(42),
  PRIMARY KEY (`idmarche`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `VEND` (
  `idcreancier` VARCHAR(42),
  `idart` VARCHAR(42),
  `prixvente` VARCHAR(42),
  `datedevente` VARCHAR(42),
  PRIMARY KEY (`idcreancier`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_ART` (
  `idart` VARCHAR(42),
  `titre` VARCHAR(42),
  `type` VARCHAR(42),
  `cote` VARCHAR(42),
  `date` VARCHAR(42),
  PRIMARY KEY (`idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `EXPERTISE` (
  `idexpert` VARCHAR(42),
  `idart` VARCHAR(42),
  PRIMARY KEY (`idexpert`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_EXPERT` (
  `idexpert` VARCHAR(42),
  `nomexpert` VARCHAR(42),
  `typeexpert` VARCHAR(42),
  `nationaliteexpert` VARCHAR(42),
  PRIMARY KEY (`idexpert`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_MECENE` (
  `idmecene` VARCHAR(42),
  `nommecene` VARCHAR(42),
  `reputationmecene` VARCHAR(42),
  `capitalmecene` VARCHAR(42),
  `nationalitemecene` VARCHAR(42),
  PRIMARY KEY (`idmecene`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `PARTICIPE` (
  `idcreancier` VARCHAR(42),
  `idmarche` VARCHAR(42),
  `oeuvreachetee` VARCHAR(42),
  `oeuvrevendue` VARCHAR(42),
  PRIMARY KEY (`idcreancier`, `idmarche`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_CREANCIER` (
  `idcreancier` VARCHAR(42),
  `nomcreancier` VARCHAR(42),
  `capitalcreancier` VARCHAR(42),
  `nationalitecreancier` VARCHAR(42),
  PRIMARY KEY (`idcreancier`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `POSSEDE` (
  `idcreancier` VARCHAR(42),
  `idart` VARCHAR(42),
  `datedebut` VARCHAR(42),
  `datefin` VARCHAR(42),
  PRIMARY KEY (`idcreancier`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `CREE` (
  `idartiste` VARCHAR(42),
  `idart` VARCHAR(42),
  PRIMARY KEY (`idartiste`, `idart`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `P1_ARTISTE` (
  `idartiste` VARCHAR(42),
  `nomartiste` VARCHAR(42),
  `webartiste` VARCHAR(42),
  `reputationartiste` VARCHAR(42),
  `nationaliteartiste` VARCHAR(42),
  PRIMARY KEY (`idartiste`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `AIDE` (
  `idmecene` VARCHAR(42),
  `idartiste` VARCHAR(42),
  PRIMARY KEY (`idmecene`, `idartiste`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

ALTER TABLE `EXPOSE` ADD FOREIGN KEY (`idgalerie`) REFERENCES `P1_GALERIE` (`idgalerie`);
ALTER TABLE `EXPOSE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `RESTAURE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `RESTAURE` ADD FOREIGN KEY (`idrestaurateur`) REFERENCES `P1_RESTAURATEUR` (`idrestaurateur`);
ALTER TABLE `PRET` ADD FOREIGN KEY (`idmusee`) REFERENCES `P1_MUSEE` (`idmusee`);
ALTER TABLE `PRET` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `JUGE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `JUGE` ADD FOREIGN KEY (`idcritique`) REFERENCES `P1_CRITIQUE` (`idcritique`);
ALTER TABLE `P1_MARCHE` ADD FOREIGN KEY (`idcommissaire`) REFERENCES `P1_COMMISSAIRES-PRISEURS` (`idcommissaire`);
ALTER TABLE `EXPERTISE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `EXPERTISE` ADD FOREIGN KEY (`idexpert`) REFERENCES `P1_EXPERT` (`idexpert`);
ALTER TABLE `PARTICIPE` ADD FOREIGN KEY (`idmarche`) REFERENCES `P1_MARCHE` (`idmarche`);
ALTER TABLE `PARTICIPE` ADD FOREIGN KEY (`idcreancier`) REFERENCES `P1_CREANCIER` (`idcreancier`);
ALTER TABLE `POSSEDE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `POSSEDE` ADD FOREIGN KEY (`idcreancier`) REFERENCES `P1_CREANCIER` (`idcreancier`);
ALTER TABLE `CREE` ADD FOREIGN KEY (`idart`) REFERENCES `P1_ART` (`idart`);
ALTER TABLE `CREE` ADD FOREIGN KEY (`idartiste`) REFERENCES `P1_ARTISTE` (`idartiste`);
ALTER TABLE `AIDE` ADD FOREIGN KEY (`idartiste`) REFERENCES `P1_ARTISTE` (`idartiste`);
ALTER TABLE `AIDE` ADD FOREIGN KEY (`idmecene`) REFERENCES `P1_MECENE` (`idmecene`);