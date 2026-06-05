---
id: add_terrain_movement_constraints
aliases: []
tags: []
description: Units cannot pass wherever they want. Add a configuration to state that kind of fact.
title: Add Terrain Movement constraints
---
## Movements constraints

- A Unit cannot pass on any terrain. Some are unwalkable (like Oceans for walkers, Plains for boats, etc.)
- Spawning must account for that new fact. Michel cannot spawn in the Ocean anymore !

## Lessons Learned

- In the future we will handle different terrain "speed", so better to acknowledge for a "difficulty" ranging from 0 (unpassable) to 100 (fully passable without penalty), but for now a simple "booleany" Component like `Moveable` is enough => [[Handle unit speed]]
- Not all Units behave the same way. Boats do not care about Oceans (but they care about Rivers), but Land units do not know how to swim 😁=> [[Create an Ocean Unit (boat)]]
