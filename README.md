# faf-allocator-bench

A Rust allocator benchmarking tool.

## Results

### SnMalloc (snmalloc-rs v0.3.4 (snmalloc-sys v0.3.4))

```
SnMalloc          2 bytes   268435455 allocs        4 ns per alloc
SnMalloc          4 bytes   268435455 allocs        4 ns per alloc
SnMalloc          8 bytes   268435455 allocs        4 ns per alloc
SnMalloc         16 bytes   268435455 allocs        4 ns per alloc
SnMalloc         32 bytes   268435455 allocs        4 ns per alloc
SnMalloc         64 bytes   268435455 allocs        4 ns per alloc
SnMalloc        128 bytes   268435455 allocs        5 ns per alloc
SnMalloc        256 bytes   268435455 allocs        5 ns per alloc
SnMalloc        512 bytes   268435455 allocs        5 ns per alloc
SnMalloc       1024 bytes   268435455 allocs        5 ns per alloc
SnMalloc       2048 bytes   268435455 allocs        6 ns per alloc
SnMalloc       4096 bytes   268435455 allocs        7 ns per alloc
SnMalloc      32768 bytes   268435455 allocs        7 ns per alloc
SnMalloc      65536 bytes   268435455 allocs      290 ns per alloc
SnMalloc     131072 bytes   268435455 allocs      295 ns per alloc
SnMalloc     262144 bytes   268435455 allocs      448 ns per alloc
SnMalloc     524288 bytes   268435455 allocs      338 ns per alloc
SnMalloc    1048576 bytes   268435455 allocs      391 ns per alloc
```

### TCMalloc (tcmalloc v0.3.0)

```
TCMalloc          2 bytes   268435455 allocs        6 ns per alloc
TCMalloc          4 bytes   268435455 allocs        6 ns per alloc
TCMalloc          8 bytes   268435455 allocs        6 ns per alloc
TCMalloc         16 bytes   268435455 allocs        6 ns per alloc
TCMalloc         32 bytes   268435455 allocs        6 ns per alloc
TCMalloc         64 bytes   268435455 allocs        6 ns per alloc
TCMalloc        128 bytes   268435455 allocs        6 ns per alloc
TCMalloc        256 bytes   268435455 allocs        6 ns per alloc
TCMalloc        512 bytes   268435455 allocs        6 ns per alloc
TCMalloc       1024 bytes   268435455 allocs        6 ns per alloc
TCMalloc       2048 bytes   268435455 allocs        6 ns per alloc
TCMalloc       4096 bytes   268435455 allocs        6 ns per alloc
TCMalloc      32768 bytes   268435455 allocs        6 ns per alloc
TCMalloc      65536 bytes   268435455 allocs        6 ns per alloc
TCMalloc     131072 bytes   268435455 allocs        6 ns per alloc
TCMalloc     262144 bytes   268435455 allocs        6 ns per alloc
TCMalloc     524288 bytes   268435455 allocs       46 ns per alloc
TCMalloc    1048576 bytes   268435455 allocs       46 ns per alloc
```

### Jemalloc (tikv-jemallocator v0.5.4 (tikv-jemalloc-sys v0.5.4+5.3.0-patched))

```
Jemalloc          2 bytes   268435455 allocs        4 ns per alloc
Jemalloc          4 bytes   268435455 allocs        4 ns per alloc
Jemalloc          8 bytes   268435455 allocs        4 ns per alloc
Jemalloc         16 bytes   268435455 allocs        4 ns per alloc
Jemalloc         32 bytes   268435455 allocs        4 ns per alloc
Jemalloc         64 bytes   268435455 allocs        4 ns per alloc
Jemalloc        128 bytes   268435455 allocs        5 ns per alloc
Jemalloc        256 bytes   268435455 allocs        5 ns per alloc
Jemalloc        512 bytes   268435455 allocs        5 ns per alloc
Jemalloc       1024 bytes   268435455 allocs        5 ns per alloc
Jemalloc       2048 bytes   268435455 allocs        6 ns per alloc
Jemalloc       4096 bytes   268435455 allocs        7 ns per alloc
Jemalloc      32768 bytes   268435455 allocs       32 ns per alloc
Jemalloc      65536 bytes   268435455 allocs      210 ns per alloc
Jemalloc     131072 bytes   268435455 allocs      209 ns per alloc
Jemalloc     262144 bytes   268435455 allocs      208 ns per alloc
Jemalloc     524288 bytes   268435455 allocs      209 ns per alloc
Jemalloc    1048576 bytes   268435455 allocs      209 ns per alloc
```

### MiMalloc (mimalloc v0.1.39 (libmimalloc-sys v0.1.35))

```
MiMalloc          2 bytes   268435455 allocs        4 ns per alloc
MiMalloc          4 bytes   268435455 allocs        4 ns per alloc
MiMalloc          8 bytes   268435455 allocs        4 ns per alloc
MiMalloc         16 bytes   268435455 allocs        8 ns per alloc
MiMalloc         32 bytes   268435455 allocs        9 ns per alloc
MiMalloc         64 bytes   268435455 allocs        8 ns per alloc
MiMalloc        128 bytes   268435455 allocs       10 ns per alloc
MiMalloc        256 bytes   268435455 allocs       12 ns per alloc
MiMalloc        512 bytes   268435455 allocs       11 ns per alloc
MiMalloc       1024 bytes   268435455 allocs        7 ns per alloc
MiMalloc       2048 bytes   268435455 allocs       23 ns per alloc
MiMalloc       4096 bytes   268435455 allocs       23 ns per alloc
MiMalloc      32768 bytes   268435455 allocs       23 ns per alloc
MiMalloc      65536 bytes   268435455 allocs       23 ns per alloc
MiMalloc     131072 bytes   268435455 allocs       23 ns per alloc
MiMalloc     262144 bytes   268435455 allocs      192 ns per alloc
MiMalloc     524288 bytes   268435455 allocs      190 ns per alloc
MiMalloc    1048576 bytes   268435455 allocs      192 ns per alloc
```
