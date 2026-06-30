---
id: migrate_the_game_to_bevy_0.19
aliases: []
tags: []
description: Migrate the game to Bevy 0.19
title: Migrate the game to Bevy 0.19
---
## Content

- Migrate the whole game to Bevy 0.19
- Identify dependencies that needs to be updated alongside bevy (namely `noisy_bevy` to 0.14)
- Fix every error (compiler goes BRRRRRRR !)

## Lessons Learned

- [`bevy_framepace`](https://github.com/aevyrie/bevy_framepace) is not compatible with Bevy 0.19 (we cannot add it to our systems without an error). Since we don't absolutely need it for the time being, we removed it.
