This directory contains an example game spec.

In the future, the binary target `make-funky` will allow converting this directory into the format used by the game.

Charting for this software will be unfamiliar to those used to FNF. This game's charts are meant to be written in your DAW, rather than in-game. To write charts...
* Create a silent track named `CUE $NAME`,where `$NAME` is replaced with the name of the cue track you want to write.
  * Cue track names are configured in `meta.song`.
* Place a note at the very start, ensuring that it is the lowest note in the track. This is the "root note"
* The key used by a note will be determined by its distance from the root note. The leftmost notes should be one key above the root note, and the rightmost notes should be 4 keys above the root note.
* Repeat for all players and enemies.