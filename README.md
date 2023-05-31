# StaMESP

A State Machine Emulator for String Processing.

![build-badge](https://img.shields.io/github/actions/workflow/status/Jerem-dY/StaMESP/rust.yml)
[![GitHub](https://img.shields.io/github/license/Jerem-dY/StaMESP)](https://github.com/Jerem-dY/StaMESP/blob/main/LICENSE)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/Jerem-dY/StaMESP?include_prereleases)](https://github.com/Jerem-dY/StaMESP/releases)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2FJB09SI)](https://twitter.com/JB09SI)


- GitHub:   https://github.com/Jerem-dY/StaMESP
- Crates.io: https://crates.io/crates/stamesp
- Doc:      https://docs.rs/matx/latest/stamesp/index.html

## TODO
- [x] Scanner
- [ ] Parser
- [ ] Interpreter
- [ ] Output tree-like structures (like XML)

## The 4 Constituents
The language itself relies on 4 elements:
- SETS, that are a list of litterals which can constitude a shared premise to a transition function.
- sets are made of LITTERALS, which are strings specified between inverted commas `<">` or single quotes `<'>`. 
- STATES describe both the string output of a transition (the name of the state) and the different transitions available from that point on. 
- EXPRESSIONS are found in states and link a premise (a set) to a conclusion (a state) in a particular state.

## Nodes

The system relies on a tree system, with each new state being a node that has the token(s) that lead to it as children. Such nodes can then be exported to XML.

## Syntax

### Basics

SETS and LITTERALS are described as follows:  

`setname('this' 'is' 'a' 'test')`

While commas or other separators are technically viable to separate the different litterals, it is recommended to go with the space character. A comma or a line break are required after the closing of parentheses to end the set.

STATES and EXPRESSIONS must be written this way:

```
state_one{
    setname = state_two
    another_set = state_three
}
``` 


### Start and End

To be usable, a system described by those rules must have at most (and at least) one ENTRY POINT and at least one END POINT. An asterix `<*>` and/or a colon `<:>` can be put after the state's identifier to make it an entry or end point, respectively. 

The ENTRY POINT is the state which the simulation will start in, and END POINTS are the states which the system can stop within without throwing an error. This allows for recognition (automata), as the error thrown can be caught and treated accordingly. Entry point can be chosen when starting the interpreter, as long as it exists in the system.

### Modifiers

States can have several MODIFIERS in expressions.

- a hat `<^>` means we do not want to write a new node, only changing the state internally. Beware, as it can lead to unforeseen behavior!

- a percentage `<%>` means we do want to write, but *before* we get to the new state. This is particularly useful for suffixes, as they should both be added to the current token **and** signal the end of it.

### Special elements

- `<@>` is a special set that describes any token that does not belong to the different sets specified in the expressions. It is local, meaning that it does not have the same value across states: it is a way to describe potentially infinite tokens, thus acting like a `default` in a switch-case statement.

- `<.>` is a special state that signals we want to stay in the current state, ***without creating a new node***. This allows for **concatenation**. If one wants to create a new node while staying in the same state, all they have to do is write the state's name.

