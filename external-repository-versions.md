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

## Case Study Baselines

### Verified Node Replication in Linear Dafny (part of IronSync): https://github.com/secure-foundations/ironsync-osdi2023

Baseline for Node Replication.

Commit: `7c912e29fd9e770d2fb9866606d0bf2a97629252`

### Verified IronKV in Dafny (part of IronFleet): https://github.com/microsoft/Ironclad.git 

Baseline for IronKV.

Commit: `2fe4dcdc323b92e93f759cc3e373521366b7f691`

#### Depends on a different version of Verus:  https://github.com/verus-lang/verus

Commit: `96957b633471e4d5a6bc267f9bf0e31555e888db`

## Other Verifiers

### Creusot: https://github.com/creusot-rs/creusot

Commit: `9203a5975184ba6be5a0d0b47ef3adc3029e0dda`

#### Depends on Why3: https://gitlab.inria.fr/why3/why3

Commit: `c51c244ded49abe332635a126f381aedb1c67715`

### Prusti: https://github.com/viperproject/prusti-dev

Commit: `a5c29c994cee03e1ba02c3bc2c2761803571d3f5`

### Fstar (Low*): https://github.com/FStarLang/FStar

Commit: `1d823c247b578280cd05a7f416f813589334c569`

#### With the KaRaMeL library: https://github.com/FStarLang/karamel

Commit: `5c7ac22a85fb0b9ce8c278084665022bf7dbb3f7`

### Dafny: https://github.com/dafny-lang/dafny

Binary release: Dafny 4.3.0 -- https://github.com/dafny-lang/dafny/releases/tag/v4.3.0
