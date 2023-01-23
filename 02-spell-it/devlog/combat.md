# Combat

## Draft

* Occurring during a stage
* Each character (player, enemies) has a Speed statistic
* Default value is 100

* Create a Game Master struct holding a mutable reference to the game state
* It will call systems on it
* Draw function calls can stay in main

* The Game Master struct will get updates on each tick.
* An initialize function will allow it to start a combat system and get the current stage

* A combat is represented as a list of turns
* A turn represents a player or enemy action
* At the beginning of each stage, we take the player and enemies in a list, sort them by decreasing speed value and place them in a sequence
* When the player or the enemy plays his turn, he's put at the end of the sequence.
* At the end of the sequence, we check if a creature has enough speed to act again:
  * If so, we place a bonus turn for that creature
  * If not, we place the remainder into the creature "recipient"
    * When we do another check, if that recipient holds enough value, we place a bonus action, and reset it
* If the speed of the player or a creature changes, we recalculate the sequence.

## Inspiration (From "For The King")

Priority: Current Turn / Speed x 100
(the lowest acts first.)

#1:
Player has 70 Speed
Goblin A has 50 Speed
Goblin B has 50 Speed
Goblin C has 50 Speed

  1 / 2 / 3 / 4 / 5 / 6 / 7 / 8 / 9 / 10

P 1.43 
A 2
B 2
C 2

## Examples of desired result

#1:
Considering everyone has the same speed (100):
p (Player), a, b, c (enemies)

Turns will look like this:
p, a, b, c, p, a, b, c, p, a, b, c...

#2:
Consider the player has double the speed of everyone else (200)

Turns will look like this:
p, p, a, b, c, p, p, a, b, c, p, p, a, b, c...

#3:
Consider the player has 133 Speed:

Turns will look like this:
p, a, b, c, p, a, b, c, p, p, a, b, c