use std::Cursor;
use std::sync::Mutex;
use Uuid::Uuid;

enum JobType{
    LinearizePDF, // PDF text extraction
    SemanticEmbed, // minish potion multlingual mebdding
    ImgOCR // OCR
}

// Task that is written to disk on creation, cleared on completion
// goal is to be able to resume(or at least restart) incomplete jobs on boot
struct PersistantTask {
    Uuid doc_id,
    JobType jobtype,
    Mutex<bool> needs_data,
    Mutex<bool> in_flight
}

struct TaskQueue {} // alias for priorityq of mltasks
struct Backend {} // replace with burn backend
struct MLModel {
    bool loaded = false,
    Cursor parambuff
}

struct MLEngine {
    Backend backend, // runtime reference
    MLModel pdfmodel,
    MLModel imgmodel,
    MLModel txtmodel,
    TaskQueue jobs
}

Impl MLEngine {
    fn new(&mut self) -> Result<MLEngine>{

    }
}