pub mod async_std;

pub mod tokio;

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::PathBuf};

    use get_dir::{get_project_root, DirTarget, FileTarget, GetDir, Target};

    #[test]
    fn test_get_dir_by_target_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget { name: "src".to_string() })])
            .get()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget {
                name: "Cargo.toml".to_string(),
            })])
            .get()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("get_dir = { workspace = true }"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_dir() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::Dir(DirTarget {
                name: "target".to_string(),
            })])
            .get_reverse()
            .unwrap();

        let content: String = read_to_string(dir.join("Cargo.toml")).unwrap();

        assert!(content.contains("[workspace.dependencies]"));
    }

    #[test]
    fn test_get_dir_by_target_reverse_file() {
        let dir: PathBuf = GetDir::new()
            .targets(vec![Target::File(FileTarget {
                name: "LICENSE".to_string(),
            })])
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
