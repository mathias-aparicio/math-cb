# math-cb

Cours de mathématiques collaboratif se voulant accessible et exhaustif.

## Contribuer

Les sources mathématiques sont écrites en `typst`, un langage de formatage similaire à LaTeX.
Suivre [la documentation officielle](https://github.com/typst/typst#installation) pour installer le programme.

Une fois `typst` installé et [familiarisé avec le langage](https://typst.app/docs/tutorial).

Pour voir le rendu d'un fichier `typst`, sur une page web. Les modifications apparaitront une fois enregistrer sur le fichier source.

```sh
typst watch frontend/typst/cours/complexe.typ frontend/html/cours/complexe.html --format html --features html --open
```

Pour l'instant uniquement [ces fonctionalitées](https://github.com/typst/typst/issues/5512) sont supportées pour l'export html.
