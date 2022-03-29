-- Projection :

	-- Les noms des oeuvres d'art des Artistes avec un grand A (reputation > 1200) avec les artistes en questions.

SELECT P1_ARTISTE.nomartiste, P1_ART.titre
FROM (P1_CREE JOIN P1_ARTISTE on P1_ARTISTE.idartiste = P1_CREE.idartiste )
	      JOIN P1_ART on P1_ART.idart = P1_CREE.idart
WHERE reputationArtiste > 1200
;

-- Jointure :

	-- Les mecènes et les collectionneurs d'art (créanciers) qui partagent le même nom et prénom.

SELECT *
FROM P1_MECENE Join P1_CREANCIER on P1_CREANCIER.nomCreancier=P1_MECENE.nomMecene
;

-- Moyenne sur l'integralite d'un attribut : 

	-- La moyenne du capital supposé des créanciers.

SELECT AVG(capitalCreancier) as moy_capitalcreancier
FROM P1_CREANCIER
;

-- Un regroupement par calcul : 

	-- Les prix d'entree au Marche d'art les plus bas, actuel (qui ne sont pas expirée)
	
SELECT MIN(prixMarche)
FROM P1_MARCHE
WHERE dateMarche > CURDATE()
;
			  
	-- Les Marches d'art dont leur billet d'entree sont en dessous de la moyenne
	
SELECT nomMarche
FROM P1_MARCHE
WHERE dateMarche > CURDATE() AND
      prixMarche < (SELECT AVG(prixMarche)
					FROM P1_MARCHE)
;

-- Une Différence :

	-- Les artistes qui n'ont rien créé

SELECT nomArtiste -- DISTINCT non nécéssaire
FROM P1_ARTISTE
WHERE idartiste NOT IN (SELECT DISTINCT idartiste
                         FROM P1_CREE)
;

-- Une Division : 

	-- Nom et ID des Creanciers ayant possedé toutes les oeuvres d'art.

SELECT idcreancier, nomCreancier
FROM P1_CREANCIER
WHERE (SELECT DISTINCT COUNT(*)
       FROM P1_POSSEDE
       WHERE P1_POSSEDE.idcreancier = P1_CREANCIER.idcreancier)
      =
	  (SELECT DISTINCT COUNT(*) from P1_ART)
;

	-- Nom et ID des Musées ayant disposé de toutes les oeuvres d'art.

SELECT idmusee, nommusee
FROM P1_MUSEE
WHERE (SELECT DISTINCT COUNT(*)
       FROM P1_PRET
       WHERE P1_PRET.idmusee = P1_MUSEE.idmusee)
      = (SELECT DISTINCT COUNT(*) from P1_ART)
;


-- Un Group By :

	-- Nombre d'artiste par nationalité.

SELECT nationaliteArtiste, COUNT(*)
FROM P1_ARTISTE
GROUP BY nationaliteArtiste
ORDER BY COUNT(*) DESC 
;