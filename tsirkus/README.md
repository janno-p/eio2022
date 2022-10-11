# 3. Snakes and Ladders (`tsirkus`)

*1 sec*  
*40 points*

Juku is not very good at board games. He decided to improve his skills before the next board game
night by practicing playing the game "Snakes and Ladders" by himself.

Game "Snakes and Ladders" is a simple board game: the game board consists of N squares, labelled 1
to *N*. The game pieces of all players start on square 1. On their turn a player rolls a six-sided
die and moves the game piece forward by as many squares as the number shown on the die (1 to 6).
Some squares, however, have the beginning of a ladder or a snake on them. Snakes take the game
pieces backward on the game board, ladders forward. If the player stops on a square on which a snake
starts, they must take their game piece to the other end of the snake. Similarly, if the game piece
stops where the ladder starts, they must move their piece to the other end of the ladder. A player
has won if their piece reaches square *N* or would go past it.

In order to learn how much room of development he still has, Juku wishes to know the least number of
dice rolls he needs to win the game.

**Input.** The first line of input contains *N* , the number of squares on the game board, and *M*,
the total number of snakes and ladders (2 &le; *N* &le; 10<sup>5</sup>, 0 &le; *M*). The following
*M* lines each contain integers *A* and *L* , the description of a snake or a ladder
(1 &lt; *A* &lt; *N*, 1 &le; *L* &le; *N*, *A* &ne; *L*):

* If *A* &lt; *L*, then this line describes a ladder that starts at square *A* and takes the player
  to square *L*.
* If *A* &gt; *L*, then this line describes a snake that starts at square *A* and takes the player
  to square *L*.

It is guaranteed that the values of starting points *A* are all distinct and that no value of *L* is
equal to the value of any *A*.

**Output.** The first line of output should contain the smallest possible number of dice rolls that
are necessary to win the game. The second line should contain those dice rolls, separated by spaces.
If there are multiple suitable minimum length sequences of dice rolls, then output any of them. If
it's not possible to win the game then output a single line containing the text `EI SAA`.

**Example.**

`Input`

    30 0

`Output`

    5
    5 6 6 6 6

**Example.**

`Input`

    7 1
    2 7

`Output`

    1
    1

In this example there’s one ladder on the board, leading from square 2 to square 7. Therefore it is
possible to get to the end with a single dice roll, by rolling 1, in which case the ladder takes the
game piece to the last square, or by rolling 6, in which case the game piece also ends up on the
last square.

**Example.**

`Input`

    31 6
    5 2
    6 2
    7 2
    8 2
    9 2
    10 2

`Output`

    EI SAA

In this example there are snakes starting from squares 5 to 10. It is possible to show that in this
case no sequence of dice rolls will let the game piece reach the end.

**Grading.** In this task, tests are divided into groups. For each group, only those solutions get
points that solve correctly all the tests in the group. In the test groups, the following additional
conditions hold:

1. (3 points) *M* = 0.
2. (3 points) *M* = 1.
3. (4 points) *N* &le; 10<sup>3</sup> and the input only contains snakes.
4. (4 points) *N* &le; 10<sup>3</sup>, the input only contains ladders, and there are no two ladders
   (*A*<sub>1</sub>, *L*<sub>1</sub>) and (*A*<sub>2</sub>, *L*<sub>2</sub>), for which
   *A*<sub>1</sub> &lt; *L*<sub>2</sub> and *A*<sub>2</sub> &lt; *L*<sub>1</sub>.
5. (6 points) *N* &le; 10<sup>3</sup> and the input only contains ladders.
6. (3 points) The input only contains ladders.
7. (7 points) *N* &le; 10<sup>3</sup>.
8. (10 points) No additional constraints.

If for some test in a group the second line of the output is incorrect, but the first line is
correct for all the tests in that group, then the solution gets 50% of the points of that group.
