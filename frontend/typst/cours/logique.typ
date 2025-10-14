#import "@preview/theorion:0.4.0": *
#import cosmos.fancy: *
#set text(lang: "fr")
#show: show-theorion
#show math.equation: it => {
  if target() == "html" {
    box(html.frame(it))
  } else {
    it
  }
}
= Éléments de logique

== Définition : proposition (assertion)

#definition(title: "Proposition (assertion)")[
On appelle proposition (ou assertion) un énoncé qui est soit vrai soit faux et qui décrit des propriétés, des objets mathématiques. 
]

#example()[
  - $0 = 1 "faux"$ 
  - $3 < 4 "vrai"$ 
]

Les propositions sont désignées par des majuscules P, Q, R

#definition(title: "Table de vérité")[On appelle table de vérité d'une proposition un tableau qui donne la valeur de vérité (V ou F) de cette proposition]

Deux propositions ayant la même valeur de vérité sont logiquement équivalentes. 

== Connecteurs logique 
Les connecteurs logiques ($not, and, or, #sym.arrow.double, #sym.arrow.l.r.double$) servent à construire de nouvelles propositions. 

Soient P et Q deux propositions. 

=== Négation 

=== Conjonction 

=== Disjonction 
