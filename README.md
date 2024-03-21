# ECE 421 Project 2 
# Trees, Trees and More Trees

This is a "company task" (in class project) on building Rust data structures to provide faster, more efficient data structure solutions.

Within the project contains an implementation of [Red Black Tree](https://en.wikipedia.org/wiki/Red-black_tree) and [AVL tree](https://en.wikipedia.org/wiki/AVL_tree).

## Authors
Kevin H, Castor S, Nhung N


## Requirements (Crates)
- `rustc` and `cargo`
- `git` to clone this repo

**cargo packages** 
- `criterion` = { version = "0.4", features = ["html_reports"] } - Used to do benchmarking.
  

## Project Setup
To set up this project, start by cloning the code from [here](https://github.com/Kevin-was-here/ece421_project2) (or open/extract the zip/tar ball)

Check in the cloned project folder inside the `cargo.toml` file there should be the same list of dependencies as listed in the [requirements](#requirements-crates) section 

Once the project is cloned head into the project directory in the terminal and run program with the command:
```
> cargo run
```
This may take a second on the first attempt as cargo is initalizing.



## Useage Instructions

1. Follow the instructions in [project setup](#project-setup) and obtain the executable `trees.exe` in the `./target` file
2. execute the program by running 
```
> .\trees.exe
```