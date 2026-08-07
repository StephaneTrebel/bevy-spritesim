---

id: Kanban
aliases: []
tags: []
kanban-plugin: board

---

## ToDo

- [ ] [[starting menu]]
- [ ] [[Animate_unit_movement]]
- [ ] [[Define Terrain Layer Hierarchy]]
- [ ] [[Create an Ocean Unit (boat)]]
- [ ] [[Handle Camera Panning through Drag&Drop]]
- [ ] [[Add variable MovementPoints alterations for some tiles (Marshes, Hills, etc.)]]
- [ ] [[Enqueue several movements at once (during several turns) that will be automatically performed at every turn end]]
- [ ] [[Define Z-index rules]]
- [ ] [[WASM Build]]
- [ ] [[Define a keyboard layout]]
- [ ] [[Have a way to easily identify units that still can do something during a turn from units that cannot anymore]]
- [ ] [[Multiplayer]]


## InProgress



## Done

**Complete**
- [x] [[BUG - Ending a turn must replenish all MovementPoints]]
- [x] [[Handle Unit Speed]]
- [x] [[Turn-by-turn system]]
- [x] [[Bevy 0.19 migration]]
- [x] [[Decide what to do with the camera]]
- [x] [[Fix Sprite Tile Selection]]
- [x] [[Add terrain movement constraints]]
- [x] [[Refactor RealCoordinates]]
- [x] [[Move Settler]]
- [x] [[Add a Winning screen]]
	- Triggered when a village is built by the player
	- "Yay !"
	- And then exit (errocode=0)
	#phase1
- [x] [[Add a Village sprite]]
	- Displayed on the map when created
	#phase1
- [x] [[Add a playable unit]]
	- Can "act" (create a village)
	#phase1
- [x] [[Generate Map]]
	#phase0
- [x] [[Display Map]]
	#phase0
- [x] [[Display Selector on tiles]]
	#phase0


***

## Archive

- [ ] Gérer les variantes (les "coins" etc.) des tuiles qui en ont (plaines,
	  déserts, forets, etc.) dans chacune des couches (Layers)
- [x] Gérer le lien (draw_call) entre la génération de la Map (qui sera une ressource, donc) et les SpritexxxxxAtlas (un seul Atlas ?)

%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false,false,false],"metadata-keys":[{"metadataKey":"description","label":"Description","shouldHideLabel":false,"containsMarkdown":false}],"move-tags":true,"tag-action":"kanban","tag-colors":[{"tagKey":"#phase","color":"rgba(236, 250, 137, 1)","backgroundColor":""}]}
```
%%