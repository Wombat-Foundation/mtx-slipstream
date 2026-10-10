# Endpoint authentication tables

`src/endpoint_auth.rs` (the table `Metadata::new` consults) and
`src/endpoint_auth_spec.rs` (test-only, for the cross-check) are generated.

- `gen_ruma_table.py` reads `authentication:` out of each `metadata!` block of the
  pinned ruwuma checkout (extracted to `ruwuma_rev/crates/`); this is the contract
  Continuwuity ran under, so it is what `endpoint_auth.rs` declares.
- `gen_spec_table.py` reads `security:` from matrix-spec `data/api/` and applies
  `spec_overrides.json`. Any endpoint with no `security` block or an unmappable scheme must
  be resolved there (scheme plus reason) or excluded; otherwise the script exits non-zero
  and writes nothing. Regenerating `src/endpoint_auth_spec.rs` is therefore reproducible
  (apart from rustfmt wrapping). Pass `--root <matrix-spec>/data/api`.

Decisions that differ from the spec, or where the spec is silent, are recorded with
their winning source in `endpoint.rs` (`SPEC_DIFFERENCES`, `REVIEWED_NO_SECURITY`).
`gen_ruma_table.py` still expects `ruwuma_rev/crates/` in the current directory.
