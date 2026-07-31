---
id: handle_unit_movement_points
aliases: []
tags: []
description: Units can be slow or fast. This means variable tile movement PER TURN
title: Handle Unit MovementPoints
---
## Content

- A Unit can move a specific distance in a single turn, which will be called its "**MovementPoints**"
- These MovementPoints will be expressed as absolute [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
- One turn will move the unit at most its MovementPoints value. In the future this unit will have several turns enqueued that will decrement the remaining distance by using its MovementPoints every turn until the target point is reached
- The visual representation of a unit MovementPoints must be matched by its movement selectors.
    🛑Impossible to reach tiles must not be covered by a movement selector jig 🛑
    🛑Out of map tiles must not be reachable 🛑
- Tiles will have variable "weight" regarding MovementPoints (e.g Marshes/Hills will be more difficult to move through)

## ToDo

- [x] A terrain unit like Michel cannot move through Mountains or Oceans
- [x] Add a "MovementPoints" property to Unit entities
- [x] Adjust Move selector to account for a Unit movementPoints (for Michel, it will have to cover a Manhattan Distance of 2 tiles)
- [x] Michel will have a MovementPoints of 2 tiles
- [x] A unit can be moved in several increments during one turn, as long as it does not expend all its Mouvement Points
- [ ] A Unit can only move at most its MovementPoints in one turn

## Lessons Learned

- Add variable MovementPoints alterations for some tiles (Marshes, Hills, etc.)
- Enqueue several movements at once (during several turns) that will be automatically performed at every turn end
