# Pong

This repository contains a Pong game, built with simplicity in mind, and inspired by the visual aesthetics of the Tron universe. The game is developed using the Rust programming language.


![pong_mockup](https://github.com/AaronWatson2975/pong/assets/36612616/9f2bd923-4da9-4eba-b93f-abd3d6a2ccca)


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

Enjoy playing your very own Tron-themed Pong game!
