---
id: Move Settler
aliases: []
tags:
  - phase1
description: Add the ability for the player to select the Settler unit and then click on a nearby tile to move it
title: Move Settler on the map
---
## Can be selected

- Ability to select a tile by raycasting on the Z-axis (z-index) to actually find "what" is selected, and in our case, if it's the Settler unit
- If it's the Settler unit, display other "target selectors" (displayed with another color) to indicate possible tiles to go:
 \[ \] \[ \] \[ \]
 \[ \] \[S\] \[ \]
 \[ \] \[ \] \[ \]

## Can be moved
- When the "target selectors" are displayed, clicking on one of them move the Settler to the target tile
- If clicked outside of these selectors, clear the selection
