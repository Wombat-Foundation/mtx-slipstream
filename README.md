# mtx-slipstream

High-performance Rust serialization and Matrix client/server API types.

This crate is currently developed as part of the Slipstream workspace.

## `res/` submodule

`res/` is an optional encrypted submodule (GCrypt over SSH) holding private
resources. It is set to `update = none` so normal checkouts never fetch it.
To initialize it you need [`git-remote-gcrypt`](https://github.com/spwhit/git-remote-gcrypt)
and a GCrypt key authorized for the remote:

```sh
git submodule update --init res
```
