import { useState } from "react";
import { StudyHistory } from "./StudyHistory";
import type {
  ActiveEnrollment,
  CanvasMaterialChoice,
  CanvasStudyCourse,
  ImportedStudyMaterial,
  StudyDocument,
  StudyHistoryEntry,
  StudyModuleGroup,
  StudyModuleItem,
  StudyOutcome,
} from "./studyTypes";

type Tab = "aulas" | "materiais" | "historico" | "notas";

export function SubjectDetail({
  enrollment,
  documents,
  importedMaterials,
  modules,
  canvasCourse,
  canvasStatus,
  canvasBusy,
  choices,
  selectedItem,
  outcomes,
  history,
  historyLoading,
  onBack,
  onOpenDocument,
  onOpenImported,
  onConnectCanvas,
  onRefreshCanvas,
  onInspectItem,
  onImportChoice,
  onResumeHistory,
}: {
  enrollment: ActiveEnrollment;
  documents: StudyDocument[];
  importedMaterials: ImportedStudyMaterial[];
  modules: StudyModuleGroup[];
  canvasCourse: CanvasStudyCourse | null;
  canvasStatus: string;
  canvasBusy: boolean;
  choices: CanvasMaterialChoice[];
  selectedItem: StudyModuleItem | null;
  outcomes: StudyOutcome[];
  history: StudyHistoryEntry[];
  historyLoading: boolean;
  onBack: () => void;
  onOpenDocument: (document: StudyDocument) => void;
  onOpenImported: (material: ImportedStudyMaterial) => void;
  onConnectCanvas: () => void;
  onRefreshCanvas: () => void;
  onInspectItem: (item: StudyModuleItem) => void;
  onImportChoice: (choice: CanvasMaterialChoice, item: StudyModuleItem) => void;
  onResumeHistory: (entry: StudyHistoryEntry) => void;
}) {
  const [tab, setTab] = useState<Tab>("aulas");
  const outcome = outcomes.find((item) => normalize(item.courseTitle) === normalize(enrollment.title));
  const localLessonCount = documents.filter((document) => document.readStatus !== "unsupported").length;
  const hasLessons = localLessonCount > 0 || importedMaterials.length > 0 || modules.some((module) => module.items.length > 0);

  return <section className="content-width subject-detail-page">
    <button className="back-link" onClick={onBack}>← <span>Minhas matérias</span></button>
    <header className="subject-detail-header"><div><p className="eyebrow">{enrollment.semester ?? "MATRÍCULA ATIVA"}{enrollment.section ? ` · ${enrollment.section}` : ""}</p><h1>{enrollment.title}</h1><p className="lead">{enrollment.instructorAsDisplayed ? `Professor(a): ${enrollment.instructorAsDisplayed}` : "Aulas, materiais e desempenho desta matéria."}</p></div><span className="enrolled-badge"><i /> Em andamento</span></header>

    <nav className="subject-tabs" aria-label={`Seções de ${enrollment.title}`}>
      {([ ["aulas", "Aulas"], ["materiais", "Materiais"], ["historico", "Histórico"], ["notas", "Desempenho"] ] as [Tab, string][]).map(([key, label]) => <button key={key} className={tab === key ? "active" : ""} onClick={() => setTab(key)}>{label}{key === "aulas" && hasLessons ? <span className="tab-count">{localLessonCount + importedMaterials.length + modules.reduce((sum, module) => sum + module.items.length, 0)}</span> : null}</button>)}
    </nav>

    {tab === "aulas" && <div className="subject-lessons-panel">
      <div className="subject-panel-heading"><div><p className="eyebrow">ROTEIRO DE ESTUDO</p><h2>Aulas desta matéria</h2></div>{canvasCourse ? <button className="button secondary" disabled={canvasBusy} onClick={onRefreshCanvas}>{canvasBusy ? "Atualizando…" : "Atualizar aulas do Canvas ↻"}</button> : canvasStatus === "authenticated" ? <button className="button secondary" disabled={canvasBusy} onClick={onRefreshCanvas}>{canvasBusy ? "Procurando matéria…" : "Buscar aulas no Canvas ↻"}</button> : <button className="text-button" onClick={onConnectCanvas}>Conectar ao Canvas para atualizar ↗</button>}</div>
      {documents.filter((document) => document.readStatus !== "unsupported").map((document) => <button className="lesson-list-row" key={document.relativePath} onClick={() => onOpenDocument(document)}>
        <span className="lesson-row-icon">▤</span><span className="lesson-row-copy"><small>{document.semester ? `${document.semester} · ` : "Período do material não informado · "}{document.category ?? "Material local"}</small><b>{document.title}</b><em>Disponível na pasta local</em></span><span className="lesson-row-action">Apresentar →</span>
      </button>)}
      {importedMaterials.map((material) => <button className="lesson-list-row" key={material.id} onClick={() => onOpenImported(material)}>
        <span className="lesson-row-icon imported">✳</span><span className="lesson-row-copy"><small>{material.moduleTitle} · {material.semester ?? "período não informado"} · Canvas</small><b>{material.title}</b><em>{material.pages} páginas · atualizada {formatDate(material.importedAt)}</em></span><span className="lesson-row-action">Apresentar →</span>
      </button>)}
      {modules.map((module) => <article className="canvas-module-card" key={module.title}><div className="canvas-module-heading"><span>▤</span><div><small>UNIDADE / MÓDULO</small><b>{module.title}</b></div></div>{module.items.map((item) => <div className="canvas-lesson-row" key={item.href}><span className="file-format-chip">{item.itemType === "file" ? "ARQ" : "WEB"}</span><span><b>{item.title}</b><small>{item.itemType === "file" ? "Arquivo do curso" : "Conteúdo do módulo"}</small></span><button className="text-button" disabled={canvasBusy} onClick={() => onInspectItem(item)}>Localizar material →</button></div>)}</article>)}
      {selectedItem && choices.length > 0 && <div className="canvas-choice-panel"><div><small>CONTEÚDO ENCONTRADO</small><b>{selectedItem.title}</b></div>{choices.map((choice) => <div className="canvas-choice-row" key={choice.id}><span><b>{choice.title}</b><small>{choice.host}{choice.embedded ? " · incorporado no módulo" : ""}</small></span><button className="button primary small" disabled={!choice.allowed || canvasBusy} onClick={() => onImportChoice(choice, selectedItem)}>{choice.allowed ? "Baixar texto e preparar aula" : "Origem não permitida"}</button></div>)}</div>}
      {!hasLessons && <div className="study-state-card"><span className="subject-placeholder">▤</span><div><b>Nenhuma aula associada ainda</b><p>{canvasStatus === "authenticated" ? "Atualize esta matéria pelo Canvas para procurar módulos e materiais." : "A matéria está salva localmente. Entre no Canvas para buscar aulas novas; o conteúdo local já existente continuará disponível."}</p>{canvasStatus !== "authenticated" && <button className="button secondary" onClick={onConnectCanvas}>Conectar Canvas</button>}</div></div>}
    </div>}

    {tab === "materiais" && <div className="subject-materials-panel">
      <div className="subject-panel-heading"><div><p className="eyebrow">ARQUIVOS ASSOCIADOS</p><h2>Materiais</h2></div></div>
      {[...documents].map((document) => <button className="material-resource-row" key={document.relativePath} onClick={() => onOpenDocument(document)}><span className="file-format-chip">{document.format.toUpperCase()}</span><span><b>{document.title}</b><small>{document.semester ?? "Período não identificado"} · {document.category ?? "Material local"}</small></span><span>↗</span></button>)}
      {importedMaterials.map((material) => <button className="material-resource-row" key={material.id} onClick={() => onOpenImported(material)}><span className="file-format-chip">MD</span><span><b>{material.title}</b><small>{material.moduleTitle} · {material.semester ?? "período não informado"} · {material.pages} páginas · Canvas</small></span><span>↗</span></button>)}
      {!documents.length && !importedMaterials.length && <div className="study-history-empty"><b>Nenhum material local associado</b><p>Use a atualização do Canvas na aba Aulas para procurar conteúdos novos.</p></div>}
    </div>}

    {tab === "historico" && <div className="subject-history-panel"><div className="subject-panel-heading"><div><p className="eyebrow">SUAS SESSÕES</p><h2>Histórico de aulas</h2></div></div><StudyHistory items={history} loading={historyLoading} courseTitle={enrollment.title} onOpenLesson={onResumeHistory} /></div>}

    {tab === "notas" && <div className="subject-performance-panel"><div className="subject-panel-heading"><div><p className="eyebrow">REGISTRO ACADÊMICO</p><h2>Desempenho</h2></div></div>{outcome ? <><div className="performance-metrics"><div><b>{outcome.lyceumAverage == null ? "—" : outcome.lyceumAverage.toLocaleString("pt-BR")}</b><small>Média do Lyceum</small></div><div><b>{outcome.absences == null ? "—" : outcome.absences}</b><small>Faltas</small></div><div><b>{outcome.attendancePercent == null ? "—" : `${outcome.attendancePercent.toLocaleString("pt-BR")}%`}</b><small>Frequência</small></div></div>{outcome.lyceumGrades?.map((grade, index) => <div className="grade-row" key={`${grade.assessment}-${index}`}><span>{grade.assessment}</span><b>{grade.score == null ? "Sem nota" : `${grade.score.toLocaleString("pt-BR")} / ${grade.scale}`}</b><small>{grade.date ?? "Data não exibida"}</small></div>)}{outcome.canvasAvaTotals && <p className="canvas-grade-note">Canvas: {outcome.canvasAvaTotals.earnedPoints} / {outcome.canvasAvaTotals.possiblePoints} pontos nas atividades AVA ({outcome.canvasAvaTotals.percent}%). Esse valor não é a média oficial.</p>}</> : <div className="study-history-empty"><b>Nenhum resultado acadêmico associado</b><p>As notas oficiais aparecem aqui quando estiverem registradas para esta matéria.</p></div>}</div>}
  </section>;
}

function normalize(value: string) {
  return value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLocaleLowerCase("pt-BR").replace(/[^a-z0-9]+/g, " ").trim();
}

function formatDate(value: string) {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : new Intl.DateTimeFormat("pt-BR", { dateStyle: "short" }).format(date);
}
