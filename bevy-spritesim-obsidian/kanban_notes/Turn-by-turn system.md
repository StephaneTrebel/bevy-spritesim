---
id: define_the_turn-by-turn_system
aliases: []
tags: []
description: Define how the Turn-by-turn system will be implemented
title: Define the Turn-by-turn system
---
## Content

### What is a turn ?

- A turn is a "unit of time", orthogonal to our perception of time (seconds, minutes, etc.). Think "chess turns"
- A turn is discrete, and integer. There can be 0 turns, 1 turn, 3 turn, 18282 turns
- A unit that "does something" will "spend" at least one turn to do so
- Turns starts at 0 and then go upwards up to Infinity (@TODO to be defined)
- Every unit can be used during a Turn to do "something" (if it's available to do so). It can be moved, attack another unit, create a village, and so on
- A Unit can "enqueue" several Turns (in a buffer) for an action through "Action points consumption", so that they can then be "spent" over time whenever a turn ends

### How to visualise what turn we are in ?

- A UI element will always be present on the "Map" state, that will show what is the current "turn".
- A UX consideration would to present it as a "day" (January 1st, and then January 2nd, etc.), but this raises the question of Years: Should we consider a base Year or treat the first year as "Year 0" ? @TODO to be defined

### How are turns ended ?

- A UI element (Button) "End Turn" is available, which will "end the current turn" and then start the new one

## ToDo

- [ ] Add a UI element that shows the current turn
- [ ] Add a UI element (button) that trigger the end of a turn

## Lessons Learned

- Handle a unit "buffer/queue" for actions that takes several turns
- Once a unit has spents its turn, it cannot do anything else
- Have a way to easily identify units that still can do something during a turn from units that cannot anymore
- Multiplayer/AI players for the turn paradigm to make any sense ? :D
