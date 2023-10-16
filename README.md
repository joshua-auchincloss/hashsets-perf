# Rust Collections Benchmarks

Basic CI to test common rust collection types (in particular hash-based). Refer to the latest [benchmarks] for results. Outputs are formatted in:

```
{lib} {procedure test} key type: {} value type: {}
```

Where lib is one of the below libraries in scope, procedure test is work (e.g. `HashMap::get`), and key & value types are the types tested against the hashing algorithm.

In Scope:

- `std`
- [`ahash`]
- [`seahash`]

[`ahash`]: https://github.com/tkaitchuck/aHash
[`seahash`]: https://gitlab.redox-os.org/redox-os/seahash
[benchmarks]: https://joshua-auchincloss.github.io/hashsets-perf
