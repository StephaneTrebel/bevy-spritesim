---

kanban-plugin: board

---

## ToDo

- [ ] Define Terrain Layer Hierarchy (base terrain, features, zones, biomes, specials, etc.)
	#phase0
- [ ] Move Settler:
	- Can be selected
	- Can be moved (not on oceans)
	#phase1
- [ ] WASM Build
	#other
- [ ] Handle BÉPO (and other non-QWERTY layouts 😅)


## InProgress

- [ ] Add a Winning screen:
	- Triggered when a village is built by the player
	- "Yay !"
	- And then exit
	#phase1


## Done

**Complete**
- [x] Add a Village sprite:
	- Displayed on the map when created
	#phase1
- [x] Add a playable "unit":
	- Can "act" (create a village)
	#phase1
- [x] Generate Map
	#phase0
- [x] Display Map
	#phase0
- [x] Display Selector on tiles
	#phase0


***

## Archive

- [ ] Gérer les variantes (les "coins" etc.) des tuiles qui en ont (plaines,
	  déserts, forets, etc.) dans chacune des couches (Layers)
- [x] Gérer le lien (draw_call) entre la génération de la Map (qui sera une ressource, donc) et les SpritexxxxxAtlas (un seul Atlas ?)

%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false,false,true]}
```
%%