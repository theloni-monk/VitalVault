use Uuid::Uuid;

use tauri_plugin_biometric::{BiometricExt, AuthOptions};

fn mobile_bio_auth(app_handle: tauri::AppHandle) {

    let options = AuthOptions {
        // Set True if you want the user to be able to authenticate using phone password
        allow_device_credential:false,
        cancel_title: Some("Vault cannot be accessed if authentication is canceled".to_string()),

        // iOS only feature
        fallback_title: Some("Sorry, authentication failed".to_string()),

        // Android only features
        title: Some("Intialize Vault".to_string()),
        subtitle: Some("Authenticate to access the your Vital Vault".to_string()),
        confirmation_required: Some(true),
    };

    // if the authentication was successful, the function returns Result::Ok()
    // otherwise returns Result::Error()
    match app_handle.biometric().authenticate("The Vault is locked".to_string(), options) {
        Ok(_) => {
            println!("Successfully Authenticated!");
        }
        Err(e) => {
            println!("Authentication failed because : {e}");
        }
    }
}

#[tauri::command]
async fn launchdb(state: tauri::State<'_, Mutex<AppState>>) -> Result<> {
    // if on mobile use bioauth handle
    // if on macos or windows request password
    // init db handle
    Ok()
}

#[tauri::command]
async fn validatestore(state:tauri::State<'_, Mutex<AppState>>) -> Result<bool> {
    Ok()
}

#[tauri::command]
async fn ingest(state: tauri::State<'_, Mutex<AppState>>, fname: &str) -> Result<Uuid> {
    // load authkey from state mutex
    // query filename -> fail if not exists
    // gen uuid
    // create db entry via appstate db reference
    // decrypt content -> fail if invalid authkey
    // dispatch content to ML Pipeline -> fail if ml pipeline error
    // push semantic vector to db via connection reference in appstate
    // return success
    Ok()
}
