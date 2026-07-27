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
networking stuff, which requires [serialization](#serialization)/deserialization.  

Protocol requires serialization from `serde` because it handles most of `lightyear`'s networking. 
It's a lot easier to have serialization over a network for cleaner data packing and unpacking across 
different systems.  
For this reason, Server and Client (the root `game`) likely do not require `serde` since Protocol does 
most of the lifting network-wise. `serde` may be removed from client and server if it turns out that 
Protocol holds all the serialization/deserialization.  

Server and Protocol have default features turned off for `bevy` and `avian` so that a renderer/windows 
don't get packed into the build, because we don't need that on the Server and Protocol. They run headless.  
The client needs the renderer and windows because, we need to see stuff on the client.

For `lightyear`, there are *client* and *server* features, which must be toggled.  

### SERIALIZATION
For those who don't know what serialization is, it's about turning memory values into a flat sequence 
of bytes. Deserialization decodes those bytes back into values.  

It's like shipping a chair, the sender first disassembles the chair into pieces (for easier transport), 
then the receiver reassembles the chair based on agreed-upon instructions. `serde` does this to our data.  

If we didn't agree on how to assemble the chair, the chair might not be assembled the way we expect.

# PROTOCOL
Our first two points would be `PlayerMarker` and `PlayerInputs`.

`PlayerMarker` is attached to player entities, and allows us to filter out players through ECS.  
`PlayerInputs` rides inside `ActionState<PlayerInputs>`, and `ActionState` is attached to player 
entities by **Lightyear**.  

Anything attached to an entity is a **Component** of that entity, and thus must derive `Component`.
Since `PlayerMarker` is directly attached to a player, it derives `Component`. `PlayerInputs` simply rides 
inside `ActionState`, and thus `PlayerInputs` does not require `Component`, even if `ActionState` is a 
component.

`PlayerMarker` is like a sticker that is stuck on a box to allow us to identify it.
`PlayerInputs` is like data inside an envelope, which is stuck on a box.

***
`Component` is basically like the adhesiveness. `PlayerMarker` needs to be adhesive because it sticks onto 
the box directly. `PlayerInputs` does not require adhesives because it rides inside an envelope that 
has adhesives to stick onto the box.
***

Taking a look at `PlayerMarker`, the components are as such:  
```rust
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Component)]
pub struct PlayerMarker;  
```
For each:  
`Clone`: Allows `lightyear` to duplicate entities into a history buffer for rollback, even if 
the component has no data.  
`Serialize`/`Deserialize`: `serde` derives that allow this component to be converted to and from bytes 
over the network.  
`Debug`: Allows us to print this for debugging  
`PartialEq`: Allows `lightyear` to check if this component actually changed instead of wasting bandwidth 
sending this. More importantly though, this allows the client to know whether the server disagreed with 
its prediction. 
`Component`: Required because we attach `PlayerMarker` to an entity directly.  

`PlayerInputs` doesn't have `Component`, but has `Reflect` and `Default`.  
`Reflect`: Simply allows the computer to see what is inside the struct without having to explicitly define 
it, which is especially difficult to do during runtime. As a result of this, `bevy` or `lightyear` can 
simply know what is inside the `PlayerInputs` struct by looking at it.  
`Default`: Even when there is no input, we must send something. In the case of a `motion` `Vec2`, 
if the player is not pressing anything, the default `Vec2 { x: 0.0, y: 0.0 }` is sent, indicating 
that the player is not pressing any motion buttons. We cannot just send nothing, that wouldn't be 
very reassuring.

## MapEntities
`MapEntities` is a trait bound required by `InputPlugin`. It is necessary especially when a struct has 
fields that contains entities.  
When Entity IDs are sent over the network by a sender, the entity the ID was pointing to is not 
necessarily the same entity the receiver would be pointing to. It is like telling someone to call 
the third contact on their phone, for each person it would likely call a different contact. If 
you want them to contact the same person, there needs to be a mapping between the third contact on
one person's phone, who that person is, and which contact that would be on your phone, say the 5th 
contact on your phone. Thus it is required that a mapping exists for entities when we point at 
entities using `EntityId` and attempt to broadcast that over the network.
