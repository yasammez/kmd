# Bindings to write windows kernel mode drivers

## What is this

This is basically a fork of [winapi-kmd-rs](https://github.com/pravic/winapi-kmd-rs), with some additional bindings
because I needed them and a bit of sugar (using the `?` operator on NTSTATUS and iterating over devices) because I
wanted them.

## Why not use bindgen on the WDK instead?

That would probably be the smarter thing to do. Maybe at some time in the future I will try it out.
