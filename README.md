# Poople Solver
Solves Poople using A* search, returns you a list of words seperated by newlines. Example of a program being run:
```
poopl-solver cyan
[0] CYAN
[1] CLAN
[2] CLAP
[3] PLAP
[4] PLOP
[5] POOP
```
You can change the word poop for an arbitrary 4 letter word in the poople dictionary:
```
poople-solver cyan crap
[0] CYAN
[1] CLAN
[2] CLAP
[3] CRAP
```
The syntax of the program is:
```
poople-solver [+poople-word] [target]
```