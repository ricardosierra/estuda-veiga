import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import type { LessonSession } from "./studyTypes";
import { splitLessonSections } from "./studyTypes";

export function LessonPresenter({
  title,
  courseTitle,
  markdown,
  documentPath,
  session,
  onBack,
  onSessionChanged,
  onRelatedDocument,
  onCompleted,
}: {
  title: string;
  courseTitle: string;
  markdown: string;
  documentPath?: string;
  session: LessonSession;
  onBack: () => void;
  onSessionChanged: (session: LessonSession) => void;
  onRelatedDocument: (relativePath: string) => void;
  onCompleted: () => void;
}) {
  const sections = useMemo(() => splitLessonSections(markdown), [markdown]);
  const [index, setIndex] = useState(() => Math.min(session.sectionIndex, Math.max(0, sections.length - 1)));
  const [saving, setSaving] = useState(false);
  const [completed, setCompleted] = useState(!!session.completedAt);
  const [saveError, setSaveError] = useState("");
  const section = sections[index] ?? sections[0];

  useEffect(() => {
    setIndex(Math.min(session.sectionIndex, Math.max(0, sections.length - 1)));
    setCompleted(!!session.completedAt);
  }, [session.id, session.sectionIndex, session.completedAt, sections.length]);

  const saveProgress = async (nextIndex: number, markComplete = false) => {
    setSaving(true);
    setSaveError("");
    try {
      const next = await invoke<LessonSession>("save_lesson_progress", {
        sessionId: session.id,
        sectionIndex: nextIndex,
        sectionCount: sections.length,
        completed: markComplete,
      });
      onSessionChanged(next);
      setIndex(nextIndex);
      if (markComplete) {
        setCompleted(true);
        onCompleted();
      }
    } catch (reason) {
      setSaveError(typeof reason === "string" ? reason : "Não foi possível salvar seu progresso. Tente novamente.");
    } finally {
      setSaving(false);
    }
  };

  const components: Components = {
    img: ({ src, alt }) => <LessonImage documentPath={documentPath} src={src} alt={alt} />,
    a: ({ href, children }) => {
      if (href && isSupportedLocalDocument(href)) {
        const target = resolveRelativeDocument(documentPath, href);
        return target ? <a href={href} onClick={(event) => { event.preventDefault(); onRelatedDocument(target); }}>{children}</a> : <span>{children}</span>;
      }
      if (href && /^(?:https?:|mailto:)/i.test(href)) return <a href={href} target={/^https?:/i.test(href) ? "_blank" : undefined} rel="noreferrer">{children}</a>;
      return <span>{children}</span>;
    },
  };

  return <section className="content-width lesson-presenter-page">
    <button className="back-link" onClick={onBack}>← <span>Voltar à matéria</span></button>
    <header className="presenter-header"><div><p className="eyebrow">{courseTitle}</p><h1>{title}</h1><p>{session.completedAt ? "Aula concluída · você pode revisar quando quiser." : `Progresso salvo · seção ${index + 1} de ${sections.length}`}</p></div><span className="presenter-progress-label">{Math.round(((index + (completed ? 1 : 0)) / sections.length) * 100)}%</span></header>
    <div className="presenter-progress-track"><span style={{ width: `${Math.min(100, ((index + (completed ? 1 : 0)) / sections.length) * 100)}%` }} /></div>
    <div className="presenter-layout">
      <aside className="presenter-outline"><small>SEÇÕES DA AULA</small>{sections.map((item, itemIndex) => <button key={`${item.title}-${itemIndex}`} className={itemIndex === index ? "active" : itemIndex < index || completed ? "read" : ""} onClick={() => { void saveProgress(itemIndex); }} disabled={saving}><span>{itemIndex + 1}</span>{item.title}</button>)}</aside>
      <article className="lesson-card presenter-content"><div className="lesson-toolbar"><div><span className="lesson-badge">AULA GUIADA</span><p>{documentPath ? "Material disponível na pasta local" : "Material importado e convertido para texto"}</p></div>{documentPath && <details className="presenter-source"><summary>Origem</summary><code>{documentPath}</code></details>}</div>{saveError && <p className="presenter-save-error" role="alert">{saveError}</p>}<div className="presenter-section-heading"><small>SEÇÃO {index + 1} DE {sections.length}</small><h2>{section.title}</h2></div><div className="markdown-body"><ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>{section.markdown}</ReactMarkdown></div>
        <footer className="presenter-controls"><button className="button secondary" onClick={() => { void saveProgress(Math.max(0, index - 1)); }} disabled={index === 0 || saving}>← Anterior</button><span>{index + 1} / {sections.length}</span>{index + 1 < sections.length ? <button className="button primary" onClick={() => { void saveProgress(index + 1); }} disabled={saving}>{saving ? "Salvando…" : "Próxima seção →"}</button> : <button className="button primary" onClick={() => { void saveProgress(index, true); }} disabled={saving || completed}>{completed ? "Aula concluída ✓" : saving ? "Salvando…" : "Concluir aula ✓"}</button>}</footer>
      </article>
    </div>
  </section>;
}

function LessonImage({ documentPath, src, alt }: { documentPath?: string; src?: string; alt?: string }) {
  const [url, setUrl] = useState("");
  const [unavailable, setUnavailable] = useState(false);
  useEffect(() => {
    let disposed = false;
    let objectUrl = "";
    setUrl("");
    setUnavailable(false);
    const relativePath = src ? resolveRelativeDocument(documentPath, src) : null;
    if (!relativePath || !/\.(?:png|jpe?g|webp|gif)$/i.test(relativePath)) {
      setUnavailable(true);
      return () => { disposed = true; };
    }
    void invoke<number[]>("read_local_asset", { relativePath }).then((bytes) => {
      if (disposed) return;
      const extension = relativePath.split(".").pop()?.toLowerCase();
      const mime = extension === "png" ? "image/png" : extension === "webp" ? "image/webp" : extension === "gif" ? "image/gif" : "image/jpeg";
      objectUrl = URL.createObjectURL(new Blob([Uint8Array.from(bytes).buffer as ArrayBuffer], { type: mime }));
      setUrl(objectUrl);
    }).catch(() => { if (!disposed) setUnavailable(true); });
    return () => { disposed = true; if (objectUrl) URL.revokeObjectURL(objectUrl); };
  }, [documentPath, src]);
  if (url) return <img src={url} alt={alt ?? "Imagem do material"} loading="lazy" />;
  if (unavailable) return <span className="local-image-unavailable">Imagem local indisponível: {alt ?? src ?? "sem descrição"}</span>;
  return <span className="local-image-loading">Carregando imagem local…</span>;
}

function resolveRelativeDocument(documentPath: string | undefined, href: string): string | null {
  if (!documentPath || /^(?:[a-z]+:|\/\/)/i.test(href)) return null;
  const base = documentPath.split("/").slice(0, -1);
  for (const segment of href.split(/[?#]/, 1)[0].split("/")) {
    if (!segment || segment === ".") continue;
    if (segment === "..") {
      if (!base.length) return null;
      base.pop();
    } else base.push(segment);
  }
  const target = base.join("/");
  return target && !target.startsWith("/") ? target : null;
}

function isSupportedLocalDocument(href: string) {
  return /\.(?:md|markdown|txt|alg|csv|json|pdf|docx|pages)$/i.test(href.split(/[?#]/, 1)[0]);
}
