# mtx-slipstream

High-performance, minimalist Ruma alternative.

Currently used by my Congruent (and to a lesser extent, Sithnapse) forks.

## JSON

Slipstream exposes Rezzy's JSON value and parser through `mtx_slipstream::json`.
Its internal `slipstream_json` adapter keeps the owned-value and writer
interfaces used by the sync and federation paths. The underlying JSON
implementation is `rezzy-json`, brought in through the `rezzy` dependency;
Rezzy also provides the canonicalization and signing support used by Slipstream.
