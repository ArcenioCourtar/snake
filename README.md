# Snake
My first Rust project!
---
It's a snake game running in the terminal. Move by typing one of the "wasd" keys and pressing enter. I used Robert Heaton's https://robertheaton.com/2018/12/02/programming-project-5-snake/ for inspiration and guidance.
---
Manual:
Move with `wasd` followed by `enter` key. 
Terminate game early with `t` followed by `enter`

use command line arguments before running to change board size and edge behaviour
Formatted like: `[binary] [arg1] [arg2] [arg3]`
`arg1 and arg2` are board width and height respectively (min 5, max 25 in either direction)
`arg3` lets you change edge behaviour from "wrapping/teleporting" to being lethal "walls" by inserting `Wall`
CLA are only read if you submit two or three args after the executable.
For any argument that cannot be parsed it will revert to the default value (10*10 board with wrapping)

TODO, roughly by order of importance:
* functional
- [x] Fix Snake collision detection. I broke it somewhere along the way 
- [x] Make sure tail segments update properly upon apple eating
- [x] Make sure the apple can't appear in a snake-occupied location
- [x] Add a min-max size to player-defined boards
- [x] Count score and display it 
- [x] Add option to change board size (probably by using CLA?)
- [x] Add option to change board edge behaviour (currently wraps by default, (CLA?))
- [x] Give player the option the exit the game early
- [ ] Register moves with single button presses using `curses` library (or make it move automatically? Apparently really difficult using the terminal)
- [ ] Variable apple score based on steps it took for player to reach it
- [ ] 🍎🍎 End the game when a player fills the entire board with Snake. Currently loops endlessly trying to find a location for Apple 🍎🍎
- [ ] 🧱 Let user switch edge behaviour with a single CLA instead of three 🧱

* refactoring
- [x] Don't clear the board and recalculate every tick. Just update the necessary spaces (currently I set all spaces to empty and then insert the apple and snake accordingly)
- [ ] Wrap CLA handling in a function
- [ ] For the board, switch from 2D array with a set size, to a 1d vec with an indexer
- [ ] Move the structs and such into separate files?
- [x] Replace if-else piles with `match` statements where possible?

Changelog:
- 22/08/2022 
  - First working version
- 23/08/2022 part 1:
  - FIXED: collision detection & tail segment updating
- 23/08/2022 part 2 electric boogaloo:
  - FIXED: new apple can no longer appear inside of the Snake
  - ADDED: score counter, command line options for the player
  - CHANGED: changed the way I handle board changes, substituted *most* `else-if` mountains with `match` hill
  - (note: added pancurses to `[dependencies]` for later use)