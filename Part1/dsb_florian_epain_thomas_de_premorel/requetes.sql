-- Projection :

	-- Les noms des oeuvres d'art des Artistes avec un grand A

SELECT titre
FROM P1_ART Join P1_ARTISTE on nomArtiste
WHERE reputaionArtiste > 1200
;

	-- Les oeuvres d'art dont on ignore le createur
 
SELECT titre
FROM P1_ART
WHERE nomArtiste IS NULL
;

-- Jointure : Les Mecenes qui achetent aussi de l'art

SELECT *
FROM P1_MECENE Natural Join P1_CREANCIER on nomCreancier=nomMecene
;

-- Moyenne sur l'integralite d'un attribut : La moyenne du capital des créanciers

SELECT AVG(capitalCreancier) as capitalCreancier.MOY
FROM P1_CREANCIER
;

-- Un regroupement par calcul : 

	-- Les prix d'entree au Marche d'art les plus bas, actuel (qui ne sont pas expirée)
	
SELECT MIN(prixEntryMarche)
FROM P1_MARCHE
WHERE dateMarche > -- dateActuelle
;
			  
	-- Les Marches d'art dont leur billet d'entree sont en dessous de la moyenne
	
SELECT nomMarche
FROM P1_MARCHE
WHERE dateMarche > -- dateActuelle
      prixEntryMarche < (SELECT AVG(prixEntryMarche)
			  FROM P1_MARCHE
			  WHERE dateMarche > -- dateActuelle)
;

-- Une Différence : Les artistes qui n'ont rien créé

SELECT nomArtiste -- DISTINCT non nécéssaire
FROM P1_ARTISTE
WHERE nomArtiste NOT IN (SELECT DISTINCT nomArtiste
                         FROM P1_ART)
;

-- Une Division : Noms des Creanciers ayant possedé toutes les oeuvres d'art
	-- peut faire la même chose avec les Musees (Création d'une nouvelle table
	-- pour se rappeler par quels musées une oeuvre est passée : relation pret works)

SELECT nomCreancier
FROM P1_CREANCIER
WHERE (SELECT COUNT(DISTINCT *)
       FROM P1_POSSEDE
       WHERE P1_POSSEDE.idART = P1_ART.idArt -- P1_POSSEDE.#idArt
       AND   P1_POSSEDE.idCreancier = P1_CREANCIER.idCreancier)
      = (SELECT COUNT(DISTINCT *) from P1_ART)
;

-- Un Group By :

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