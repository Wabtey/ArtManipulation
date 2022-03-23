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


Nous avons omis les conservateurs de musée, propriétaire de gallerie et d'autres acteurs dans l'équation de l'évaluation du prix d'une oeuvre. Ce procédé reste complexe et non exhaustivement représenté par cette base de donnée. Il s'agit surtout d'une ébauche.

L'art est au milieu de tout ce système. 
Ce Marché se compose de plusieurs acteurs, ces acteurs vont influencer sur la valeur que l'oeuvre porte.

Une pièce d'oeuvre d'art possède un numéro d'objet (unique), un titre, un medium (type) et une cote (la valeur estimée de l'oeuvre).
Nous avons décidé de séparer le créateur de la pièce, étant donné que son créateur peut etre inconnu ou en avoir plusieurs.

Nombreuses tables représentent des humains, ces humains possèdent tous ces *attributs fixes* :
    Un ID unique, un nom et une nationalité.
    Certains d'entre peuevnt avoir comme attribut un capital, une spécialité, un site web ou une réputation.

    Les artistes ont un "constituent_id" unique,
puisqu'on crée le profil artiste si l'on sait qu'il existe par la seule création d'une oeuvre, celui ci peut etre complètement inconnu et ne pas avoir de nom, de site web, de réputation, d'orgine (nationalité).
    On associe les artistes avec les artworks par la relation CREE.
Un Artwork peut etre créé par personne ou plusieurs artistes. Et un artiste peut ne rien avoir créé ou avoir une multitude de création.

    Les Mecenes sont peu présent dans le marche de l'art,
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

    Ce Marché devra être dirigé par un commissaire-priseur et peut avoir aucun ou plusieurs créanciers participant.
Il s'agit de galerie privée, où le prix d'entrée vise à resteindre les acheteurs (n'avoir que la même 'clientèle').
Il se déroule à un certain moment, rendez vous à ne pas manquer pour repérer les fluctuations de prix et certaines oeuvres qui peuvent se rentabiliser.

    Les Galeries et les musées sont des organismes présentant des oeuvres (EXPOSE et PRET resp), elles ne peuvent pas ouvrir si ces organismes n'ont pas assez d'oeuvres à exposer.


---
Modele Relationnel

