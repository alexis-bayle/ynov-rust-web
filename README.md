# Overview

Lorsqu'on fait une requête GET sur /ping alors cela renvoie une réponse status 200 avec les headers de la requête en JSON. Toutes les autres routes retourneront
un status 404 sans contenu.

## Définition du port

Par défaut, le port 7878 est utilisé pour écouter les requêtes, celui-ci est modifiable via une variable d'environnement: PING_LISTEN_PORT
