# Rust Software Rasterizer 
A naive implementation of a software rasterizer in Rust. It uses Bresenham's Line Drawing algorithm and simple linear interpolations to rasterize the triangle with the given three vertices of the with their respective RGB color values. Features like Anti-aliasing, depth testing or overlapping optimisations have not been implemented.

One Slider is used for setting resolution for visulisation and the other one is for rotation about y-axis.
<img width="792" alt="Screenshot 2024-10-18 at 4 04 12 AM" src="https://github.com/user-attachments/assets/6a8edff1-7803-4471-96a5-0f3902f72598">


Installation Guide \br
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
