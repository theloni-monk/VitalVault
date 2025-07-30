use uuid::Uuid;
use std::path::Path;
use std::{Vec, Option};
use std::hash::{DefaultHasher, Hash};
use std::fs;
use std::io::Cursor;

mod docstuff{
struct SemanticVector{} // abstract type for now

enum StorableType{
    TXT,
    IMG,
    PDF
}

//#derive[duck db schema type thingy]
// documents get ingested, encrypted and written to disk on js side, then a notif is sent to backend to process
#[derive(Debug)]
struct Doc {
    &str title, // name as seen by user: not unique
    Uuid id, // key for local database, also used as a salt for the encryption
    Path fpath, // where to find base data, must be unique
    StorableType ftype,
    Option<Vec<&str>> keywords, // key terms for fuzzy matching
    Option<SemanticVector> semvec, // semantic vector used for multi-language search
    Option<Cursor> data_buffer, // in memory buffer for file data. Option used so we don't hold too many in mem at a time
    Option<Hash> data_hash // computed on injest
}

impl Doc{
    // might want this async so we can batch spawn and know if it failed
    fn new(&mut self, &str title, Path fpath, bool do_load) -> Result<Doc> {
        // check if fpath exists: fail otherwise
        // assign storabletype based on file extension

        // gen uuid
        // load into data_buffer if do_load
        // find keywords if it is a txt
        if(self.ftype != IMG) self.find_keywords();
    }

    fn is_complete(&self) -> bool {
        self.keywords.is_some() && self.semvec.is_some()
    }

    fn is_loaded(&self) -> bool {
        self.data_buffer.is_some()
    }

    #[allow(dead_code)]
    fn fetch(&mut self, usize key) -> Result<Cursor>{
        //WRITEME: load into memory file decrypted via provided key

    }

    
    fn clear_buff(&mut self){
        // drop the in memory buffer
        self.data_buffer.drop()
    }

    #[allow(dead_code)]
    fn find_keywords(&mut self) -> Result<usize>{
        // scrape for uncommon words and put them in list
    }
}

#[derive(Debug, Clone, Copy)]
struct DocScope { Vec<Uuid> ids }

impl DocScope {
    fn new(&mut self, Vec<&Doc> docs) -> DocScope{
        self.ids = docs.iter().map(|doc| doc.id)
                                .collect<Hashset::<_>>() // de duplication of ids in scope
                                    .into_iter().collect()
    }
    fn append(&mut self, Doc doc){
        self.ids.append(doc.id)
    }

    fn intersect(&self, DocScope other) -> Result<DocScope> {
        // fails if disjoint
        Err()
    }

    fn quotient(&self, DocScope other) -> Result<DocScope> {
        // fails if identical
        Err()
    }

    #[allow(dead_code)]
    fn union(&self, DocScope other) -> DocScope {
        // doesn't fail
    }
}
}