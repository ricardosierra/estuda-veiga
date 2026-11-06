import type { CanvasCaptureLimits, CaptureItem, CaptureRun } from "./canvasCapture";
import { captureItemStatusLabel, capturePrivacyCopy, captureProgressLabel, captureRunActions, groupCaptureItemsByCourse, validateCanvasCaptureLimits } from "./canvasCapture";

const LIMIT_FIELDS: { key: keyof CanvasCaptureLimits; label: string; min: number; max: number; step: number }[] = [
  { key: "maxCoursePages", label: "Páginas de cursos", min: 1, max: 100, step: 1 },
  { key: "maxCourses", label: "Cursos", min: 1, max: 500, step: 1 },
  { key: "maxItems", label: "Itens", min: 1, max: 20_000, step: 1 },
  { key: "maxPdfBytes", label: "Tamanho máximo do PDF (MiB)", min: 1, max: 60, step: 1 },
  { key: "maxRedirects", label: "Redirecionamentos", min: 0, max: 4, step: 1 },
  { key: "itemTimeoutSeconds", label: "Tempo por item (segundos)", min: 5, max: 120, step: 1 },
];

export function CanvasCapturePanel({
  run,
  items,
  hasMoreItems,
  limits,
  busy,
  error,
  canvasAuthenticated,
  onStart,
  onResume,
  onCancel,
  onConnect,
  onRetry,
  onLimitsChange,
  onSaveLimits,
  onLoadMore,
}: {
  run: CaptureRun | null;
  items: CaptureItem[];
  hasMoreItems: boolean;
  limits: CanvasCaptureLimits | null;
  busy: boolean;
  error?: string;
  canvasAuthenticated: boolean;
  onStart: () => void;
  onResume: () => void;
  onCancel: () => void;
  onConnect: () => void;
  onRetry: (item: CaptureItem) => void;
  onLimitsChange: (limits: CanvasCaptureLimits) => void;
  onSaveLimits: () => void;
  onLoadMore: () => void;
}) {
  const actions = captureRunActions(run);
  const limitErrors = limits ? validateCanvasCaptureLimits(limits) : [];
  const groups = groupCaptureItemsByCourse(items);
  const processed = run ? run.counts.captured + run.counts.reused + run.counts.ignored + run.counts.failed + run.counts.needsReview : 0;

  return <section className="canvas-capture-panel" aria-labelledby="canvas-capture-title">
    <header className="capture-panel-heading">
      <div><p className="eyebrow">MATERIAIS PUBLICADOS · LEITURA AUTOMÁTICA</p><h2 id="canvas-capture-title">Capturar aulas do Canvas</h2></div>
      <span className={`capture-state capture-state-${run?.status ?? "idle"}`}>{run ? captureProgressLabel(run) : "Pronto para iniciar"}</span>
    </header>
    <p className="capture-privacy-copy">{capturePrivacyCopy()}</p>

    <div className="capture-actions">
      {!canvasAuthenticated && <button className="button secondary" disabled={busy} onClick={onConnect}>Abrir Canvas para entrar ↗</button>}
      {actions.canStart && canvasAuthenticated && <button className="button primary" disabled={busy || !limits || limitErrors.length > 0} onClick={onStart}>{busy ? "Iniciando…" : "Capturar todas as disciplinas"}<span>↓</span></button>}
      {actions.canResume && <button className="button primary" disabled={busy || !canvasAuthenticated} onClick={onResume}>{busy ? "Retomando…" : run?.status === "waiting_for_login" ? "Retomar após entrar no Canvas" : "Retomar captura"}<span>↻</span></button>}
      {actions.canCancel && <button className="button secondary" disabled={busy} onClick={onCancel}>Cancelar com segurança</button>}
      {run && <span className="capture-counts">{run.counts.coursesDone}/{run.counts.coursesTotal} cursos · {processed}/{run.counts.itemsTotal} itens processados · {run.counts.pending} na fila</span>}
    </div>

    {run && run.counts.itemsTotal > 0 && <progress className="capture-progress" max={run.counts.itemsTotal} value={Math.min(processed, run.counts.itemsTotal)} aria-label="Progresso da captura" />}
    {run?.warning && <p className="capture-warning" role="status">{run.warning}</p>}
    {error && <p className="capture-error" role="alert">{error}</p>}

    <details className="capture-limits">
      <summary>Limites da varredura</summary>
      {limits ? <>
        <div className="capture-limit-grid">{LIMIT_FIELDS.map(({ key, label, min, max, step }) => <label key={key}>{label}
          <input type="number" min={min} max={max} step={step} value={key === "maxPdfBytes" ? Math.round(limits[key] / (1024 * 1024)) : limits[key]} onChange={(event) => {
            const input = Number(event.target.value);
            onLimitsChange({ ...limits, [key]: key === "maxPdfBytes" ? input * 1024 * 1024 : input });
          }} />
        </label>)}</div>
        {limitErrors.map((message) => <p key={message} className="capture-warning" role="alert">{message}</p>)}
        <div className="capture-limits-footer"><small>Um item é processado por vez. A captura guarda o limite usado ao começar e pode ser retomada depois.</small><button className="button secondary small" disabled={busy || limitErrors.length > 0} onClick={onSaveLimits}>Salvar limites</button></div>
      </> : <p>Carregando limites salvos…</p>}
    </details>

    {groups.length > 0 ? <div className="capture-course-groups">{groups.map((group) => <article className="capture-course" key={group.courseId}>
      <h3>{group.courseTitle}</h3>
      <div className="capture-item-list">{group.items.map((item) => <CaptureItemRow key={item.referenceKey} item={item} onRetry={onRetry} busy={busy} />)}</div>
    </article>)}</div> : run && run.counts.itemsTotal > 0 ? <p className="capture-empty-items">A lista detalhada dos materiais será carregada ao abrir esta seção.</p> : null}
    {hasMoreItems && <button className="button secondary small" disabled={busy} onClick={onLoadMore}>Carregar mais resultados</button>}
  </section>;
}

function CaptureItemRow({ item, onRetry, busy }: { item: CaptureItem; onRetry: (item: CaptureItem) => void; busy: boolean }) {
  return <div className={`capture-item capture-item-${item.status}`}>
    <span className="capture-item-kind">{item.sourceKind === "pdf" ? "PDF → MD" : item.sourceKind === "canvas_page" ? "PÁGINA → MD" : "OUTRO"}</span>
    <div className="capture-item-copy"><b>{item.title}</b><small>{item.moduleTitle} · {captureItemStatusLabel(item.status)}{item.message ? ` · ${item.message}` : ""}</small></div>
    {item.status === "failed" && item.retryable && <button className="text-button" disabled={busy} onClick={() => onRetry(item)}>Tentar novamente</button>}
    {item.status === "needs_review" && <span className="capture-review-label">Revisar arquivo local</span>}
  </div>;
}
