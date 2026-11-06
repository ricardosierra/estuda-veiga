import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  ActiveEnrollment,
  SubjectSummaryRecord,
  SubjectSummarySnapshot,
} from "./studyTypes";
import { pauseSummary, resumeSummary, speakSummary, stopSummary } from "./summarySpeech";

type SummaryKind = "short" | "long";

export function StudySummaryPanel({ enrollment }: { enrollment: ActiveEnrollment }) {
  const [snapshot, setSnapshot] = useState<SubjectSummarySnapshot | null>(null);
  const [loading, setLoading] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [playback, setPlayback] = useState<SummaryKind | null>(null);
  const [paused, setPaused] = useState(false);
  const semester = enrollment.semester?.trim() ?? "";
  const snapshotRequest = useRef(0);
  const generationRequest = useRef(0);

  const fetchSnapshot = useCallback(() => {
    if (!semester) return Promise.resolve(null);
    return invoke<SubjectSummarySnapshot>("get_subject_summary_snapshot", {
      courseId: enrollment.id,
      courseTitle: enrollment.title,
      semester,
    });
  }, [enrollment.id, enrollment.title, semester]);

  const loadSnapshot = useCallback(async (clearError = true) => {
    const requestId = ++snapshotRequest.current;
    if (clearError) setError(null);
    if (!semester) {
      setSnapshot(null);
      setLoading(false);
      return;
    }
    setLoading(true);
    try {
      const next = await fetchSnapshot();
      if (requestId === snapshotRequest.current) setSnapshot(next);
    } catch (reason) {
      if (requestId === snapshotRequest.current) setError(errorMessage(reason));
    } finally {
      if (requestId === snapshotRequest.current) setLoading(false);
    }
  }, [fetchSnapshot, semester]);

  useEffect(() => {
    stopSummary();
    setSnapshot(null);
    setPlayback(null);
    setPaused(false);
    setConfirming(false);
    setGenerating(false);
    void loadSnapshot();
    return () => {
      snapshotRequest.current++;
      generationRequest.current++;
      stopSummary();
    };
  }, [enrollment.id, enrollment.semester, loadSnapshot]);

  const confirmAndGenerate = async () => {
    if (!snapshot || snapshot.sources.length === 0 || !semester || generating) return;
    const confirmedSourceHash = snapshot.currentSourceHash;
    const requestId = ++generationRequest.current;
    setConfirming(false);
    setGenerating(true);
    setError(null);
    try {
      const summary = await invoke<SubjectSummaryRecord>("generate_subject_summaries", {
        courseId: enrollment.id,
        courseTitle: enrollment.title,
        semester,
        confirmedSourceHash,
      });
      if (requestId === generationRequest.current) {
        setSnapshot((current) => current ? { ...current, latestSummary: summary } : current);
      }
    } catch (reason) {
      const message = errorMessage(reason);
      if (requestId === generationRequest.current) {
        setError(message);
        if (message.includes("mudaram")) void loadSnapshot(false);
      }
    } finally {
      if (requestId === generationRequest.current) setGenerating(false);
    }
  };

  const play = (kind: SummaryKind, text: string) => {
    const started = speakSummary(text, () => {
      setPlayback(null);
      setPaused(false);
    });
    if (!started) {
      setError("A síntese de voz não está disponível neste dispositivo.");
      setPlayback(null);
      setPaused(false);
      return;
    }
    setError(null);
    setPlayback(kind);
    setPaused(false);
  };

  const pause = () => {
    pauseSummary();
    setPaused(true);
  };

  const resume = () => {
    resumeSummary();
    setPaused(false);
  };

  const stop = () => {
    stopSummary();
    setPlayback(null);
    setPaused(false);
  };

  const summary = snapshot?.latestSummary ?? null;
  const stale = !!summary && !!snapshot && summary.sourceHash !== snapshot.currentSourceHash;
  const previousSources = summary ? sourcePaths(summary.sourceManifestJson) : [];

  return <section className="subject-summary-panel" aria-labelledby="subject-summary-title">
    <div className="summary-panel-heading">
      <div>
        <p className="eyebrow">REVISÃO PARA OUVIR</p>
        <h2 id="subject-summary-title">Resumos de {enrollment.title}</h2>
        <p>Gere explicações curtas ou detalhadas usando o material didático local desta matéria.</p>
      </div>
      <button className="button secondary" type="button" disabled={loading || generating || !semester} onClick={() => void loadSnapshot()}>
        Atualizar fontes ↻
      </button>
    </div>

    <div className="summary-privacy-note">
      <b>Antes de gerar</b>
      <p>Depois da sua confirmação, os textos dos materiais listados abaixo serão enviados à Anthropic para criar os dois resumos. A chamada pode consumir créditos. Notas e registros oficiais não fazem parte das fontes.</p>
    </div>

    {!semester && <div className="summary-empty-state"><b>Sem semestre associado</b><p>Esta matrícula ativa ainda não informa o semestre necessário para selecionar os materiais.</p></div>}
    {semester && loading && <p className="summary-loading" role="status">Atualizando o catálogo local da matéria…</p>}
    {semester && snapshot && <section className="summary-sources-card" aria-label="Materiais incluídos">
      <div className="summary-card-heading"><div><b>Materiais que podem ser enviados</b><small>{semester} · {snapshot.sources.length} {snapshot.sources.length === 1 ? "arquivo" : "arquivos"}</small></div><span className="summary-local-badge">Acervo local</span></div>
      {snapshot.sources.length > 0 ? <ul className="summary-source-list">
        {snapshot.sources.map((source) => <li key={source.relativePath}>
          <span className="file-format-chip">{source.format.toUpperCase()}</span>
          <span><b>{source.title}</b><small>{source.relativePath} · {formatBytes(source.sizeBytes)}</small></span>
        </li>)}
      </ul> : <p className="summary-no-sources">Não há materiais legíveis associados a esta matéria e semestre. Nenhuma chamada à API será feita.</p>}
      <button className="button primary" type="button" disabled={loading || generating || snapshot.sources.length === 0} onClick={() => setConfirming(true)}>
        {generating ? "Gerando os resumos…" : "Gerar resumos"}
      </button>
    </section>}

    {confirming && snapshot && <section className="summary-confirmation" role="alertdialog" aria-modal="true" aria-labelledby="summary-confirm-title">
      <div><p className="eyebrow">CONFIRMAÇÃO DE ENVIO</p><h3 id="summary-confirm-title">Enviar material à Anthropic?</h3></div>
      <p>Serão enviados os conteúdos de {snapshot.sources.length} {snapshot.sources.length === 1 ? "arquivo" : "arquivos"} de {enrollment.title}, no semestre {semester}. A geração pode consumir créditos da API.</p>
      <ul>{snapshot.sources.map((source) => <li key={source.relativePath}>{source.relativePath}</li>)}</ul>
      <div className="summary-confirm-actions">
        <button className="button secondary" type="button" onClick={() => setConfirming(false)}>Cancelar</button>
        <button className="button primary" type="button" disabled={generating} onClick={() => void confirmAndGenerate()}>Confirmar envio e gerar</button>
      </div>
    </section>}

    {error && <p className="summary-error" role="alert">{error}</p>}

    {summary && <section className="summary-saved-card" aria-label="Resumo salvo">
      <div className="summary-saved-heading">
        <div><p className="eyebrow">RESUMO SALVO LOCALMENTE</p><h3>Disponível para ouvir</h3></div>
        <span className={stale ? "summary-stale-badge" : "summary-current-badge"}>{stale ? "Fontes alteradas" : "Atualizado"}</span>
      </div>
      <p className="summary-metadata">Gerado em {formatDate(summary.createdAt)} · {summary.model} · {previousSources.length} {previousSources.length === 1 ? "fonte" : "fontes"}</p>
      {previousSources.length > 0 && <details className="summary-previous-sources"><summary>Ver fontes usadas nesta geração</summary><ul>{previousSources.map((path) => <li key={path}>{path}</li>)}</ul></details>}
      {stale && <p className="summary-stale-note">O material desta matéria mudou desde esta geração. O resumo anterior continua salvo; gere novamente para usar as fontes atuais.</p>}

      <article className="summary-text-card">
        <div className="summary-text-heading"><div><p className="eyebrow">CERCA DE 2 MINUTOS</p><h4>Resumo curto</h4></div><button className="button secondary small" type="button" onClick={() => play("short", summary.shortText)}>Ouvir resumo curto</button></div>
        <p className="summary-text">{summary.shortText}</p>
      </article>
      <article className="summary-text-card">
        <div className="summary-text-heading"><div><p className="eyebrow">CERCA DE 8 MINUTOS</p><h4>Resumo longo</h4></div><button className="button secondary small" type="button" onClick={() => play("long", summary.longText)}>Ouvir resumo longo</button></div>
        <p className="summary-text">{summary.longText}</p>
      </article>

      <div className="summary-playback-controls" aria-label="Controles de leitura em voz alta">
        <span role="status">{playback ? `${playback === "short" ? "Resumo curto" : "Resumo longo"}${paused ? " pausado" : " sendo lido"}` : "Voz do dispositivo · pt-BR"}</span>
        <div>
          <button type="button" className="button secondary small" disabled={!playback || paused} onClick={pause}>Pausar</button>
          <button type="button" className="button secondary small" disabled={!playback || !paused} onClick={resume}>Retomar</button>
          <button type="button" className="button secondary small" disabled={!playback} onClick={stop}>Parar</button>
        </div>
      </div>
    </section>}
  </section>;
}

function sourcePaths(manifestJson: string): string[] {
  try {
    const manifest: unknown = JSON.parse(manifestJson);
    if (!Array.isArray(manifest)) return [];
    return manifest.flatMap((item) =>
      item && typeof item === "object" && "relativePath" in item && typeof item.relativePath === "string"
        ? [item.relativePath]
        : [],
    );
  } catch {
    return [];
  }
}

function errorMessage(reason: unknown): string {
  return reason instanceof Error ? reason.message : String(reason);
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toLocaleString("pt-BR", { maximumFractionDigits: 1 })} KB`;
  return `${(bytes / (1024 * 1024)).toLocaleString("pt-BR", { maximumFractionDigits: 1 })} MB`;
}

function formatDate(value: string): string {
  const date = new Date(value);
  return Number.isNaN(date.getTime())
    ? value
    : new Intl.DateTimeFormat("pt-BR", { dateStyle: "short", timeStyle: "short" }).format(date);
}
