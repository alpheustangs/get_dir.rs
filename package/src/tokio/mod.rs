use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use tokio::{
    fs::{self, ReadDir},
    io,
};

use crate::{GetDir, Target, TargetType};

async fn target_exists(
    path: &Path,
    target: &Target,
) -> bool {
    match target.r#type {
        | TargetType::Dir => path.is_dir(),
        | TargetType::File => path.is_file(),
    }
}

async fn search_targets(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> Option<PathBuf> {
    for target in targets {
        let target_path: PathBuf = dir.join(&target.name);
        if target_exists(&target_path, target).await {
            return Some(dir.to_owned());
        }
    }

    None
}

async fn search_dir(
    dir: &PathBuf,
    targets: &Vec<Target>,
) -> io::Result<Option<PathBuf>> {
    let mut entries: ReadDir = fs::read_dir(dir).await?;

    if let Some(found) = search_targets(dir, targets).await {
        return Ok(Some(found));
    }

    while let Ok(Some(entry)) = entries.next_entry().await {
        let current: PathBuf = entry.path();

        if current.is_dir() {
            if let Some(found) = search_targets(&current, targets).await {
                return Ok(Some(found));
            }

            if let Ok(Some(found)) =
                Box::pin(search_dir(&current, targets)).await
            {
                return Ok(Some(found));
            }
        }
    }

    Ok(None)
}

/// Trait for getting directory with tokio.
pub trait AsyncGetterExt {
    /// Get directory asynchronously.
    fn get_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;

    /// Get directory in reverse asynchronously.
    fn get_reverse_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<PathBuf>> + Send;
}

impl AsyncGetterExt for GetDir {
    async fn get_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

        match search_dir(&current, &self.targets).await {
            | Ok(Some(path)) => Ok(path),
            | _ => Err(io::Error::from(io::ErrorKind::NotFound)),
        }
    }

    async fn get_reverse_async(&self) -> io::Result<PathBuf> {
        let current: PathBuf = current_dir()?;

        for ancestor in current.ancestors() {
            for target in &self.targets {
                let target_path: PathBuf = ancestor.join(&target.name);
                if target_exists(&target_path, target).await {
                    return Ok(ancestor.to_path_buf());
                }
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root`] to handle the error automatically.
pub async fn get_project_root_directory() -> io::Result<PathBuf> {
    GetDir::new()
        .targets(vec![
            Target { name: "target".to_string(), r#type: TargetType::Dir },
            Target { name: "Cargo.lock".to_string(), r#type: TargetType::File },
        ])
        .get_reverse_async()
        .await
}

/// Get the project root directory by searching for
/// the `target` folder and the `Cargo.lock` file.
/// Use [`get_project_root_directory`] to handle the error manually.
pub async fn get_project_root() -> PathBuf {
    get_project_root_directory().await.expect("Failed to get project root")
}
