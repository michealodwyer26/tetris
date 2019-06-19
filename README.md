# tetris.rs

A simple tetris clone, written in Rust, using the sdl2 library bindings.

![Gif showing a demo of the game](https://github.com/michealodwyer26/tetris/blob/master/demo.gif) 

## How to Play

#### Windows
For 64-bit computers, just download/clone this repo and run the executable file `tetris.exe`. For 32-bit computers, 
download/clone the repo and delete the dlls provided as these are 64-bit, and download the following development libraries 
from www.libsdl.org: `sdl2`, `sdl2_image` and `sdl2_ttf`. After extracting, copy the dlls found in `lib/x86` of each library 
and paste them to the root directory of this repo.

If you wish to compile the code from source, first install [Rust](https://www.rust-lang.org/tools/install), and install the 
three development libraries listed above. Instructions on installing these libraries can be found at 
[rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2).

#### Linux/Mac
The easiest way to play the game is to download/clone this repo and compile the source code.
First, install [Rust](https://www.rust-lang.org/tools/install), and then install 
`sdl2`, `sdl2_image` and `sdl2_ttf` using your package manager. Once these libraries have been installed, you can run the game 
using `cargo run --release`. More information on installing the libraries can be found at 
[rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2).

## Controls

**Left/Right/Down** - Moves the tetrimino 

**Up** - Rotates the tetrimino

**Spacebar** - Drops the tetrimino as far down as possible

**P** - Pauses the game

**Escape** - Exits the game

## Assets

The free assets used, created by [Buch](http://blog-buch.rhcloud.com), can be found [here](https://opengameart.org/content/arcade-pack).
Also, the Inconsolata font was used for the score counter.

Thanks for reading!
