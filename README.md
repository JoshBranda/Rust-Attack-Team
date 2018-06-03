<h1 align="center">
  Rust-Attack-Team Presents  <br /> <br />
  :car: :car: :truck: :car: CRABBER :truck: :car: :car: :truck:<br /> <br />
  <h3 align="center">
  A <a href="https://www.rust-lang.org/en-US/">Rust</a> equivalent of Frogger 
  </h3>
  <p align="center">
    <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/github/license/mashape/apistatus.svg"></a>
  </p>
</h1>

## About

Our game, titled "Crabber", is a Rust application based on the arcade game Frogger developed by Konami in 1981.  

The object of the game is to safely guide each frog from the bottom of the screen to one of the empty cubbies at the top of the screen. To do so, the player must navigate around the traffic in the road and use objects floating in the river to cross the river without touching the water. If the player fails to do either of these tasks, the frog dies and a life is lost.  Once all 3 lives are lost, the game is over.  Otherwise, if the player can successfully fill each cubby with a frog, the player wins the game.

<br /> <br />
*No crabs were harmed in the making of this application.*

## Installation
1.  Clone the repository to your local machine.
2.  Using terminal, putty, or a linux shell, enter the cloned folder.
3.  Run "Cargo build" (or install Cargo if you haven't yet)
```
cargo build
```
4.  If using a Mac, you may need to install sdl2:
```
brew install sdl2
```
5.  Run "./run.sh" when running for the first time.
```
./run.sh
```
6.  Run "Cargo run" for any run of the game after that.
```
cargo run
```

## Roadmap

First and foremost, we are interested in a functioning equivalent of Frogger.  That means if you don't see the basic functionality you would by playing Frogger (which is available free online), our projects is still missing our primary benchmarks.

A run down of the features you will experience in our completed project:
* Crab for you to control
* Lanes, grass, and a river to cross
* Obstacles, such as vehicles (colored squares) or logs
* Score tracker (and a way to score points)
* Life counter which decrements when you die
* Timer
* Main menu
* High score page
* Goal end spaces for winning

A run down of our stretch goals for future completion:
* Online play option
* Multi-player option

## ggez

There are two common game building libraries for Rust: ggez and piston.  We chose ggez.  We relied heavily on the ggez library.  We also looked at several example games available on the ggez github page for ideas on how to structure our code: https://github.com/ggez/ggez
