Unified file interface
---
author: Thelonious Cooper<theloni@berkeley.edu>

## Goals

Local filestore cryptographic security.
 - choice of password or biometric protection
 - ofuscated local database file

Fast pdf, jpg, png, hcif, webp rendering

Flexible caching if available

Allow for incremental indexing

Cross platform testability on desktop, ios, and android


## Components

#### Apache Arrow config
We will use the arrow subconfig to make sure all writes are done via an [encrypted filewriter](https://arrow.apache.org/docs/cpp/api/formats.html#_CPPv4N7parquet16WriterProperties7BuilderE)

### First boot

we will use expo to create the apache parquet database file in memory via [DuckDB](https://duckdb.org/docs/stable/operations_manual/footprint_of_duckdb/files_created_by_duckdb), this will be encrypted on disk upon write and store a schema with the following
 - pdf path and per-doc secret
 - keywords for [fuzzy finding](https://github.com/heyimalex/bitap)
 - vectors for nn search

Client-side logic: if key exists, use bioauth to retrieve it and then send it to the server to boot
if check fails, generate a random number, load it into the native store, and send to rust to create a db with that key


### Challenges
Mobile devices can arbitrarily halt and deallocate a process without warning. For in-memory databases, this means all data will be lost. 

As such, we must persist the DB to disk in an encrypted [parquet file](https://parquet.apache.org/docs/file-format/data-pages/encryption/#53-protection-of-sensitive-metadata)

DuckDB can do data-at-rest encryption which is perfect for this
https://duckdb.org/2025/11/19/encryption-in-duckdb

### Sources
```src-tauri/src/docstruct```

### Tests
...tbd
testing plan: main challenge is persisting through unexpected process halts, not sure how best to approach this with traditional CI

### Security
Apache Parquet db file format is chosen for its modular encryption paradigm.

Potential sources of secret leakage include
 - in memory buffer overread(unlikely)
 - disk permission manipulation(sideloaded apps can get file read access)



