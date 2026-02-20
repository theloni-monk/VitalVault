use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;
use std::vec::Vec;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use std::fs::File;
use std::hash::{DefaultHasher, Hash};
use std::io::{BufRead, Cursor};

use derive_more::Debug;

type ModelID = Uuid;
const SEMANTIC_HASH_DIMENSION: usize = 256;
const SEMANTIC_HASH_BYTES: usize = SEMANTIC_HASH_DIMENSION * 4;
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticHash {
    #[serde_as(as = "Bytes")]
    vector: [u8; SEMANTIC_HASH_BYTES],
    hashed_by: ModelID,
}

// faroff writeme : hierarchical semantic hashing / page-wise sem search
pub trait SemanticHashable<'a, T: Serialize + Deserialize<'a> + std::fmt::Debug> {
    fn hash_data(&self) -> Rc<SemanticHash>;

    fn metricdistance(&self, other: T) -> f32;

    fn nonzero(&self) -> bool;

    fn equals(&self, other: T) -> bool;

    fn hashed_by(&self) -> ModelID;
}

#[derive(Debug, Serialize, Deserialize)]
enum StorableType {
    TXT,
    IMG,
    PDF,
}
#[derive(Debug, Serialize, Deserialize)]
struct DocMeta<T> {
    // additional metadata about a document relevant to integrity checks and user search
    user_pinned: bool,

    checksum: u32, // computed on ingest

    keywords: Option<Vec<T>>, // key terms for fuzzy matching

    user_categories: Option<Vec<T>>, // user-attached labels

    semhash: Option<SemanticHash>, // semantic vector used for multi-language search
}

impl<T> DocMeta<T> {
    fn new(
        user_pinned: bool,
        checksum: u32,
        keywords: Option<Vec<T>>,
        user_categories: Option<Vec<T>>,
        semhash: Option<SemanticHash>,
    ) -> Self {
        Self {
            user_pinned,
            checksum,
            keywords,
            user_categories,
            semhash,
        }
    }

    fn completed(&self) -> bool {
        self.keywords.is_some() && self.user_categories.is_some() && self.semhash.is_some()
    }
}

pub type DocID = Uuid;
#[derive(Debug, Serialize, Deserialize)]
struct DocSpec<'a> {
    // sufficient information to retrieve a document
    title: &'a str,      // name as seen by user: not unique
    id: DocID,           // key for local database, also used as a salt for the encryption
    ftype: StorableType, // for icon rendering
}

#[derive(Debug)]
struct Doc<'a> {
    // wrapper for data
    spec: DocSpec<'a>,             // part of struct that crosses frontend/backend
    meta: Option<DocMeta<String>>, // additional file data, possibly added after ingestion
    fpath: &'a Path,               // where to find base data, must be unique
    data_buffer: Option<Cursor<File>>, // in memory buffer for file data. Option used so we don't hold too many in mem at a time
}

#[derive(Debug)]
enum DocError {
    FileDNE,
    BadExtension,
}

impl Doc<'_> {
    fn new(&mut self, spec: &DocSpec, fpath: &Path, lazy_load: bool) -> Result<Doc<'_>, DocError> {
        // check if fpath exists: fail otherwise
        // assign storabletype based on file extension

        // gen uuid
        // load into data_buffer if do_load
        // find keywords if it is a txt
        // if(self.ftype != IMG) self.find_keywords();
        Err(DocError::FileDNE)
    }

    //WRITEME: documents
    // fn writeback(&self, &DBConn)

    fn injestion_complete(&self) -> bool {
        self.meta.as_ref().is_some_and(|meta| meta.completed())
    }

    fn is_loaded(&self) -> bool {
        self.data_buffer.is_some()
    }

    // #[allow(dead_code)]
    // fn fetch(&mut self, key: usize ) -> Result<Cursor>{
    //     //WRITEME: load into memory file decrypted via provided key
    // }

    #[allow(dead_code)]
    fn rename(&mut self, new_title: &str) {}

    fn clear_buff(&mut self) {
        // drop the in memory buffer
        // drop(self.data_buffer);
    }

    // #[allow(dead_code)]
    // fn find_keywords(&mut self) -> Result<usize>{
    //     // scrape for uncommon words and put them in list
    // }
}

pub type DocScope = HashSet<Uuid>;

#[derive(Debug)]
enum ScopeError {
    DisjointIntersect,
    OverlappingQuotient,
}

fn intersect(self_: &mut DocScope, other: DocScope) -> Result<(), ScopeError> {
    // fails if disjoint
    Err(ScopeError::DisjointIntersect)
}

fn quotient(self_: &mut DocScope, other: DocScope) -> Result<(), ScopeError> {
    // fails if identical
    Err(ScopeError::OverlappingQuotient)
}

fn union(self_: &mut DocScope, other: &DocScope) {
    // doesn't fail
}
