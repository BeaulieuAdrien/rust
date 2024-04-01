# RUST VS PHP

Vous trouverez dans ce repository trois projets. Un en php et les deux autres en Rust.

L'objectif était de comparer le temps d'execution entre les deux langages, à savoir que Rust peut s'executer en multi-thread.

## Lancement des apps

#### METTRE le fichier de log dans /hello/ sous le nom ssl_access_log
#### hello/ssl_access_log
Php :
```
cd loganalysis
php bin/console app:analyze-logs ../hello/ssl_access_log
cd ..
```

Rust :
````
cd hello
cargo run
cd ..
````

Rust multi-thread :
````
cd multi_thread
cargo run
cd ..
````

## TLDR

1. PHP (19 secondes) 
2. Rust Multi-thread (110 secondes) 
3. Rust (180 secondes)

Rust semble être un langage intéressant mais de mon point de vue la gestion des multi-thread est trop compliqué à gérer.

Etant un développeur PHP, je ne vois pas l'intérêt d'investir du temps dans l'apprentissage de Rust.
Le temps d'execution de PHP est plus qu'acceptable au vu de la taille du fichier.

J'ai bien observé une amélioration du temps d'execution avec les multithread rust, mais à mon niveau de connaissance et tout en gardant des données cohérente, je n'ai pas réussi à faire mieux que PHP. Cela semble cependant possible, mais là revient la question de l'intérêt d'investir dans ce langage alors que PHP se débrouille très bien de base et est beaucoup plus simple à apprivoiser.

## PHP :

### Résultats:

- Nombre total de requêtes: 35094647
- Statut HTTP 503 : 17
- Statut HTTP 302 : 1180893
- Statut HTTP 500 : 83306
- Statut HTTP 200 : 33124246
- Statut HTTP 301 : 2230
- Statut HTTP 404 : 494340
- Statut HTTP 304 : 77295
- Statut HTTP 400 : 2575
- Statut HTTP 403 : 95140
- Statut HTTP 499 : 18802
- Statut HTTP 206 : 2657
- Statut HTTP 405 : 1076
- Statut HTTP 401 : 1381
- Statut HTTP 504 : 131
- Statut HTTP 502 : 10549
- Statut HTTP 204 : 9
- Nombre total d'adresses IP uniques : 13701
- Adresse IP la plus fréquente : 92.122.54.14 (7297725 requêtes, 20.79% du total)

#### Temps d'exécution du script: 18.948670148849 secondes.

## RUST (Non multi-thread): 

### Résultats

- TOTAL HTTP REQUEST : 35094647
- HTTP Status 200: 33124246
- HTTP Status 204: 9
- HTTP Status 206: 2657
- HTTP Status 301: 2230
- HTTP Status 302: 1180893
- HTTP Status 304: 77295
- HTTP Status 400: 2575
- HTTP Status 401: 1381
- HTTP Status 403: 95140
- HTTP Status 404: 494340
- HTTP Status 405: 1076
- HTTP Status 499: 18802
- HTTP Status 500: 83306
- HTTP Status 502: 10549
- HTTP Status 503: 17
- HTTP Status 504: 131
- Unique IP : 13701
- Most recurrent IP 92.122.54.14: 7297725
#### Execution time: 180.484326125s

## RUST (MULTI-THREAD)

### Résultats

- TOTAL HTTP REQUEST : 35094647
- HTTP Status 200: 33124246
- HTTP Status 204: 9
- HTTP Status 206: 2657
- HTTP Status 301: 2230
- HTTP Status 302: 1180893
- HTTP Status 304: 77295
- HTTP Status 400: 2575
- HTTP Status 401: 1381
- HTTP Status 403: 95140
- HTTP Status 404: 494340
- HTTP Status 405: 1076
- HTTP Status 499: 18802
- HTTP Status 500: 83306
- HTTP Status 502: 10549
- HTTP Status 503: 17
- HTTP Status 504: 131
- Unique IP : 13701
- Most recurrent IP 92.122.54.14: 7297725
#### Execution time: 111.225246083s

