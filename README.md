# Logix

Project that contains a set of tools related to electronic circuits (mainly for
digital circuits).

> :construction: This project is in its early stages and constantly evolving.  All names, structures, etc. may change in the future.

## Crates

### `logix_core`

Contains the basic structures to create components.

The principal design goal of this crate is to allow the creation of components
of any kind. Therefore, there is no implementation of any
*basic* or *primitive* component here, neither the components know their
behavior (how to compute its outputs).

### `logix_sim`

Contains some primitive components (logic gates, clock, constants) and
implements their behavior. Simulates nested components made using those
primitives.

### `logix_lang`

DSL that allows the creation of components easily.

See example
[here](https://github.com/jmorgadov/logix/blob/main/crates/logix_lang/examples/main.lgx).

### `logix_gui`

GUI application for designing and simulating digital circuits.
