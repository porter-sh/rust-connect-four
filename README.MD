# rust-connect-four
```text
     ,,========..        ,,========..        ,,========..        ,,========..
    //  ,----.  \\      //  ,----.  \\      //  ,----.  \\      //  ,----.  \\
   //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\
  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||
   \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //
    \\  `----'  //      \\  `----'  //      \\  `----'  //      \\  `----'  //
     ``========''        ``========''        ``========''        ``========''
```
## Group
### Name
Iron Oxide
### Memebers
Porter Shawver (porters2)  
Alex Broihier (adb12)

## Project
The goal of this project is to make a connect-four game in Rust. We want to have local multiplayer, multiplayer over the internet, and singleplayer against an AI (both a perfect AI and machine learning variants). All of this underneath a pretty UI. This will allow us to learn about front and back end web dev in Rust (using Yew for the front end), machine learning in Rust, a connect four algorithm (connect four is a solved game, and a perfect algorithm exists), and we anticipate we will need multithreading either for managing inter-user communication, training of the AI, or both.

### Technical Overview
**High level:** we are going to make a yew web app that can play connect four locally and communicate with a server to play online.

**Slightly lower level:** local play is fairly basic with yew components. We can use routing to make the game mode appear in the URL, and communicate state to the board yew component. It seems like websockets are best suited for server communication, and we have found that it works with `gloo-net` on the client side, and `websocket` on the server side. Writing changes in state to the server should be farily straightforward, but reading from the server will require a separate thread to constantly listen for changes. We are thinking that we can have both clients verify if moves are valid or if the game is won as an attempt to prevent the client from cheating. For the AI, we know that connect four is a "solved game," in that there is always a correct move to be made. We are hoping to try to implement an algorithm that finds the best move more efficiently than testing all the future possibilities. Hopefully if we get this working, we can use it to train a neural network just for the experience of doing that. Then we could also train the NN to different amounts so that the player can select the difficulty of the AI.

#### Checkpoint 1 (11/5)
 - Full website layout with routing and and UI.
 - Directive on-screen messages, like "red wins", or "yellow's turn".
 - Game mechanics (placing pieces, stop once someone wins, etc.).
 - Undo button to revert the board state (even to the start of the game).
 - Online multiplayer.

#### Checkpoint 2 (11/30)
 - Ability to select a specific online room, and to spectate online games.
 - Ideally we would have our perfect AI mostly done.
 - ML based AI game modes / difficulty settings.
 - AI that plays in the background against itself on the home screen.
 - A FFI that optionally allows the server to use students' (specially complied) connect 4 mp binary to implement cheating detection.

### Possible Challenges
 - Learning how to use the Yew library, which is concerned less with writing a main loop but with creating components that react via callbacks.
 - Effectively sharing state across Yew components.
 - Finding a good ML library for Rust.
 - Managing state / handling networking errors for communications between multiple clients and the server.
 - Learning a perfect algorithm for connect four.

### Random Ideas
 - Let the player select which column to drop a disk into by entering a number.
 - Survival mode!!! Battle against an increasingly difficult AI and see how many rounds you can last.
