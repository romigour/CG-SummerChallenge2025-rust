# MergeFiles

Merge les class dans un fichier :
`py .\mergefiles.py`

# Referee (pour éviter timeout java)

- Récupérer le GameManager.java : https://github.com/CodinGame/codingame-game-engine/blob/master/engine/core/src/main/java/com/codingame/gameengine/core/GameManager.java
- L'ajout dans le referee dans le package com.codingame.gameengine.core
- Modifier les deux constantes GAME_DURATION_HARD_QUOTA et GAME_DURATION_SOFT_QUOTA avec : 600 * 100000.
- Then to override the referee timeouts, modify the methods setTurnMaxTime : this.turnMaxTime = 100000; and setFirstTurnMaxTime :this.firstTurnMaxTime = 100000;
- Rebuild

# Lancement avec la class Main
java -cp referee.jar com.codingame.Main stat "C:\GIT\CG-SummerChallenge2025-rust\bots\PlayerOld.exe" "C:\GIT\CG-SummerChallenge2025-rust\bots\Player.exe"


# cg-selfarena https://github.com/Telokis/cg-selfarena


# CG-Arena

Dans une console gitbash `cgarena init arena`


copier les fichiers nécessaire depuis le dossier cgarena
- build.sh
- revoir le cgarena_config.toml,
- revoir la league si besoin dans le play_game.py,
- copier le referee.jar du jeu


`cgarena run arena`

# CG-Local
Autopush dans l'IDE codingame :
CG-Local : https://www.codingame.com/forum/t/cg-local/10359
Lancer le jar : java -jar cg-local-app-1.3.0.jar
Et avoir le plugin firefox ou chrome d'installer

# CG-BrutalTester
CG-BrutalTester:
Dossier au dessus qui contient le .jar

cg-local / cg-exhancer / Violentmonkey

# Psyleague

# cgsubmit
Outil qui permet de voir les matchs timeout ou perdu dans son historique
https://github.com/FrequentlyMissedDeadlines/cgsubmit

Exemple: python -m cgsubmit -t 706344417256bc809fa636f89e195d6f4d28b989




Chatgpt


Je travail sur un bot pour un challenge codingame de combat de bot, je souhaite développer mon bot en Rust et faire un MCTS. 
Le jeu est un combat en 1v1 où chaque joueur gère entre 3 et 5 agents en simultané.
La déroulement est comme tout les autres challenges de combat de bot codingame, c'est du tour par tour, chaque joueur envoie ses actions et le referee gère le combat.
Je souhaite que tu me proposes une structure de code pour ce projet, en incluant des tests pour simuler un tour de jeu à partir d'un fichier `input.txt` qui contiendrait les données d'un tour de jeu. 
L'objectif est de pouvoir exécuter le code localement pour comprendre le fonctionnement du jeu.
