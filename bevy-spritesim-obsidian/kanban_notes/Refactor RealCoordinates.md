---
id: refactor_realcoordinates
aliases: []
tags:
  - phaseX
description: Draw tiles based on their MAP_INDEX instead of handling a dual-coordinates system
title: Refactor RealCoordinates
---

## The dual-coordinates system is BAD

- Need to handle both system (no kidding !)
- Updates to one must be carried on the other (and which one ? yeah, thought so too…)
- Moving tiles are based on World coordinates that are converted to "Real coordinates" that would then be converted to "Map coordinates" ?! THIS IS MADNESS (No, this is Patrick)

## So ?

- Implement a SINGLE coordinates system, based on the MAP index, with a dedicated `Component`
- All tiles should follow this system
- Moving a tile should thus means to move its Component to the right MAP coordinates (with "snapping" on the right tile)

## Plan
- Migrate from RealCoordinates to Entity Transform coordinates
- Migrate Map from a HashMap(Coords, Tile) to a Vec(Tile)

## Lessons learned
- We can click on a tile sprite but not "on" the sprite itself but this does not trigger the "Move Unit" action -> [[Fix Sprite Tile selection]]
- Z-index is messed up -> [[kanban_notes/Define Z-index rules|Define Z-index rules]]
