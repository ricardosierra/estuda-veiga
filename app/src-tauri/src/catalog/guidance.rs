use std::{fs::OpenOptions, io::Write, path::Path};

const AGENTS_GUIDANCE: &str = "# Instruções do acervo Faculdade\n\n- Preserve os arquivos acadêmicos originais; não os sobrescreva nem os mova durante a catalogação.\n- Trate notas, frequência, documentos e credenciais como dados privados locais. Nunca inclua senhas, cookies ou tokens em Markdown, JSON, logs ou Git.\n- O Estuda UVA usa Rust/Tauri para ler o acervo e mantém o conteúdo acessível sem login no Canvas.\n- Ao editar um registro acadêmico, mantenha as representações SQLite, Markdown e JSON sincronizadas e preserve conflitos para revisão humana.\n- Diferencie dados confirmados por fonte oficial de estimativas, material didático e campos desconhecidos. Registre fonte e data para notas, frequência e progresso.\n- Execute verificações focadas antes de concluir mudanças e relate claramente o que foi e o que não foi validado.\n";

const CLAUDE_GUIDANCE: &str = "# Guia do projeto para Claude\n\nLeia `AGENTS.md` antes de editar o acervo. Preserve originais e dados privados, mantenha SQLite, Markdown e JSON coerentes, e nunca invente dados acadêmicos ausentes.\n";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GuidanceCreation {
    pub agents_created: bool,
    pub claude_created: bool,
}

pub fn create_missing_guidance(root: &Path) -> Result<GuidanceCreation, String> {
    if !root.is_dir() {
        return Err("A pasta do acervo não existe ou não é uma pasta.".into());
    }

    Ok(GuidanceCreation {
        agents_created: create_if_missing(&root.join("AGENTS.md"), AGENTS_GUIDANCE)?,
        claude_created: create_if_missing(&root.join("CLAUDE.md"), CLAUDE_GUIDANCE)?,
    })
}

fn create_if_missing(path: &Path, contents: &str) -> Result<bool, String> {
    let mut file = match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => return Ok(false),
        Err(error) => {
            return Err(format!(
                "Não foi possível criar {}: {error}",
                path.display()
            ));
        }
    };
    if let Err(error) = file
        .write_all(contents.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(format!(
            "Não foi possível gravar {}: {error}",
            path.display()
        ));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::create_missing_guidance;
    use std::fs;

    #[test]
    fn creates_both_guidance_files_when_missing() {
        let root = tempfile::tempdir().unwrap();

        let report = create_missing_guidance(root.path()).unwrap();

        assert!(report.agents_created);
        assert!(report.claude_created);
        assert!(
            fs::read_to_string(root.path().join("AGENTS.md"))
                .unwrap()
                .contains("SQLite, Markdown e JSON")
        );
        assert!(
            fs::read_to_string(root.path().join("CLAUDE.md"))
                .unwrap()
                .contains("Leia `AGENTS.md`")
        );
    }

    #[test]
    fn leaves_each_existing_guidance_file_untouched() {
        let root = tempfile::tempdir().unwrap();
        let agents = root.path().join("AGENTS.md");
        fs::write(&agents, "instrução pessoal original\n").unwrap();

        let report = create_missing_guidance(root.path()).unwrap();

        assert!(!report.agents_created);
        assert!(report.claude_created);
        assert_eq!(
            fs::read(agents).unwrap(),
            "instrução pessoal original\n".as_bytes()
        );
    }

    #[cfg(unix)]
    #[test]
    fn does_not_follow_or_replace_existing_guidance_symlinks() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("private-target.txt");
        let agents = root.path().join("AGENTS.md");
        fs::write(&target, "fora do arquivo de instruções").unwrap();
        symlink(&target, &agents).unwrap();

        let report = create_missing_guidance(root.path()).unwrap();

        assert!(!report.agents_created);
        assert_eq!(
            fs::read_to_string(target).unwrap(),
            "fora do arquivo de instruções"
        );
        assert!(
            fs::symlink_metadata(agents)
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }
}
