# Onigiri - literally bevy wrapper (framework? lol)

## Fork of `bevy_editor_pls`

- Independence since `6062b86`

## Goal

- Standalone high performance & efficient editor
- High performance level `ser-de`
- Assets optimization (obsfuscation and encryption maybe)
- Target wasm (native is ok)

## Description

Low motivated recent day. Decided to push this piece of junk (at least it just work (partialy)). Im moving onto a new project until get motivated again (likely the next time is kotlin game engine) At least it work better than "dungeonlight" lol.

## Purpose

Fill the emptyness of my github profile atm. Future recruiters please don't read this T-T

## Plans

none

## How to use this? atm

Run onigiri crate. It should work like bevy_editor_pls atm, no modification atm. Press compile after a little play with the editor. The standalone should behave normal if:

replace bincode crate with normal serde serializer in scene.rs (it might work). Idk why bincode somehow create '&str'. Btw have fun.