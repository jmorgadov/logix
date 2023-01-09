# Logix

Project that contains a set of tools related to electronic circuits (mainly for
digital circuits).

> :construction: This project is in its early stages and constantly evolving. All names, structures, etc. may change in the future.

## Crates

### `logix_core`

Contains the basic structures to create components.

The principal design goal of this crate is to allow the creation of
components of any kind. Therefore, there is not implementation of any
*basic* or *primitive* component here, neither components know their
behavior (how to compute its outputs).

### `logix_sim`

Contains some primitive components (logic gates, clock, constants) and
implements their behavior. Simulates nested components made using those
primitives.

## Possible future crates

### `logix_io`

Library crate that allows component serialization (and deserialization) in
different formats.

> Possible formats: JSON, binary...

### `logix_lang`

DSL that allows the creation of components in an easy way.

The first goal is that this DSL can be use to declare a component structure and
store it in some way (maybe using `logix_io`).

Then, maybe implement the possibility to specify the components behaviour and
with the help of something like `logix_sim` simulate them.

### `logix_vhdl`

Library crate the allows to obtain the
[VHDL](https://en.wikipedia.org/wiki/VHDL) code of a component.

The inverse conversion may be implemented as well but taking into account
that the behavior information of the components will be lost. Maybe the
conversion could be made into `logix_lang` (if it allows to specify
behaviors).
