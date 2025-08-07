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
java -cp referee.jar com.codingame.Main stat "java C:\GIT\CG-SummerChallenge2025-bit\bots\PlayerOld.java" "java C:\GIT\CG-SummerChallenge2025-bit\bots\Player.java"


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

# cg-analyze-match
Outil qui permet de voir les matchs timeout ou perdu dans son historique
à lancer de préférence sur un shell linux qui gère les unicode
Exemple: java "-Dfile.encoding=UTF-8" -jar cg-analyze-match.jar 706344417256bc809fa636f89e195d6f4d28b989 -w -l -s cannot

# cgsubmit
Outil qui permet de voir les matchs timeout ou perdu dans son historique
https://github.com/FrequentlyMissedDeadlines/cgsubmit

Exemple: python -m cgsubmit -t 706344417256bc809fa636f89e195d6f4d28b989

# message intéressant entre MC et HC
MC is monte Carlo (full random) sims and take the best

HC is Hill climbing. Instead of just doing random sims, you always keep the
best in memory somewhere and you try a tiny mutation. If the mutation improves
the scoring you keep the new sim as the best to be mutated in the next iteration.
Else you throw it

SA is simulated annealing, almost like Hill climbing but instead of always having
current==best you can sometimes lower the current score using some formula and use
another state as current. It enables to not remain too often stuck in local optimas like HC may do
(you can add diversity because the formula enables to sometimes go down, it's working mathematically)
