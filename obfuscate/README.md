# serpentine
Uses various macros to generate all of the inner-workings of a Finite State Machine at compile time, so everything is already set up by the time `new()` is called.

Uses a DSL to outline the transitions which are available to make from any given state.
