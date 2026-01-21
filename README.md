# Snake
Il s'agit d'un simple jeu Snake codé en Rust. 
Le framework Dioxus permet d'afficher le jeu sur une interface web.

## Prérequis
Cloner le projet :
```bash
git clone https://github.com/Valdyzer/Snake.git
cd Snake
```
Installer RUST via rusup :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Recharger le shell (uniquement pour MacOS) :
```bash
source $HOME/.cargo/env
```

## Lancer le jeu
```bash
dx serve
```
Puis rentrer l'adresse indiquée dans la barre de recherche de n'importe quel navigateur.

## Commandes
- ⬅️➡️⬆️⬇️ pour orienter le serpent
- ESPACE pour relancer la partie

## Avancement
- [x] Initialisation du projet avec dioxus
- [x] Définir les propriétés de bases (position, direction)
- [x] Définir les composantes décrivant l'état du jeu (snake, fruit, direction, score...)
- [x] Premier visuel de l'interface statique
- [x] Boucle principale du jeu avec une phase définie
- [x] Premier mouvement du serpent
- [x] Changement de direction du serpent
- [x] Collisions et GAME OVER
- [x] Extension du serpent
- [x] Faire apparaître les fruits aléatoirement
- [x] Bloquer les demi-tours directs
  
