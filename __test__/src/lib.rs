pub mod async_std;

pub mod tokio;

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::PathBuf};

    use get_dir::{get_project_root, GetDir, Target};

    #[test]
    fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target {
                name: "src".to_string(),
                r#type: get_dir::TargetType::Dir,
            }])
            .get()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target {
                name: "Cargo.toml".to_string(),
                r#type: get_dir::TargetType::File,
            }])
            .get()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target {
                name: "target".to_string(),
                r#type: get_dir::TargetType::Dir,
            }])
            .get_reverse()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target {
                name: "LICENSE".to_string(),
                r#type: get_dir::TargetType::File,
            }])
            .get_reverse()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_project_root() {
        let dir: PathBuf = get_project_root();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }
}
