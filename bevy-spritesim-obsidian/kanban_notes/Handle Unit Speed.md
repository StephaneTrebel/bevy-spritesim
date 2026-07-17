---
id: handle_unit_speed
aliases: []
tags: []
description: Units can be slow or fast. This means variable tile movement PER TURN
title: Handle Unit Speed
---
## Content

- A Unit can move a specific distance in a single turn, which will be called its "**Speed**"
- This Speed will be expressed as absolute [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
- One turn will move the unit at most its Speed value. In the future said unit will have several turn enqueued that will decrement the remaining distance by using its Speed every turn
- The visual representation of a unit Speed must be matched in its movement selectors.
    🛑Impossible to reach tiles must not be covered by a movement selector jig 🛑
    🛑Out of map tiles must not be reachable 🛑
- Tiles will have variable "weight" regarding Speed (e.g Marshes/Hills will be more difficult to move through)

## ToDo

- [x] Add a "Speed" property to Unit entities
- [x] Michel will have a Speed of 2 tiles
- [ ] A Unit can only move at most its Speed in one turn
- [ ] Adjust Move selector to account for a Unit speed (for Michel, it will have to cover a Manhattan Distance of 2 tiles)

## Lessons Learned

- Add variable Speed alterations for some tiles (Marshes, Hills, etc.)
- Enqueue several movements at once (during several turns) that will be automatically performed at every turn end
