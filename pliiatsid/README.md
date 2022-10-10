# 2. Flipping Pencils (`pliiatsid`)

*1 sec*  
*30 points*

Pille has *N* pencils side by side in a box. The pencils are numbered 1, 2, &hellip; , *N* from left
to right.

Pille wants all the pencils to point the same way. But it may happen that some pencils have the
sharp end pointing up and some have it pointing down. Then Pille can take any consecutive set of
pencils and flip them all around in one move. If needed, she can perform several such moves.

Pille wants to know which pencils she should flip on each move to make all the pencils point the
same way in the smallest possible number of moves. If that helps, she may flip some pencils back and
forth repeatedly.

**Input.** The first line of input contains *N* , the number of pencils (1 &le; *N* &le; 1&nbsp;000). The
second line contains exactly N letters, where the letter t means that the sharp end of the pencil
points up and the letter n that it points down.

**Output.** The first line of output should contain *M* , the number of moves needed. Each of the
following *M* lines should contain *A*-*B*, indicating that one move should flip all the pencils
from pencil number *A* to pencil number *B* (pencils *A* and *B* both included). It does not matter
if by the end of all the moves all pencils point up or down. It is only required that they all point
the same way. If there are several flipping plans with minimal number of moves, output any one of
them.

**Example.**

`Input`

    10
    tttnntnttt

`Output`

    2
    4-7
    6-6

Here, the first move flips pencils 4 to 7, resulting in `ttttttnttt`. Then the pencil 6 should be
flipped back to have them all point the same way. Note that this is not the only way to get all
the pencils to point the same way, but there is no way to do it with less than two moves.

**Grading.** The tests are divided into three groups, where the following conditions hold:

* (2 points) *N* = 2.
* (9 points) *N* &le; 10.
* (19 points) *N* &le; 1&nbsp;000.

In each test case, half the points are awarded for outputting the correct *M* . For full score, both
*M* and the sequence of moves must be correct.
