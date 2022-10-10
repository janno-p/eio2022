# 1. Delivery Robot in Manhattan (`robot`)

*1 sec*
*20 points*

A self-driving robot is delivering packages in the Manhattan district of New York. The robot is
quite large and can deliver multiple packages to different destinations in one go. After delivering
all of the packages, the robot must return to its warehouse to pick up new packages.

In Manhattan, all streets form a grid: every street is a straight line either exactly North to South
or exactly East to West. The distance between consecutive parallel streets is also always the same.

Let us call movement along one street from one interesction to the next a step of the robot.

Find how many steps it takes at minimum for the robot to get back to its warehouse.

**Input.** The first line of input contains *N* , the number of steps taken by the robot so far
(1 &le; *N* &le; 1 000). The second line contains *N* characters, each of which gives the direction
of one step of the robot: `N` for a step to North, `E` for a step to East, `S` for a step to South,
and `W` for a step to West.

**Output.** On the only line of output, print how many steps it takes at minimum for the robot to
return.

**Example.**

`Input`

    5
    NNEEE

`Output`

    5

The robot has moved 2 steps to North and 3 steps to East. There is no shorter way for the robot to
get back than to simply retrace its steps, going 2 steps to South and 3 steps to West. Thus, the
answer is 5.

**Example.**

`Input`

    7
    NNNSSWE

`Output`

    1

The robot has moved 3 steps North, 2 to South, 1 to West and 1 to East. The steps to East and to
West cancel each other out, as do two steps to North and the steps to South. Thus, the robot is just
one step away from its starting point and the answer is 1.

**Grading.** The tests are divided into five groups, where the following conditions hold:

* (1 point) *N* = 1.
* (1 point) *N* = 2.
* (2 points) Robot only moved in one direction.
* (5 points) Robot only moved along one street, that is, either North-South or East-West.
* (11 points) No further constraints.
