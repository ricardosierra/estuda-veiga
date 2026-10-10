import { useMemo, useState } from "react";
import type { ActiveEnrollment, StudyDocument, ImportedStudyMaterial } from "./studyTypes";
import { SubjectCards } from "./StudyHome";

export function SubjectList({
  enrollments,
  documents,
  materials,
  loading,
  indexing,
  partial,
  loadError,
  onOpenSubject,
}: {
  enrollments: ActiveEnrollment[];
  documents: StudyDocument[];
  materials: ImportedStudyMaterial[];
  loading: boolean;
  indexing: boolean;
  partial: boolean;
  loadError?: string;
  onOpenSubject: (enrollment: ActiveEnrollment) => void;
}) {
  const [query, setQuery] = useState("");
  const visible = useMemo(() => {
    const needle = query.trim().toLocaleLowerCase("pt-BR");
    return enrollments.filter((item) => !needle || `${item.title} ${item.section ?? ""} ${item.semester ?? ""}`.toLocaleLowerCase("pt-BR").includes(needle));
  }, [enrollments, query]);

  return <section className="content-width subject-list-page">
    <div className="page-intro"><p className="eyebrow">MATRÍCULAS LOCAIS · SEMESTRE ATIVO</p><h1>Matérias</h1><p className="lead">Entre em uma matéria para ver aulas, materiais e seu histórico.</p></div>
    <label className="subject-search"><span className="sr-only">Buscar matéria</span><span aria-hidden="true">⌕</span><input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Buscar matéria ou turma…" /></label>
    {loading && !enrollments.length ? <div className="study-state-card"><span className="study-spinner" /><div><b>Carregando matrículas</b><p>O conteúdo salvo na pasta não depende do Canvas.</p></div></div> : visible.length ? <SubjectCards enrollments={visible} documents={documents} materials={materials} partial={partial} onOpenSubject={onOpenSubject} /> : <div className="study-state-card"><div><b>{enrollments.length ? "Nenhuma matéria corresponde à busca" : loadError ? "Não foi possível carregar as matrículas" : "Matrículas atuais não encontradas"}</b><p>{enrollments.length ? "Revise o termo digitado." : loadError ?? (indexing ? "A pasta local ainda está sendo indexada; suas matérias aparecem assim que o catálogo carregar." : "Confira o registro acadêmico em Dados e sincronização.")}</p></div></div>}
    {(indexing || partial) && <p className="study-indexing-note">{indexing ? "Indexando documentos da pasta local. Você pode abrir as matérias enquanto isso." : "Índice parcial: alguns materiais podem ainda não aparecer nas matérias."}</p>}
  </section>;
}
