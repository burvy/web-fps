# Web FPS Game
A game for my website at [burvy.dev](https://burvy.dev/)!  
You can also visit the game directly through [this link](https://burvy.dev/game)

# Building/Running
`game` and `game-server` must be built and ran separately  
`game-protocol` is automatically bundled into both of them

## `game` (client)
Building: `cargo build -p game --release`  
Running: `cargo run -p game`  
## `game-server` (server)
Building: `cargo build -p game-server --release`  
Running: `cargo run -p game-server`  

Note that this crate targets WASM, and the browser build is, on my machine, driven by a 
site at `burvy-dev/crates/game-wasm` (mine is built with Leptos). See my other repo 
if you would like to view the implementation. 
Since this is the case, there is no filesystem nor clock.

There also exist 3 different getrandom versions as dependencies within our tree require different 
versions of getrandom, js -> getrandom@0.2, wasm_js -> getrandom@0.3 and getrandom@0.4

WebGL2 is used for the browser, and looks a bit worse than native, without atmosphere, background, SSAO, 
order independent transparency.

Checking for errors using `cargo check --target wasm32-unknown-unknown` won't help much with errors 
that surface at runtime, especially from `std`, which compiles for WASM but certain calls like filesystem 
operations and attempting to access system time are not possible on the web and break.

# BASICS

## `protocol`
`game-protocol` contains two main things:  
`protocol` and `shared`.  
It may also contain other stuff, for example, `world`, 
which is in the same category as `shared`.

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
For this reason, Server and Client (the root `game`) do not require `serde` since Protocol does 
the lifting network-wise.   

Server and Protocol have default features turned off for `bevy` and `avian` so that a renderer/windows 
don't get packed into the build, because we don't need that on the Server and Protocol. They run headless.  
The client needs the renderer and windows because, we need to see stuff on the client.

For `lightyear`, there are *client* and *server* features, which must be toggled.  
`game-protocol` enables neither feature, the two programs on either side declare 
which one they are.

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
`Default`: The client always sends ticks, but if ticks never arrive, `Default` can be used to fill 
the hole for the current simulation step. For example, if tick 2005's input is missing and the server 
must simulate tick 2005 now, we can just use `Default`.  
`Default` is a neutral input for lightyear to fall back on when a sent-tick's input has been lost or 
hasn't arrived yet.

Also in the protocol, you may notice that three physics plugins are disabled:

```rust
app.add_plugins((
    PhysicsPlugins::default()
        .build()
        .disable::<IslandSleepingPlugin>() // DETERMINISIM ISSUES
        .disable::<PhysicsInterpolationPlugin>() // DUPLICATION ISSUES
        .disable::<PhysicsTransformPlugin>(), // DUPLICATION ISSUES
    LightyearAvianPlugin::default(), // MUST be added manually
    ));
```

It may look like free performance that we are ignoring, but those plugins actually conflict with 
determinism and duplication in lightyear. Please do not re-enable them. 

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

## `shared.rs`
`shared.rs` is shared by both client and server in `game-protocol`.  
Like `protocol.rs`, it contains things that should be similar between client and server, 
except that it is more simulation specific. For example, for both server and client to 
agree on how fast a player can go, `WALKSPEED` is defined here and reused whenever we 
want to simulate walkspeed, which is also conveniently in `shared.rs` as `apply_input`.  
If `WALKSPEED` were not shared between client and server, there would be disagreement, 
and as our system is server-authoritative, it may cause rubberbanding client side. 
This desync would easily occur when manually typing in the constants everywhere, 
then later on changing the constants in one area but not the other. For example, if 
`WALKSPEED` was 5 on the server but 4 on the client, the client would be lagged 
forward constantly because the server expects the client to be further ahead than 
they currently are. Vice versa, the client would be lagged back due to the opposite 
reason.

------
There is an important distinction in what belongs in `game-protocol` and what doesn't.
Think of it more like what the server uses to simulate the world, the client must
listen to that to be able to play. As the server is headless, world building in
here contains mainly colliders and such.
------
If something is not replicated over the network, it can be directly compiled from the 
shared library into both server and client instead of wasting bandwidth
------

### `apply_input`
Simulation constants aren't the only things shared between client and server. The 
two also must agree on how movement happens. For example, walking is done by reading 
the 2d motion vector from `PlayerInputs`, clamping magnitude to `[0, 1]` and 
multiplying by `WALKSPEED`, then setting the entity's velocity to that.

Note that we will touch motion on the `x` and `z` axes, but we leave `y` to the physics system, it 
uses it for gravity.

We will also modify the rotation of the object in accordance to the yaw and pitch 
sent by the player in the `PlayerInputs` `ActionState`.

### Replicate
Replication is for state that one side owns and changes. Static constants can 
just be put in `game-protocol` for both sides to compile into their builds to 
save on bandwidth.

### PredictionTarget
`PredictionTarget` exists on `server.rs`. The documentation in lightyear itself states that 
it controls which clients should run client-side prediction for that entity. In our case, we send 
the `PredictionTarget` signal for all players, because all players are physics objects. Each client 
predicts every other client's actions and rolls back if incorrect. Though this can cause very jarring 
jitter, it also means that physics collisions resolve identically everywhere, which wouldn't be possible 
if we simply send a visual position that doesn't react instantly to collisions. 

### ControlledBy
The server sets a certain `Entity` with `ReplicationSender` to control a certain entity here, 
and replicates it to everyone, with one client getting the `Controlled` marker, which they can 
use to target themselves for control on their own end. `ControlledBy` essentially allows a 
certain client to *own* an entity on the server. 

The `lifetime` field on `ControlledBy` determines what happens after disconnect, and for our case, 
with our `SessionBased` lifetime, we simply just despawn the entity.

For `PredictionTarget` and `ControlledBy`, they can essentially be described as such:  
`ControlledBy`: Who owns the entity
`PredictionTarget`: Who simulates the entity early

### `run_loop` vs `tick_duration`
the `run_loop` is a faster loop than `tick_duration`.

Imagine a bus schedule, `tick_duration` is a bus every 15 minutes, while `run_loop` is how often 
you glance for a bus. Glancing every 4 minutes means the bus could have been there for a whole 4 minutes 
before you notice the bus. Glancing every 15 minutes means the bus could have been there for 15.

We must set `run_loop` faster so we notice that we should tick faster. We could have missed the tick 
window for a whole 15ms if `run_loop` and `tick_duration` were both 15ms. On the other hand, we only 
miss by 4ms at worst if `run_loop` is 4ms.

### `digest.txt`
The digest is a SHA-256 fingerprint of the certificate the server signs and is regenerated with every run, 
which means it shouldn't be committed. 


The client reads a digest the server gives them, to use as the digest that validates the server's actually 
the server to connect to. Actual certificate authorities are saved in the browser, and 
would not require the `digest.txt` to be sent between the client and the server explicitly. 
This file really only exists to work around not having a certificate authority.

Note that `hash().to_string()` returns a colon seperated hex like `1a:2b:...`, but `from_hex` wants 
a 64-character hex. The colon separated hex has 95 characters while the raw hex is 64 characters.  
That's why you need to strip the colons.

Note that we cannot just send the `digest.txt` over the webtransport connection, to match what we have now 
with the client and server sharing one directory. We need the digest to actually connect to the webtransport 
server; the digest comes with actually spawning the client connection:

```rust
let client = cmds
    .spawn((
        Client::default(),
        // ...
        WebTransportClientIo {
            certificate_digest: digest.0.clone(), // <-- HERE!
        },
        // ...
    ))
    .id();
cmds.trigger(Connect { entity: client });
```

Even if ordering allowed this, an impostor can send their own certificate and this would also 
pass, making this insecure. 

Once we have an actual certificate authority, which there will be on the release builds, 
this `digest.txt` workaround is no longer necessary except on the dev build.

In the WASM build, the digest is passed when the game is started up rather than stored in a file because 
WASM builds do not have a filesystem.

# SERVER
You may notice this line:  
```rust
app.add_input_validator(
    authorize_controlled_targets::<NativeStateSequence<protocol::PlayerInputs>>,
);
```
This line is not enabled by default, 
but it prevents clients from tricking the server into thinking they are actually another 
entity. This prevents hackers from causing chaos by spoofing entity ids. Don't delete it!

# IP Certificate
An IP Certificate is the same object as a TLS Certificate, only different by one field, which prevents 
it from working on browsers like Safari.  

A Certificate Authority (CA) asserts that a certain *public key* belongs to this *name*. The browser 
is then responsible for checking that the name they were asked to visit appears in 
the certificate.  

Names are typed, and live in the Subject Alternative Name extension, which carry their type tags:  
`dnsName` holds something like `webtrans.burvy.dev`  
`ipAddress` holds an IP address, like `123.456.768.90`  

As of 8/31/26, `client.rs:66` uses ipAddress, so the browser looks for an ipAddress SAN, 
but my webtrans.burvy.dev certificate only has a dnsName, which does not say anything 
about ipAddress. 

This was rare until recently, because it was easier to verify who owned a domain as 
opposed to owning an ip address, as domains can be traced through registrars and 
stuff like WHOIS. Let's Encrypt issued SANs only [recently](https://letsencrypt.org/2025/01/16/6-day-and-ip-certs)

It's only 6 days because IP addresses last for a short time and can be shuffled around easier than 
domains. You can only prove you control an IP address using `http-01` and `tls-alpn-01` for now, 
`dns-01` does not work.
