# rust-connect-four
## Group
### Name
Iron Oxide
### Memebers
Porter Shawver (porters2)

Alex Broihier (adb12)
## Project
The goal of this project is to make a connect-four game in Rust. We want to have local multiplayer, multiplayer over the internet, and singleplayer against an AI (both a perfect AI and machine learning variants). All of this underneath a pretty UI. This will allow us to learn about front and back end web dev in Rust (using Yew for the front end), machine learning in Rust, a connect four algorithm (connect four is a solved game, and perfect algorithms exist), and we anticipate we will need multithreading either for managing inter-user communication, training of the AI, or both.
### Technical Overview
#### Checkpoint 1 (11/15)
 - Full website layout with routing and and UI.
 - Directive on-screen messages, like "red wins", or "yellow's turn".
 - Game mechanics (placing pieces, stop once someone wins, etc.).
 - Undo button to revert the board state (even to the start of the game).
 - Online multiplayer.
#### Checkpoint 2 (11/30)
 - Ideally we would have our perfect AI mostly done.
 - ML based AI game modes / difficulty settings.
 - AI that plays in the background against itself on the home screen.
 - A FFI that optionally allows the server to use students' (specially complied) connect 4 mp binary to implement cheating detection.
### Possible Challenges
 - Learning how to use the Yew library, which is concerned heavily...
 - **TODO**

### Random Ideas
 - Let the player select which column to drop a disk into by entering a number.
 - Survival mode !!!
 - Undo move option (would need to store move history)
