---
id: handle_camera_zoom
aliases: []
tags: []
description: Zooming in and out with the Scroll Wheel.
title: Handle Camera Zoom
---
## Camera initialization

- Initial Zoom is fine

## Camera movement

- For the moment, the camera is anchored to Michel. Nice but can be improved:
	- Free cam when nothing is selected
	- Anchor to selected unit when a selection action happens

## Camera Zoom

- Zooming in and out should be done with mouse scroll
- Zooming in and out should be done with a discrete step that does not make weird lines appear (multiple of `SPRITE_SIZE` ?)

## ToDo
- [ ] Scroll in and out (with clamping) with mouse scroll
- [ ] Handle "anchor on selection"