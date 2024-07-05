Squaro: A Reinforcement Learning Game
Squaro is a game developed in Rust and the AI in python. The game features a movable square which destroys other shapes (enemies) upon contact, increasing the score by one.
Developed by: Fabio Gruosso



How-To start the game:
1.) Install Rust on your system: f.e. (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
2.) Extract squaro zip
3.) Create python .vev in project folder
4.) project folder, sh: "cargo build"
5.) start python: supversior.py
note: you can also start a not supervised instance of the game by just: project folder, sh: "cargo run"


Installation Errors:
error: failed to run custom build command for `python3-sys v0.5.2`:
That error has to do with missing Python development headers in the Python Installation



Name: Squaro
Language: Rust, AI: Python
Resource Usage: Game: 28MB, AI: 30MB, in total: 58MB + 4GB Cache + 4GB Disk Space
Game Features
Different Enemy Types: Circles and triangles alongside squares.
Enemy Behaviors: Enemies types have three different behaviors: chasing the player, moving randomly, and Standing still whilst fleeing from the Player if close.
Health System: The game had a Health System for the Player in an older Version (findable on GitHub).
Machine Learning Integration: Squaro integrates various AI libraries and frameworks through an internal interface between the game and the AI, overcoming dependency issues typically encountered with interdependent Python libraries. This interface approach can be generalized for other software and applications, including games, using memory reading techniques.
Technical Details: Rust and Python Integration
The connection between Rust and Python is established using cpython in Rust, with necessary environment variables and manual adjustments.



Performance Optimization


Q-Table Writes:
Writes to the Q-table occur every 1,000 iterations, significantly improving performance.
In-memory updates happen after each iteration, with disk writes every 10,000 iterations.

LRU Cache:
The LRU Cache capacity is set to 4 GB to enhance efficiency.

Asynchronous Disk Writes:
Asynchronous writes prevent game lags during disk operations.

Q-Table Pruning:
Pruning removes the least used 10% of entries, optimizing the stored Q-table.


AI Effectiveness
AI learning effectiveness can be tracked with the Supervisor.py
Supervisor Implementation
An "Agent" or Supervisor layer controls the program's execution and monitors its success.
The Supervisor starts the game, measures the score, and logs the results in a training_log.


