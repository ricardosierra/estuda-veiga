import { useState } from "react";
import {
  canvasAcademicOptionLabels,
  captureCounts,
  defaultCanvasAcademicOptions,
  formatDate,
  formatGrade,
  hasAnyOption,
  type CanvasAcademicCapture,
  type CanvasAcademicOptions,
  type TextEntry,
} from "./canvasAcademic";

type Props = {
  canvasConnected: boolean;
  capture: CanvasAcademicCapture | null;
  busy: boolean;
  onConnect: () => void;
  onCapture: (options: CanvasAcademicOptions) => void;
};

function Entries({ entries }: { entries: TextEntry[] }) {
  return <div className="canvas-academic-entries">
    {entries.map((entry, index) => <article key={index} style={{ marginLeft: `${Math.min(entry.depth, 6) * 14}px` }}>
      <small><b>{entry.author ?? "Autor não identificado"}</b> · {formatDate(entry.createdAt)}</small>
      <p>{entry.text}</p>
    </article>)}
  </div>;
}

export function CanvasAcademicPanel({ canvasConnected, capture, busy, onConnect, onCapture }: Props) {
  const [options, setOptions] = useState<CanvasAcademicOptions>(defaultCanvasAcademicOptions);
  const counts = capture ? captureCounts(capture) : null;
  return <section className="academic-portal-card" aria-labelledby="canvas-academic-title">
    <div className="academic-portal-header">
      <div><p className="eyebrow">PLATAFORMA VIRTUAL · SOMENTE LEITURA</p><h2 id="canvas-academic-title">Notas, fóruns, avisos e Caixa de Entrada do Canvas</h2></div>
      <span className={`portal-state portal-state-${canvasConnected ? "authenticated" : "disconnected"}`}><i />{canvasConnected ? "Canvas conectado" : "Canvas aguarda login"}</span>
    </div>
    <p className="academic-portal-copy">Lê pela API do Canvas, na janela Chromium do app, sem IA. Nada é postado, enviado ou marcado como lido. A nota oficial continua sendo a do boletim do Lyceum; a captura vai para <code>materias/catalogo/capturas/</code>, pasta fora do Git.</p>
    <fieldset className="canvas-academic-options" disabled={busy}>
      <legend>O que ler</legend>
      {(Object.keys(canvasAcademicOptionLabels) as (keyof CanvasAcademicOptions)[]).map((key) => <label key={key}>
        <input type="checkbox" checked={options[key]} onChange={(event) => setOptions({ ...options, [key]: event.target.checked })} />
        {canvasAcademicOptionLabels[key]}
      </label>)}
    </fieldset>
    {options.inbox && <p className="portal-login-help">A Caixa de Entrada tem mensagens privadas com tutores, coordenação e monitoria. Elas ficam só no acervo local.</p>}
    <div className="academic-portal-actions">
      {canvasConnected
        ? <button className="button primary" disabled={busy || !hasAnyOption(options)} onClick={() => onCapture(options)}>{busy ? "Lendo o Canvas…" : "Ler agora"}</button>
        : <button className="button primary" disabled={busy} onClick={onConnect}>{busy ? "Abrindo…" : "Conectar Canvas"}</button>}
    </div>
    {capture && counts && <div className="portal-capture-result">
      <div className="portal-capture-summary">
        <b>{capture.complete ? "Captura concluída" : "Captura parcial"}</b>
        <span>{counts.courses} disciplina(s) · {counts.graded}/{counts.grades} atividade(s) com nota · {counts.feedback} com feedback · {counts.forums} fórum(ns), {counts.forumEntries} mensagem(ns) · {counts.announcements} aviso(s){capture.options.inbox ? ` · ${counts.conversations} conversa(s)` : ""} · {formatDate(capture.capturedAt)}</span>
      </div>
      {capture.jsonPath && <small>JSON sincronizado: {capture.jsonPath}</small>}
      {capture.markdownPath && <small>Markdown sincronizado: {capture.markdownPath}</small>}
      {capture.warnings.map((warning) => <p className="catalog-note" key={warning}>{warning}</p>)}
      {capture.courses.map((course) => <details className="portal-page" key={course.courseId}>
        <summary><b>{course.title}</b><small>{course.grades.length} atividade(s) · {course.forums.length} fórum(ns) · {course.announcements.length} aviso(s){course.warnings.length ? ` · ${course.warnings.length} aviso(s) de leitura` : ""}</small></summary>
        <small className="portal-source-url">{course.sourceUrl}</small>
        {course.warnings.map((warning) => <p className="catalog-note" key={warning}>{warning}</p>)}
        {course.grades.length > 0 && <div className="portal-table-cards">
          <h4>Notas das atividades (A1)</h4>
          {course.grades.map((item) => <article className="portal-row-card" key={item.assignmentId}>
            <div><small>Atividade</small><b>{item.name}</b></div>
            <div><small>Nota</small><b>{formatGrade(item)}</b></div>
            <div><small>Prazo</small><b>{formatDate(item.dueAt)}</b></div>
            <div><small>Entregue</small><b>{formatDate(item.submittedAt)}{item.late ? " · atrasada" : ""}</b></div>
            {item.comments.length > 0 && <div style={{ gridColumn: "1 / -1" }}><small>Feedback do tutor</small><Entries entries={item.comments} /></div>}
          </article>)}
        </div>}
        {course.forums.length > 0 && <div className="portal-table-cards">
          <h4>Fóruns</h4>
          {course.forums.map((topic) => <details className="canvas-academic-topic" key={topic.topicId}>
            <summary><b>{topic.title}</b> <small>{topic.graded ? "avaliativo (A1)" : "fórum"}{topic.locked ? " · fechado" : ""} · {topic.entries.length} mensagem(ns){topic.entriesComplete ? "" : " · leitura incompleta"}</small></summary>
            {topic.opening && <p className="canvas-academic-opening">{topic.opening}</p>}
            <Entries entries={topic.entries} />
          </details>)}
        </div>}
        {course.announcements.length > 0 && <div className="portal-table-cards">
          <h4>Avisos</h4>
          {course.announcements.map((item) => <details className="canvas-academic-topic" key={item.topicId}>
            <summary><b>{item.title}</b> <small>{item.author ?? "—"} · {formatDate(item.postedAt)}</small></summary>
            <p className="canvas-academic-opening">{item.text}</p>
          </details>)}
        </div>}
      </details>)}
      {capture.options.inbox && <details className="portal-page">
        <summary><b>Caixa de Entrada</b><small>{capture.conversations.length} conversa(s)</small></summary>
        <div className="portal-table-cards">
          {capture.conversations.map((conversation) => <details className="canvas-academic-topic" key={conversation.conversationId}>
            <summary><b>{conversation.subject}</b> <small>{conversation.participants.join(", ") || "—"} · {formatDate(conversation.lastMessageAt)}{conversation.workflowState === "unread" ? " · não lida no Canvas" : ""}</small></summary>
            {!conversation.messagesChecked && <p className="catalog-note">Mensagens não lidas nesta captura.</p>}
            <Entries entries={conversation.messages} />
          </details>)}
        </div>
      </details>}
    </div>}
  </section>;
}
