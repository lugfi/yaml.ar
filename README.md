# yaml.yaml

A lambda calculus syntax for YAML

Aside from the mapping from YAML to lambda calculus itself,
you'll find modules that I implemented to test the expresiveness of the language

## risp-yaml

This module implements lisp on top of the YAML syntax.

### REPL

```bash
$ cd interpreter.rust
$ cargo run -q
risp-yaml >
a: [+, 40, a]

🔥 RispFunction(...)
risp-yaml >
[a, 2]

🔥 Int(42)
```
