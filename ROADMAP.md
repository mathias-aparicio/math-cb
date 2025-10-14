# Roadmap math-cb

## Produit minimal viable

- [x] Compilation des sources Typst en HTML
- [x] Pages sommaires pour les cours et exercices
- [x] Barre de navigation (Cours, Exercices, Méthodes, À propos, Github)
- [ ] Déployer
- [ ] Github Action pour recompiler les sources Typst en html à chaque PR

## Contenu et Style

- [ ] Blocs numérotés et référençables (théorème, définition, lemme, etc.)
  - [ ] Références inter-fichiers (ex: théorème de trigo dans un autre chapitre)
- [ ] Structure pour les démonstrations (idée principale, étapes)
- [ ] Création d'un template de base
- [ ] Ajout de points historiques en début de chapitre

## Annuaire des Méthodes

- [ ] Créer un annuaire de méthodes structuré en arbre
  - `Exemple : Diagonaliser une matrice -> Trouver valeurs propres -> ...`
- [ ] Organiser les méthodes par chapitre
- [ ] Identifier chaque méthode (numéro, indice, chapitre)
- [ ] Une méthode est liée à plusieurs exercices
- [ ] Un exercice est lié à plusieurs méthodes, et théorèmes
  - [ ] Permettre d'activer/désactiver l'affichage de ces associations
- [ ] Dans les corrections, lier chaque étape de la solution à la méthode correspondante
- [ ] Inclure hyperlien vers les méthodes dans l'annuaire

## Apprentissage et Suivi

- [ ] Quiz de fin de chapitre (théorèmes, définitions, idées de démonstration)
- [ ] Tableau de bord de maîtrise des méthodes
  - [ ] Jauge de maîtrise par méthode (augmente/diminue avec les résultats aux exercices)
  - [ ] Système de répétition espacée (type Anki)
    - [ ] La maîtrise décroît avec le temps
    - [ ] Suggestion d'exercices pour renforcer les points faibles

## Annales

- [ ] Section Annales avec des corrections utilisant uniquement les méthodes
- [ ] Afficher un "degré de classicisme" pour chaque méthode (basé sur sa fréquence dans les annales)
- [ ] Afficher pour chaque annales les méthodes requises et le niveau de maîtrise de l'utilisateur sur chacune

## Exports et Intégrations

- [ ] Bouton d'export pour les cours et exercices
  - [ ] Sources Typst
  - [ ] PDF
  - [ ] EPUB
- [ ] Intégration Anki
  - [ ] Export de cours en paquets Anki (théorèmes, définitions, etc.)
  - [ ] Synchronisation avec l'API Anki pour refléter la maîtrise sur le tableau de bord

## Backend

- [ ] Mise en cache des pages pour améliorer les performances
  `Diminue les accès disque sur le server`
