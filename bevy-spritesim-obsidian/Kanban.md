---

id: Kanban
aliases: []
tags: []
kanban-plugin: board

---

## ToDo

- [ ] [[Define Terrain Layer Hierarchy]]
- [ ] [[Turn-by-turn system]]
- [ ] [[Define a keyboard layout]]
- [ ] [[WASM Build]]
- [ ] [[Define Z-index rules]]
- [ ] [[Handle Unit Speed]]
- [ ] [[Create an Ocean Unit (boat)]]


## InProgress

- [ ] [[Decide what to do with the camera]]


## Done

**Complete**
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
{"kanban-plugin":"board","list-collapse":[false,false,true],"metadata-keys":[{"metadataKey":"description","label":"Description","shouldHideLabel":false,"containsMarkdown":false},{"metadataKey":"phase","label":"Phase","shouldHideLabel":false,"containsMarkdown":false}],"move-tags":true,"tag-action":"kanban","tag-colors":[{"tagKey":"#phase","color":"rgba(236, 250, 137, 1)","backgroundColor":""}]}
```
%%