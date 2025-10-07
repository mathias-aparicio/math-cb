# TODO math-cb

├── backend
└── Frontend
    ├── typst
    │   ├── cours
    │   │   ├── ensemble.typ
    │   │   └── trigonometrie.typ
    │   └── exercices
    │       ├── ensemble.typ
    │       └── trigonometrie.typ
    └── html
        ├── cours
        │   ├── ensemble.html
        │   └── trigonometrie.html
        └── exercices
            ├── ensemble.html
            └── trigonometrie.html

- [x] Script qui compile les sources typst en html
- [ ] Page cours et exercice avec un sommaire depuis les fichiers présents dans les dossiers respectifs
- [ ] Header navbar (Cours, Exercice, Méthodes, À propos, Github)
- [ ] Style d'une page typst
  - [ ] Bloc : théorème, définition, lemme, référence historique (theorion)
    - [ ] Doivent être numéroté
    - [ ] Doivent être facilement référencable entre fichiers différents (théorème de trigonométrie dans ensemble)
  - [ ] Démonstration divisée en idée principale
  - [ ] Template
- [ ] Méthode de solution apparaissant dans une classe d'exercice
- [ ] Méthode classée par chapitre
- [ ] Méthode a un numéro et un indice et un chapitre associé
- [ ] Annuaire de méthodes sous forme d'arbre
    Diagonaliser une matrice
    ├── Trouver valeurs propres
    ├── Trouver vecteurs propres
    ├── Matrice de passage
- [ ] Associer les exercices aux méthodes
- [ ] Dans les corrections des exercices lier chaque étape de la solution à une méthode
  - [ ] Hyperlien vers la méthode spécifique utilisée dans l'annuaire
- [ ] Quizz à fin de chaque chapitre
  - [ ] énoncé théorème et definition, idées principales des démonstration
- [ ] Tableau de bord ou chaque méthode de l'annuaire est listée avec une jauge de maitrise
  - [ ] Maitrise diminue quand un exercice est raté et augmente le cas contraire
  - [ ] Décroit avec le temps
  - [ ] Système à la Anki répétition esspacé
  - [ ] Exercice à faire aparaissent, les refaire augmente la jauge de maitrise
- [ ] Export d'un cours en paquet anki avec theoreme, proposition, definition, démonstration
- [ ] Point historique au début d'un chapitre sur un mathématicien ou une mathématicienne  

## Inspiration

<https://cpge-paradise.com/MP4MathBook.php>
<https://maths-france.fr>
<http://alain.troesch.free.fr/>
<http://ddmaths.free.fr/index.html>
