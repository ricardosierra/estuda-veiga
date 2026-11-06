use serde::Serialize;
use std::{path::Path, process::Command};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryStatus {
    pub is_repository: bool,
    pub branch: Option<String>,
    pub tracked_changes: Vec<String>,
    pub untracked_files: Vec<String>,
    pub ignored_files: Vec<String>,
    pub error: Option<String>,
}

pub fn repository_status(root: &Path) -> Result<RepositoryStatus, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar o repositório: {error}"))?;
    if !root.is_dir() {
        return Err("A pasta configurada para o acervo não é uma pasta.".into());
    }
    let root_argument = root.to_string_lossy().into_owned();
    let output = match Command::new("git")
        .args([
            "-C",
            &root_argument,
            "status",
            "--short",
            "--untracked-files=normal",
            "-z",
            "--ignored=matching",
        ])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            return Ok(RepositoryStatus {
                error: Some(format!("Git não está disponível: {error}")),
                ..RepositoryStatus::default()
            });
        }
    };
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Ok(RepositoryStatus {
            error: Some(if message.is_empty() {
                "Não foi possível ler o status do Git.".into()
            } else {
                message
            }),
            ..RepositoryStatus::default()
        });
    }

    let branch = Command::new("git")
        .args(["-C", &root_argument, "branch", "--show-current"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|branch| !branch.is_empty());
    let (tracked_changes, untracked_files, ignored_files) = parse_porcelain_z(&output.stdout);
    Ok(RepositoryStatus {
        is_repository: true,
        branch,
        tracked_changes,
        untracked_files,
        ignored_files,
        error: None,
    })
}

fn parse_porcelain_z(bytes: &[u8]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut tracked = Vec::new();
    let mut untracked = Vec::new();
    let mut ignored = Vec::new();
    let mut entries = bytes
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty());
    while let Some(entry) = entries.next() {
        if entry.len() < 4 {
            continue;
        }
        let status = &entry[..2];
        let path = String::from_utf8_lossy(&entry[3..]).into_owned();
        if status == b"!!" {
            ignored.push(path);
        } else if status == b"??" {
            untracked.push(path);
        } else {
            tracked.push(path);
            if status.contains(&b'R') || status.contains(&b'C') {
                if let Some(previous_path) = entries.next() {
                    tracked.push(String::from_utf8_lossy(previous_path).into_owned());
                }
            }
        }
    }
    (tracked, untracked, ignored)
}

#[cfg(test)]
mod tests {
    use super::repository_status;
    use std::{fs, process::Command};

    fn git(root: &std::path::Path, args: &[&str]) {
        let status = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success());
    }

    #[test]
    fn reports_tracked_untracked_and_ignored_files_without_staging_them() {
        let root = tempfile::tempdir().unwrap();
        git(root.path(), &["init", "-q"]);
        fs::write(root.path().join(".gitignore"), ".estuda-uva.sqlite3\n").unwrap();
        fs::write(root.path().join("materia.md"), "inicial\n").unwrap();
        git(root.path(), &["add", ".gitignore", "materia.md"]);
        fs::write(root.path().join("materia.md"), "alterada\n").unwrap();
        fs::write(root.path().join("nova.md"), "nova\n").unwrap();
        fs::write(root.path().join(".estuda-uva.sqlite3"), "dados locais").unwrap();

        let status = repository_status(root.path()).unwrap();

        assert!(status.is_repository);
        assert!(status.error.is_none());
        assert!(
            status
                .tracked_changes
                .iter()
                .any(|path| path == "materia.md")
        );
        assert!(status.untracked_files.iter().any(|path| path == "nova.md"));
        assert!(
            status
                .ignored_files
                .iter()
                .any(|path| path == ".estuda-uva.sqlite3")
        );
        assert!(
            !status
                .untracked_files
                .iter()
                .any(|path| path == "materia.md")
        );
    }

    #[test]
    fn reports_a_regular_folder_as_not_a_repository() {
        let root = tempfile::tempdir().unwrap();

        let status = repository_status(root.path()).unwrap();

        assert!(!status.is_repository);
        assert!(status.error.is_some());
    }
}
