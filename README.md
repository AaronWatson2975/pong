# Pong

This repository contains a Pong game, built with simplicity in mind, and inspired by the visual aesthetics of the Tron universe. The game is developed using the Rust programming language.

<img width="1590" alt="Screenshot 2023-08-15 at 12 45 54 AM" src="https://github.com/AaronWatson2975/pong/assets/36612616/c563b601-0bdf-40b8-9569-52a16b037607">

# Installation

Before starting the installation procedure, you need to ensure that SDL2 library is installed on your system. SDL2 is responsible for rendering graphics within the game. You can install it using the following command:

```
brew install sdl2
```

Post installation, it's necessary to update your terminal configuration files (such as `.bashprofile` or `.zshrc`) with the following command:

```
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib:/usr/local/lib"
```

# Launching the Game

Execution of the game is straightforward and can be triggered through one simple command:

```
cargo run
```

# Controls

Here is a list of controls you can use while playing the game. These instructions assume that two players are going head-to-head in a fiery game of pong.

## Player 1

Player 1's controls consist of the following:

- `W` key: Use this to move the paddle up on the screen.
- `S` key: Use this to move the paddle down the screen.

## Player 2

Player 2's controls are:

- `Up Arrow` key: This will allow you to move the paddle upwards.
- `Down Arrow` key: This will enable you to move the paddle downwards.

Enjoy playing your very own Tron-themed Pong game!
