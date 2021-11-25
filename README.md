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

The server uses address *127.0.0.1:5555* by default, it could be changed by running

```
cargo run --bin wuzi <host>:<port>
cargo run --bin player <host>:<port>
```

The ``Score`` struct holds scores for horizontal, vertical diagonal directions, when player place a color, score is added accordingly. Score is represented in bits, for 5-in-a-line the full score is *11111* in binary, the first one to reach *0x1f* in any direction to be the winner.


### Communication protocol

#### register 

Register player to the game, the server reply with assigned color.

request

```
register
```

response

```
<color value>
```

### place

Place a color in slot

request

```
place <index> <color value>
```

Server responses *"done"* on successed, an error message is return if anything goes wrong.

#### get_board

Get current board state

request

```
get_board
```

Server responses a string with semicolon as delimiter to represent the board.


#### get_winner

request

```
get_winner
```

Server responses ``Unknown`` if game hasn't ended, ``NoWinner`` for no one win, ``Winner <color.value>`` for one of the player won.


#### can_place

Check whether player can place a color in slot.

request

```
can_place
```

Server responses either ``true`` or ``false`` accordingly.


#### reset_board

Reset the board to initial state, the request could be sent if both players would line to have another round.

request

```
reset_board
```
