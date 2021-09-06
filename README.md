# Funky Engine

A FNF-inspired DDR-like made with Rust and the Bevy game engine.

This project is NOT an FNF mod, it is a completely separate codebase in a different language.

## `make-funky`

`make-funky` is our game specification preprocessor. It parses out charts\* and sprite atlases from MIDI and SVG files.

\* We **do not** automatically generate charts. Charts must be specially written into the MIDI by a human, usually through the use of a DAW. LMMS and FL Studio are known to work fairly well.

## Making a game

I haven't thoroughly documented the modding process, but you can use the `game` directory in this repo or the same directory in the *Vs. The Gang* (doesn't exist yet) as a reference.

For our metadata files, we use the Rusty Object Notation. Support in languages other than Rust is scarce, but it's very human-friendly. All of the properties are demonstrated in the game directory in this repo, and are defined in structs under `src/meta`.

In short, fill `game` with `meta.game`, `song.wav`, and the `weeks` directory.

Fill the weeks directory with numbered week directories.

Fill the week directories with `meta.week`, `song.wav`, and the `songs` directory.

Fill the song directories with `meta.song`, `song.wav`, `song.mid`, and optionally `dialogue_pre.txt`, `dialogue_post.txt`, `pre.wav`, and `post.wav`.

TODO have a characters dir