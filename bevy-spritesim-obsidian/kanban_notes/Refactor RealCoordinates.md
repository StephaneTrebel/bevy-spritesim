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
- Moving tiles are based on World coordinates that are converted to "Real coordintates" that would then be converted to "Map coordinates" ?! THIS IS MADNESS (No, this is Patrick)

## So ?

- Implement a SINGLE coordinates system, based on the MAP index, with a dedicated `Component`
- All tiles should follow this system
- Moving a tile should thusly means to move its Component to the right MAP index
