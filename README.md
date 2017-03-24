# minesweeper.rs

[![Build Status](https://travis-ci.org/babymotte/minesweeper.rs.svg?branch=develop)](https://travis-ci.org/babymotte/minesweeper.rs)

A library providing the core functionality of the popular minesweeper game.
This is a project I started to learn Rust and most probably be not very
useful for anyone else.

## Release notes:

### 1.0.0

Provides the functionality to start games at the common difficulty settings Beginner (9x9, 10 mines), Intermediate(16x16, 40 mines), Expert (30x16, 99 mines) and Custom, play the game (uncover tiles, set flags to mark mines, win when all mine-free tiles are uncovered, lose when you hit a mine), measure the duration of the game and persist highscores.

Also included is an executable command line interface that provides all functionality necessary to play a game.
