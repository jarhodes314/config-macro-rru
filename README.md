# tedge config macros - multi-value groups demo

These are a couple of exercises to gain some intuition about the define_tedge_config! macro.
They don't involve touching any macro code, but should give you a decent background into the
changes that I've been making to `define_tedge_config!` to support multiple named c8y instances.

## Design background

I will discuss this in more detail in the Rust ramp-up session, but the design I settled on for
configuring multiple `c8y.url` values was to allow the `c8y` field to either be a normal group
of configurations (like it is now) or a map from arbitrary strings to a group of c8y configurations.

For instance, if you wanted to migrate from connecting using the following configuration:

```toml
[c8y]
url = "https://example.com"
```

...to a (thick) edge instance as well, you would amend the config to be.

```toml
[c8y.cloud]
url = "https://example.com"
bridge.topic_prefix = "c8y-cloud"

[c8y.edge]
url = "https://my-edge-instance.local"
bridge.topic_prefix = "c8y-edge"
```

You can either use the named-`c8y`s, or the single `c8y` with a `url` directly underneath.
You can't use both simultaneously.

## Exercises
The exercises are numbered. Each `lib.rs` file has a doc comment with some background.
`multi.rs` in the second exercise will spoil the first exercise, so avoid looking at
that until you're done with the first exercise.

Both exercises have some tests. These are designed to err on the side of compiling so
you can get useful feedback about what does and doesn't work. Occasionally there are
`todo!`s inside the tests that will need replacing, but the tests _should_ work other
than that without any changes. That said, I've not fully tested them, so there may be
a bug somewhere.