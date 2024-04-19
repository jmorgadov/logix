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

```
Adder (
    subc (
        and: And(2),
        xor: Xor(2)
    )

    ins (b0, b1)
    outs (out, rest)

    design (
        b0 -> xor.0,
        b1 -> xor.1,
        xor.0 -> out,

        b0 -> and.0,
        b1 -> and.1,
        and.0 -> rest
    )
)

Main (
    subc (
        adder: Adder,
        clk: Clock(1.0),
        c2: High
    )

    ins ()
    outs ()

    design (
        clk.0 -> adder.b0,
        c2.0 -> adder.b1
    )
)

```

## Possible future crates

### `logix_io`

Library crate that allows component serialization (and deserialization) in
different formats.

> Possible formats: JSON, binary...

### `logix_vhdl`

Library crate that allows to obtain the
[VHDL](https://en.wikipedia.org/wiki/VHDL) code of a component.

The inverse conversion may be implemented as well but taking into account
that the behavior information of the components will be lost. Maybe the
conversion could be made into `logix_lang` (if it allows specifying
behaviors).
