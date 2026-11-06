//! Núcleo do Estuda UVA: domínio acadêmico, acervo, persistência, sincronização,
//! extração de conteúdo e automação do navegador. Não conhece Tauri nem GPUI.

pub mod anthropic;
pub mod archive;
pub mod browser;
pub mod calendar;
pub mod canvas;
pub mod canvas_api;
pub mod canvas_capture;
pub mod catalog;
pub mod download;
pub mod grading;
pub mod lesson;
pub mod lyceum;
pub mod pdf;
pub mod repository;
pub mod schema;
pub mod storage;
pub mod study_summaries;
pub mod sync;
pub mod sync_watcher;
pub mod tutor;

#[cfg(test)]
mod tests {
    use lopdf::{
        Document, Object, Stream,
        content::{Content, Operation},
        dictionary,
    };

    use crate::{
        archive::{self, import_pdf},
        browser::run_browser_smoke,
        canvas::{
            Course, CourseKind, discover_material_links, group_module_items, parse_courses,
            parse_module_items,
        },
        download::validate_material_url,
        lesson::{build_lesson, grade_answers, score_answer},
        pdf::{pages_to_markdown, safe_slug, split_pdf_pages},
    };

    fn two_page_pdf() -> Vec<u8> {
        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Courier"
        });
        let resources_id = document.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id }
        });
        let mut page_ids = Vec::new();
        for phrase in ["Primeira pagina UVA", "Segunda pagina Rust"] {
            let content = Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), 18.into()]),
                    Operation::new("Td", vec![40.into(), 760.into()]),
                    Operation::new("Tj", vec![Object::string_literal(phrase)]),
                    Operation::new("ET", vec![]),
                ],
            };
            let content_id =
                document.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
            page_ids.push(document.add_object(dictionary! {
                "Type" => "Page", "Parent" => pages_id, "Contents" => content_id
            }));
        }
        document.objects.insert(pages_id, Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => page_ids.iter().copied().map(Object::Reference).collect::<Vec<_>>(),
            "Count" => 2, "Resources" => resources_id, "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()]
        }));
        let catalog_id =
            document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
        document.trailer.set("Root", catalog_id);
        document.compress();
        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }

    fn contains_pdf_file(directory: &std::path::Path) -> bool {
        std::fs::read_dir(directory)
            .unwrap()
            .flatten()
            .any(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    contains_pdf_file(&path)
                } else {
                    path.extension().is_some_and(|extension| extension == "pdf")
                }
            })
    }

    #[test]
    fn extracts_pdf_into_separate_page_text() {
        let pages = split_pdf_pages(&two_page_pdf()).unwrap();
        assert_eq!(pages.len(), 2);
        assert!(pages[0].contains("Primeira pagina UVA"));
        assert!(pages[1].contains("Segunda pagina Rust"));
    }

    #[test]
    fn markdown_preserves_page_origins_and_removes_query_secrets() {
        let md = pages_to_markdown(
            "Introdução <script>",
            "https://ead.uva.br/aula.pdf?token=nao-gravar#page=8",
            &["Texto da página 1".into(), "Texto da página 2".into()],
        );
        assert!(md.contains("## Página 1"));
        assert!(md.contains("## Página 2"));
        assert!(md.contains("https://ead.uva.br/aula.pdf"));
        assert!(!md.contains("token=nao-gravar"));
        assert!(!md.contains("<script>"));
    }

    #[test]
    fn slug_is_portuguese_safe_and_nonempty() {
        assert_eq!(safe_slug("Introdução à Web / U1"), "introducao-a-web-u1");
        assert_eq!(safe_slug(" !!! "), "material");
    }

    #[test]
    fn course_and_module_parsers_keep_dynamic_ids_and_skip_activities() {
        let courses = parse_courses(
            r#"
          <h2>Disciplinas atuais</h2><a class="course" href="/courses/8765">Desenvolvimento Web</a>
          <h2>Inscrições anteriores</h2><a class="course" href="/courses/4321">Bases de Programação</a>
        "#,
        );
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].id, "8765");
        assert_eq!(courses[0].kind, CourseKind::Current);
        assert_eq!(courses[1].kind, CourseKind::Past);

        let items = parse_module_items(
            r#"
          <div class="context_module" id="module_10">
            <h3 class="ig-header-title">Unidade 1</h3>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/modules/items/21">E-book U1</a></div>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/assignments/31">Entregar prova</a></div>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/discussion_topics/41">Fórum avaliativo</a></div>
          </div>
        "#,
        );
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].module_title, "Unidade 1");
        assert_eq!(items[0].title, "E-book U1");
        let groups = group_module_items(items);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Unidade 1");
        assert_eq!(groups[0].items.len(), 1);
    }

    #[test]
    fn material_discovery_includes_embedded_ead_documents_without_pdf_suffix() {
        let materials = discover_material_links(
            r#"
              <iframe title="E-book da unidade" src="https://ead.uva.br/reader/book/314159"></iframe>
              <a href="https://uva.instructure.com/courses/8765/files/42/download">Arquivo PDF</a>
              <iframe title="vídeo externo" src="https://video.example.net/embed/42"></iframe>
            "#,
            "https://uva.instructure.com/courses/8765/modules/items/21",
        );
        assert_eq!(materials.len(), 2);
        assert_eq!(materials[0].title, "E-book da unidade");
        assert!(materials[0].embedded);
        assert_eq!(materials[1].title, "Arquivo PDF");
        assert!(!materials[1].embedded);
    }

    #[test]
    fn material_urls_require_https_and_uva_host_boundaries() {
        assert!(validate_material_url("https://ead.uva.br/aula.pdf").is_ok());
        assert!(validate_material_url("https://uva.instructure.com/files/12").is_ok());
        assert!(validate_material_url("https://evil-eaduva.br/aula.pdf").is_err());
        assert!(validate_material_url("https://ead.uva.br.attacker.example/aula.pdf").is_err());
        assert!(validate_material_url("https://usuario@ead.uva.br/aula.pdf").is_err());
        assert!(validate_material_url("https://ead.uva.br:8080/aula.pdf").is_err());
        assert!(validate_material_url("http://ead.uva.br/aula.pdf").is_err());
        assert!(validate_material_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn lesson_and_score_are_deterministic_and_trace_pages() {
        let inputs = [
            (
                1,
                "# HTML\nHTML significa HyperText Markup Language. HTML estrutura o conteúdo da página.",
            ),
            (2, "## CSS\nCSS controla a apresentação visual da página."),
        ];
        let lesson = build_lesson("Desenvolvimento Web", &inputs);
        assert!(lesson.contains("Página 1"));
        assert!(lesson.contains("Página 2"));
        assert_eq!(lesson, build_lesson("Desenvolvimento Web", &inputs));
        assert_eq!(score_answer(" html ", "HTML"), 1);
        assert_eq!(score_answer("minha resposta", "HTML"), 0);
        let (_, questions) = crate::lesson::generate_lesson("Desenvolvimento Web", &inputs);
        let grade = grade_answers(&["HyperText Markup Language".into()], &questions);
        assert_eq!(grade.correct, 1);
        assert_eq!(grade.total, 1);
        assert_eq!(grade.percent, 100);
        assert_eq!(grade.answers[0].source_page, 1);
    }

    #[test]
    fn imported_material_is_markdown_idempotent_and_never_keeps_raw_pdf() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store = crate::storage::AcademicStore::open(
            &temporary.path().join("app/db.sqlite"),
            library.clone(),
        )
        .unwrap();
        let course = crate::canvas::Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let course_dir = library.join("materias/desenvolvimento-web-8765");
        let materials_dir = course_dir.join("materiais");
        std::fs::create_dir_all(&materials_dir).unwrap();
        std::fs::write(
            library.join("materias/README.md"),
            "Índice manual da faculdade.\n",
        )
        .unwrap();
        std::fs::write(
            course_dir.join("README.md"),
            "Observação manual da matéria.\n",
        )
        .unwrap();
        let manual_material = materials_dir.join("e-book-u1.md");
        std::fs::write(&manual_material, "Anotação que não pode ser sobrescrita.\n").unwrap();
        let bytes = two_page_pdf();
        let pages = split_pdf_pages(&bytes).unwrap();
        let first = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            Some("2026.2"),
            "https://ead.uva.br/aula.pdf?download_token=secret",
            &bytes,
            &pages,
        )
        .unwrap();
        let again = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            Some("2026.2"),
            "https://ead.uva.br/aula.pdf?download_token=secret",
            &bytes,
            &pages,
        )
        .unwrap();
        assert_eq!(first.material.id, again.material.id);
        assert_eq!(first.material.semester.as_deref(), Some("2026.2"));
        assert!(
            std::fs::read_to_string(course_dir.join("README.md"))
                .unwrap()
                .contains("período 2026.2")
        );
        assert!(first.material.path.ends_with("e-book-u1-2.md"));
        assert_eq!(
            std::fs::read_to_string(&manual_material).unwrap(),
            "Anotação que não pode ser sobrescrita.\n"
        );
        assert!(
            std::fs::read_to_string(library.join("materias/README.md"))
                .unwrap()
                .contains("Índice manual da faculdade.")
        );
        assert!(
            std::fs::read_to_string(course_dir.join("README.md"))
                .unwrap()
                .contains("Observação manual da matéria.")
        );
        assert!(std::path::Path::new(&first.material.path).exists());
        assert!(std::path::Path::new(&first.material.lesson_path).exists());
        assert!(
            !std::path::Path::new(&first.material.path)
                .to_string_lossy()
                .contains("secret")
        );
        assert_eq!(store.list_materials().unwrap().len(), 1);
        assert!(!contains_pdf_file(temporary.path()));
        assert!(first.lesson_markdown.contains("Prática de recuperação"));
    }

    #[test]
    fn existing_material_can_be_reused_by_source_url_without_pdf_bytes() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            crate::storage::AcademicStore::open(&temporary.path().join("app/db.sqlite"), library)
                .unwrap();
        let course = Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let source = "https://ead.uva.br/aula.pdf?download_token=segredo";
        let bytes = two_page_pdf();
        let pages = split_pdf_pages(&bytes).unwrap();
        let imported = archive::import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            Some("2026.1"),
            source,
            &bytes,
            &pages,
        )
        .unwrap();

        let reused = archive::reuse_existing_material(
            &store,
            &course,
            Some("2026.2"),
            "https://ead.uva.br/aula.pdf?download_token=rotacionado",
        )
        .unwrap()
        .expect("material local deveria ser encontrado antes de uma nova transferência");

        assert_eq!(reused.material.id, imported.material.id);
        assert_eq!(reused.material.semester.as_deref(), Some("2026.2"));
        assert_eq!(reused.material.origin, "https://ead.uva.br/aula.pdf");
        assert_eq!(store.list_materials().unwrap().len(), 1);
        assert_eq!(reused.lesson_markdown, imported.lesson_markdown);
        let course_index = std::path::Path::new(&reused.material.path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("README.md");
        assert!(
            std::fs::read_to_string(course_index)
                .unwrap()
                .contains("período 2026.2")
        );
    }

    #[test]
    fn image_only_or_invalid_pdf_does_not_create_archive_output() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store = crate::storage::AcademicStore::open(
            &temporary.path().join("app/db.sqlite"),
            library.clone(),
        )
        .unwrap();
        assert!(split_pdf_pages(b"this is not a PDF").is_err());
        let course = crate::canvas::Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let result = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            None,
            "https://ead.uva.br/aula.pdf",
            &two_page_pdf(),
            &[" ".into(), "\n\t".into()],
        );
        assert!(matches!(result, Err(crate::archive::ArchiveError::NoText)));
        assert_eq!(store.list_materials().unwrap().len(), 0);
        assert!(!library.join("materias").exists());
    }

    #[tokio::test]
    async fn launches_real_isolated_chromium_and_reads_fixture_dom() {
        if !std::path::Path::new("/opt/homebrew/bin/chromium").exists() {
            panic!("Chromium esperado em /opt/homebrew/bin/chromium está ausente");
        }
        let profile = tempfile::tempdir().unwrap();
        run_browser_smoke("/opt/homebrew/bin/chromium", profile.path(), true)
            .await
            .unwrap();
        assert!(profile.path().exists());
    }
}
