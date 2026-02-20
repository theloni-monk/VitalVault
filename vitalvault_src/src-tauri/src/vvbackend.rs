use derive_more::Debug;
use uuid::Uuid;
use std::sync::Mutex;
use crate::BackendState;

#[derive(Debug)]
enum DBError{
    IntegrityFailure,
    DecryptionFailure
}
#[tauri::command]
async fn launchdb(state: tauri::State<'_, Mutex<BackendState>>, key: &str) -> Result<(), DBError> {

    // use key to mount db with encryption
    // check db integrity
    // if ok add db connection to the app state mutex

    Err(DBError::IntegrityFailure)
}


enum InjestError{
    FileDNE,
    FileIntegrityFailure,
    DecryptFailure,
    ModelFailure,
}
#[tauri::command]
async fn ingest(state: tauri::State<'_, Mutex<BackendState>>, fname: &str) -> Result<Uuid, InjestError> {
    // load authkey from state mutex
    // query filename -> fail if not exists
    // gen uuid
    // create db entry via appstate db reference
    // decrypt content -> fail if invalid authkey
    // dispatch content to ML Pipeline -> fail if ml pipeline error
    // push semantic vector to db via connection reference in appstate
    // delete the file if it is in the vitalvault unencrypted temp store
    // return success
    Err(InjestError::FileDNE)
}

enum QueryError{
    TokenizationFailure
}
#[tauri::command]
async fn query_vault(state: tauri::State<'_, Mutex<BackendState>>, query_text: &str) -> Result<Vec<usize>, QueryError> {
    Err(QueryError::TokenizationFailure)
}