import type { ActiveEnrollment, StudyDocument, ImportedStudyMaterial, StudyHistoryEntry } from "./studyTypes";
import { documentsForEnrollment, materialsForEnrollment } from "./studyTypes";

export function StudyHome({
  enrollments,
  documents,
  materials,
  history,
  loading,
  indexing,
  partial,
  loadError,
  semester,
  onOpenSubject,
  onContinue,
  onOpenHistory,
}: {
  enrollments: ActiveEnrollment[];
  documents: StudyDocument[];
  materials: ImportedStudyMaterial[];
  history: StudyHistoryEntry[];
  loading: boolean;
  indexing: boolean;
  partial: boolean;
  loadError?: string;
  semester?: string;
  onOpenSubject: (enrollment: ActiveEnrollment) => void;
  onContinue: (entry: StudyHistoryEntry) => void;
  onOpenHistory: () => void;
}) {
  const active = enrollments.filter((item) => item.status === "enrolled");
  const recentLesson = history.find((item) => item.kind === "lesson" && !item.completedAt);

  return <section className="content-width study-home">
    <div className="study-welcome">
      <div><p className="eyebrow">SEU ESPAÇO DE ESTUDO{semester ? ` · ${semester}` : ""}</p><h1>O que vamos estudar?</h1><p className="lead">Escolha uma matéria para abrir as aulas, materiais e seu histórico.</p></div>
      <div className="study-welcome-mark" aria-hidden="true">✦</div>
    </div>

    {recentLesson && <button className="continue-study" onClick={() => onContinue(recentLesson)}>
      <span className="continue-icon">▶</span><span><small>CONTINUAR AULA</small><b>{recentLesson.materialTitle}</b><em>{recentLesson.courseTitle} · seção {(recentLesson.sectionIndex ?? 0) + 1} de {recentLesson.sectionCount ?? 1}</em></span><span className="continue-arrow">→</span>
    </button>}

    <div className="study-section-heading"><div><p className="eyebrow">PERÍODO ATIVO</p><h2>Minhas matérias</h2></div><button className="text-button" onClick={onOpenHistory}>Ver histórico <span>→</span></button></div>
    {loading && !enrollments.length ? <div className="study-state-card"><span className="study-spinner" /><div><b>Carregando suas matérias</b><p>Consultando o catálogo acadêmico local.</p></div></div> : !active.length ? <div className="study-state-card"><span className="subject-placeholder">▤</span><div><b>{loadError ? "Não foi possível carregar as matrículas" : "Matrículas atuais não encontradas"}</b><p>{loadError ?? "Os registros locais continuam disponíveis em Dados e sincronização. Atualize o registro acadêmico para montar sua lista de matérias."}</p></div></div> : <div className="subject-card-grid">
      {active.map((enrollment) => {
        const localCount = documentsForEnrollment(enrollment, documents).length;
        const importedCount = materialsForEnrollment(enrollment, materials).length;
        const count = localCount + importedCount;
        return <button className="subject-card" key={enrollment.id} onClick={() => onOpenSubject(enrollment)}>
          <span className="subject-card-icon">▤</span><span className="subject-card-content"><small>{enrollment.section ? `${enrollment.section} · ` : ""}{enrollment.semester ?? semester ?? "Matrícula ativa"}</small><b>{enrollment.title}</b><em>{indexing ? "Lendo a pasta local…" : count ? `${count} material(is) disponível(is)` : partial ? "Nenhum material associado no índice parcial" : "Conteúdo ainda não associado"}</em></span><span className="subject-card-arrow">›</span>
        </button>;
      })}
    </div>}
    {(indexing || partial) && <p className="study-indexing-note">{indexing ? "A leitura da pasta continua em andamento. As matérias já carregadas permanecem acessíveis." : "O índice da pasta está parcial; alguns arquivos podem ainda não aparecer nas matérias."}</p>}
  </section>;
}

export function SubjectCards({
  enrollments,
  documents,
  materials,
  partial = false,
  onOpenSubject,
}: {
  enrollments: ActiveEnrollment[];
  documents: StudyDocument[];
  materials: ImportedStudyMaterial[];
  partial?: boolean;
  onOpenSubject: (enrollment: ActiveEnrollment) => void;
}) {
  return <div className="subject-card-grid">
    {enrollments.map((enrollment) => {
      const count = documentsForEnrollment(enrollment, documents).length + materialsForEnrollment(enrollment, materials).length;
      return <button className="subject-card" key={enrollment.id} onClick={() => onOpenSubject(enrollment)}>
        <span className="subject-card-icon">▤</span><span className="subject-card-content"><small>{enrollment.semester ?? "Período não informado"}{enrollment.section ? ` · ${enrollment.section}` : ""}</small><b>{enrollment.title}</b><em>{count ? `${count} material(is) associado(s)` : partial ? "Nenhum material associado no índice parcial" : "Sem material associado na pasta indexada"}</em></span><span className="subject-card-arrow">›</span>
      </button>;
    })}
  </div>;
}
