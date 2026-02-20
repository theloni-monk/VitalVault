use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::{io::Cursor, str::Bytes};
use uuid::Uuid;
use async_trait::async_trait;

use crate::docstruct::{DocID, SemanticHash};

#[derive(Debug, Serialize, Deserialize)]
enum JobType {
    LinearizePDF,  // PDF text extraction
    SemanticEmbed, // minish potion multlingual embedding
    ImgOCR         // OCR
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskContext {}

// TODO: task necromancy
type TaskID = Uuid;
// goal is to be able to resume(or at least restart) incomplete jobs on boot
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    target_id: DocID,                   // document to exectute on
    task_id: TaskID,                    // for necromancy
    priority: u8,

    jobtype: JobType,
    resumable: bool,
    interrupted: bool,

    launchtime: std::time::SystemTime,  //TODO: handle date-time w chronos
    ctx: TaskContext,                   //ModelContext allows for resuming
}

#[derive(Debug, Serialize, Deserialize)]
enum TaskError {
    TargetIOFailure,
    ModelFailure,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskUpdate {
    task_id: TaskID,
    killed: bool,
    ctx: Result<TaskContext, TaskError>,
}

// when you build a taskqueue, it will populate with persisted jobs from disk
struct TaskQueue {} // alias for priorityq of mltasks

// far-off todo make a macro to derive this
// I want to wrap every function call that takes a &mut self on some object with a given persistence hook
trait DiskPersistant<T> {} // trait for a struct that is written to disk upon update or creation

struct Backend {} // replace with burn backend

#[derive(Debug, Serialize, Deserialize)]
enum ModelError {
    BadInput,
    BadParams,
    InvalidCTX,
}

#[async_trait]
trait AIModel<I, O> {
    async fn load(&mut self, param_buff: &BufReader<Bytes>) -> bool;
    async fn process(&self, data_in: I) -> Result<O, ModelError>;
}

struct MLEngine<'a> {
    backend: Backend, // owned runtime
    pdfocr: &'a dyn AIModel<File, String>,
    imgmodel: &'a dyn AIModel<File, crate::docstruct::SemanticHash>,
    txtmodel: &'a dyn AIModel<String, crate::docstruct::SemanticHash>,
    jobs: TaskQueue,
}

impl MLEngine<'_> {
    // TODO: scheme for model serialization and loading from disk
    fn new(&mut self, lazy_models: bool) -> Result<MLEngine<'_>, ModelError> {
        Err(ModelError::BadParams)
    }
}