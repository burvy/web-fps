# Web FPS Game
A game for my website at [burvy.dev](https://burvy.dev/)!  

# Building/Running
`game` and `game-server` must be built and ran seperately  
`game-protocol` is automatically bundled into both of them

## `game` (client)
Building: `cargo build -p game --release`
Running: `cargo run -p game`
## `game-server` (server)
Building: `cargo build -p game-server --release`
Running: `cargo run -p game-server`

# BASICS

## `protocol`
`game-protocol` contains two things:  
`protocol` and `shared`.  

This folder contains things that are shared between both server and client 
for things like simulation, prediction, etc...  
This can include, but is not limited to:  
- Player Walkspeed
- Player Bounds
- Game Logic
- Packet Information
This does NOT include:  
- Textures (client-side)
- Sounds (client-side)
- Assets that are client side only (client-side)
- Databases (server-side)
- Server-side logic (server-side)

## IMPORTS
You may notice that `game-protocol`, `game-server`, and `game` (root) contain different imports.  
The reason behind this is because of the server being headless, and protocol handling most of the 
networking stuff, which requires (serialization)[#serialization]/deserialization.  
Protocol requires serialization from `serde` because it handles most of `lightyear`'s networking. 
It's a lot easier to have serialization over a network for cleaner data packing and unpacking across 
different systems.  
For this reason, Server and Client (the root `game`) likely do not require serde since Protocol does 
most of the lifting network-wise.  
Server and Protocol have default features turned off for `bevy` and `avian` so that a renderer/windows 
don't get packed into the build, because we don't need that on the Server and Protocol. They run headless.  
The client needs the renderer and windows because, we need to see stuff on the client.

### SERIALIZATION
For those who don't know what serialization is, it's about turning memory values into a flat sequence 
of bytes. Deserialization decodes those bytes back into values.  
It's like shipping a chair, the sender first disassembles the chair into pieces (for easier transport), 
then the receiver reassembles the chair based on agreed-upon instructions. `serde` does this to our data.  
If we didn't agree on how to assemble the chair, the chair might not be assembled the way we expect.
