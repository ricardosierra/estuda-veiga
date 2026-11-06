import { normalizeSubjectName, type StudyHistoryEntry } from "./studyTypes";

export function StudyHistory({
  items,
  loading,
  courseTitle,
  onOpenLesson,
}: {
  items: StudyHistoryEntry[];
  loading: boolean;
  courseTitle?: string;
  onOpenLesson?: (entry: StudyHistoryEntry) => void;
}) {
  const visible = courseTitle ? items.filter((item) => item.courseTitle && normalizeSubjectName(item.courseTitle) === normalizeSubjectName(courseTitle)) : items;
  return <section className="study-history">
    {loading && !visible.length ? <div className="study-history-empty"><span className="study-spinner" /><b>Carregando histórico…</b></div> : !visible.length ? <div className="study-history-empty"><span className="history-empty-icon">◷</span><b>Ainda não há aulas no histórico</b><p>Quando você apresentar uma aula ou concluir uma prática, ela aparecerá aqui.</p></div> : <ol className="study-history-list">
      {visible.map((entry) => <li key={`${entry.kind}-${entry.id}`} className="study-history-item">
        <span className={`history-type-icon ${entry.kind}`}>{entry.kind === "lesson" ? "▤" : "✳"}</span>
        <div className="history-entry-copy"><small>{entry.kind === "lesson" ? "AULA" : "PRÁTICA"} · {formatHistoryDate(entry.occurredAt)}</small><b>{entry.materialTitle}</b><span>{entry.courseTitle ?? "Matéria não identificada"}</span>
          {entry.kind === "lesson" && entry.sectionCount ? <em>{entry.completedAt ? "Concluída" : `Seção ${(entry.sectionIndex ?? 0) + 1} de ${entry.sectionCount}`} · iniciada {formatHistoryDate(entry.startedAt)}</em> : null}
          {entry.kind === "practice" && <em>{entry.correct ?? 0} de {entry.total ?? 0} acerto(s) · {entry.percent ?? 0}%</em>}
        </div>
        {entry.kind === "lesson" && entry.resourcePath && onOpenLesson && <button className="history-reopen" onClick={() => onOpenLesson(entry)} aria-label={`${entry.completedAt ? "Revisar" : "Retomar"} ${entry.materialTitle}`}>{entry.completedAt ? "Revisar" : "Retomar"} →</button>}
      </li>)}
    </ol>}
  </section>;
}

function formatHistoryDate(value: string) {
  const parsed = new Date(value.includes("T") ? value : `${value.replace(" ", "T")}Z`);
  return Number.isNaN(parsed.getTime()) ? value : new Intl.DateTimeFormat("pt-BR", { dateStyle: "short", timeStyle: "short" }).format(parsed);
}
