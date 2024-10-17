For Mac - 
Install SDL2 ( I am using version 2.30.8). I prefer using Homebrew : 
```
brew install SDL2
```
Set the path : 
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```
Run the project from the main directory. (Assuming Rust is already installed) : 
```
cargo run
```
For Windows, I have kept the SDL.dll within the project, so the compiler should be able to find the dynamic library without requiring its corresponding import library, but don't know. You can try compiling :
```
cargo run
```
