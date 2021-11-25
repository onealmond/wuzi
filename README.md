Wuzi is a game similar to tictactoc, except

* The two players are represented as two colors, usually red and green.
* The board is a vertical 5x5 matrix, the top is open allow player to drop bullets into slots, bullet put into a slot goes directly to the bottom of it.

The game ends when

* One color reach 5-in-a-line vertically, horizontally or diagonally, which is the winner.
* The board is full but none of the colors get a 5-in-a-line, then no winner.


```
_______________    _______________    _______________    _______________
[G][ ][ ][ ][ ]    [ ][ ][G][ ][ ]    [G][ ][ ][ ][ ]    [ ][ ][ ][ ][R]
[G][ ][ ][ ][ ]    [ ][ ][G][ ][ ]    [R][G][ ][ ][ ]    [ ][ ][ ][R][G]
[G][ ][ ][ ][ ]    [R][R][R][R][R]    [R][R][G][R][ ]    [G][ ][R][G][R]
[G][ ][R][R][ ]    [G][G][G][R][G]    [R][R][R][G][ ]    [G][R][R][R][R]
[G][R][R][R][G]    [G][R][R][G][R]    [G][R][G][G][G]    [R][G][G][G][G]
+-+-+-+-+-+-+-+    +-+-+-+-+-+-+-+    +-+-+-+-+-+-+-+    +-+-+-+-+-+-+-+

   Green won           Red won           Green won           Red won
```

The project implemented the game in command line, the two players connect to server over TCP.


Start the server

```
cargo run --bin wuzi
```

Join the game

```
cargo run --bin player
```
