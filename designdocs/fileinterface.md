Unified file interface
---
author: Thelonious Cooper<theloni@berkeley.edu>

## Goals

Why write yet another interface instead of just using [skim](https://crates.io/crates/skim-common) or [tantivy](https://github.com/quickwit-oss/tantivy)?

We have different needs of the library. [skim](https://crates.io/crates/skim-common) will be used for its fuzzy matching ability that allows those who are more likely to commit typos not challenged by the interface. Skim doesn't come with a mobile file interface adapter, so thats what we will do. Meanwhile, [tantivy](https://github.com/quickwit-oss/tantivy) is blazingly fast, but also with no simple mobile IO adapter, withinwhich it will likely loose its edge due to SIMD tricks. Lance-DB has its own full-text-search feature, but unfortunately this seems to be more difficult to integrate with a cryptographic scheme that would require 

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


### Challenges
Mobile devices can arbitrarily halt and deallocate a process without warning. For in-memory databases, this means all data will be lost. 

As such, we must persist the DB to disk in an encrypted [parquet file](https://parquet.apache.org/docs/file-format/data-pages/encryption/#53-protection-of-sensitive-metadata)

### Sources
```src-tauri/src/docstore```

### Tests
...tbd
testing plan: main enemy is persisting through unexpected process halts

### Security
Apache Parquet db file format is chosen for its modular encryption paradigm.

Potential sources of secret leakage include
 - in memory buffer overread(unlikely)
 - disk permission manipulation(sideloaded apps can get file read access)
 - 



