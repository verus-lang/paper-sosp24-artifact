The artifact refers to or pulls external repositories which contain baselines and 
open source versions of Verus and the Verus case studies. We use external repositories
as those facilitate reuse by being kept up-to-date with the current version of Verus.
This file lists the commit "refspecs" (SHAs) for all the external repositories in use.

### Verus: https://github.com/verus-lang/verus

The verification system presented in the paper.

Commit: `097ac7ed283ae60375cd9b2b6017b3c629883b2b`

## Case Studies

### Verified Memory Allocator: https://github.com/verus-lang/verified-memory-allocator

A mimalloc-like allocator verified in Verus and scripts for comparison with the unverified mimalloc baseline.

Commit: `6ee4b4fc8ac107f10d3ad420a2c42e26e3033ba7`

### Verified Node Replication: https://github.com/verus-lang/verified-node-replication

A data structure replication library verified in Verus and scripts for comparison with a version written in Linear Dafny.

Commit: `341be41a31cfc5c7539f8b78a65f166a06251d02`

### Verified IronKV implementation: https://github.com/verus-lang/verified-ironkv

A Verus version of the implementation of the IronKV distributed hash table.

Commit: `4d6efdfd47f84b7e29a765c7c92713ff646739e4`

### Verified Persistent Memory log: https://github.com/microsoft/verified-storage 

A log for persistent memory verified in Verus.

Commit: `31b2256b06413c71245baf4b2bec9cea5b20e51b`

### Verified Page Table: https://github.com/utaal/verified-nrkernel

A verified page table implementation in Verus.

Commit: `f361c7a65a7b175a0ebb1ddb518eec11d12143ef`
