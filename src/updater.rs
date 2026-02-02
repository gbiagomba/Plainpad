use velopack::{sources::HttpSource, UpdateCheck, UpdateManager};

pub enum UpdateStatus {
    NoUpdate,
    Available(String),
    Error(String),
}

pub fn check_for_updates() -> UpdateStatus {
    let url = "https://github.com/gbiagomba/Plainpad/releases/latest/download";
    let source = HttpSource::new(url);

    let um = match UpdateManager::new(source, None, None) {
        Ok(m) => m,
        Err(e) => return UpdateStatus::Error(e.to_string()),
    };

    match um.check_for_updates() {
        Ok(UpdateCheck::UpdateAvailable(info)) => {
            UpdateStatus::Available(info.TargetFullRelease.Version.clone())
        }
        Ok(UpdateCheck::NoUpdateAvailable) | Ok(UpdateCheck::RemoteIsEmpty) => {
            UpdateStatus::NoUpdate
        }
        Err(e) => UpdateStatus::Error(e.to_string()),
    }
}

pub fn download_and_apply() -> Result<(), String> {
    let url = "https://github.com/gbiagomba/Plainpad/releases/latest/download";
    let source = HttpSource::new(url);
    let um = UpdateManager::new(source, None, None).map_err(|e| e.to_string())?;

    if let UpdateCheck::UpdateAvailable(info) = um.check_for_updates().map_err(|e| e.to_string())? {
        um.download_updates(&info, None)
            .map_err(|e| e.to_string())?;
        um.apply_updates_and_restart(&info)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
