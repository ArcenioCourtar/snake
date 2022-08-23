# Snake
My first Rust project!
---
It's a snake game running in the terminal. Move by typing one of the "wasd" keys and pressing enter. I used Robert Heaton's https://robertheaton.com/2018/12/02/programming-project-5-snake/ for inspiration and guidance.
---

TODO, roughly by order of importance:
* functional
- [x] Fix Snake collision detection. I broke it somewhere along the way 
- [x] Make sure tail segments update properly upon apple eating
- [] Make sure the apple can't appear in a snake-occupied location
- [] Count score and display it 
- [] Add option to change board size (probably by using CLA?)
- [] Add option to change board edge behaviour (currently wraps by default, (CLA?))
- [] Register moves with single button presses using `curses` library (or make it move automatically? Apparently really difficult using the terminal)

* pure refactoring/cleaning up
- [] Don't clear the board and recalculate every tick. Just update the necessary spaces (currently I set all spaces to empty and then insert the apple and snake accordingly)
- [] For the board, switch from 2D array with a set size, to a 1d vec with an indexer

Changelog:
- 22/08/2022 
  - First working version
- 23/08/2022 
  - FIXED: collision detection & tail segment updating