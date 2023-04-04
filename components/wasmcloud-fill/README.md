# wasmCloud Fill Component

This component exports the `actor` interface to the host and specifically responds to the Concordance `AggregateService.ApplyEvent` operation ID.

This component in turn invokes `apply_event` on a concordance export.

To build:

```console
cargo build
```

