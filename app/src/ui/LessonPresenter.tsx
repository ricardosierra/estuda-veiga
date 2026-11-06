import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import type { LessonInteraction, LessonSession, TutorActivity, TutorAssessment } from "./studyTypes";
import { buildTutorFeedback, splitLessonSections, submitTutorAnswer, tutorSectionInput } from "./studyTypes";
import { useLessonSpeech } from "./lessonSpeech";

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
  const [activity, setActivity] = useState<TutorActivity | null>(null);
  const [interaction, setInteraction] = useState<LessonInteraction | null>(null);
  const [interactions, setInteractions] = useState<LessonInteraction[]>([]);
  const [response, setResponse] = useState("");
  const [tutorSaving, setTutorSaving] = useState(false);
  const [tutorError, setTutorError] = useState("");
  const section = sections[index] ?? sections[0];
  const speech = useLessonSpeech(section.markdown, `${session.id}:${index}`);

  useEffect(() => {
    setIndex(Math.min(session.sectionIndex, Math.max(0, sections.length - 1)));
    setCompleted(!!session.completedAt);
  }, [session.id, session.sectionIndex, session.completedAt, sections.length]);

  useEffect(() => {
    let disposed = false;
    setInteractions([]);
    void invoke<LessonInteraction[]>("list_lesson_interactions", { sessionId: session.id })
      .then((saved) => { if (!disposed) setInteractions(saved); })
      .catch((reason) => {
        if (!disposed) setTutorError(typeof reason === "string" ? reason : "Não foi possível carregar suas respostas locais.");
      });
    return () => { disposed = true; };
  }, [session.id]);

  useEffect(() => {
    let disposed = false;
    setActivity(null);
    setInteraction(null);
    setResponse("");
    setTutorError("");
    void invoke<TutorActivity>("build_lesson_activity", {
      section: tutorSectionInput(section),
      sessionId: session.id,
    }).then(async (built) => {
      if (disposed) return;
      setActivity(built);
      const saved = await invoke<LessonInteraction | null>("get_lesson_interaction", {
        sessionId: session.id,
        activityId: built.id,
        currentSourceSha256: built.sourceSha256,
      });
      if (disposed || !saved) return;
      setInteraction(saved);
      setResponse(saved.response);
    }).catch((reason) => {
      if (!disposed) setTutorError(typeof reason === "string" ? reason : "Não foi possível preparar a atividade local desta seção.");
    });
    return () => { disposed = true; };
  }, [session.id, section.index, section.title, section.markdown, section.sourcePage]);

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

  const navigateToSection = (nextIndex: number) => {
    speech.stop();
    void saveProgress(nextIndex);
  };

  const speakAndAdvance = () => {
    if (saving) return;
    speech.speak(() => {
      const nextIndex = index + 1;
      if (nextIndex < sections.length) void saveProgress(nextIndex);
    });
  };

  const repeatSpeech = () => {
    if (!saving) speech.speak();
  };

  const saveTutorResponse = async () => {
    if (!activity || !response.trim() || tutorSaving) return;
    setTutorSaving(true);
    setTutorError("");
    try {
      const saved = await submitTutorAnswer(activity, response, {
        assess: (nextActivity, answer) => invoke<TutorAssessment>("assess_lesson_answer", { activity: nextActivity, response: answer }),
        save: (sessionId, nextActivity, answer, assessment) => invoke<LessonInteraction>("save_lesson_interaction", {
          sessionId,
          activity: nextActivity,
          response: answer,
          assessment,
        }),
      });
      setInteraction(saved.interaction);
      setInteractions((current) => [...current, saved.interaction]);
    } catch (reason) {
      setTutorError(typeof reason === "string" ? reason : reason instanceof Error ? reason.message : "Não foi possível salvar sua resposta local. O texto continua no campo para tentar novamente.");
    } finally {
      setTutorSaving(false);
    }
  };

  const goToSource = () => {
    const source = document.getElementById(`lesson-source-${index}`);
    source?.focus({ preventScroll: true });
    source?.scrollIntoView({ behavior: "smooth", block: "start" });
  };

  const sourceChanged = interaction?.sourceChanged === true;
  const feedback = activity && interaction && !sourceChanged
    ? buildTutorFeedback(activity, interaction.assessment, true)
    : null;
  const currentSourceExcerpt = sourceChanged ? interaction.sourceExcerpt : activity?.sourceExcerpt;

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
      <aside className="presenter-outline"><small>SEÇÕES DA AULA</small>{sections.map((item, itemIndex) => <button key={`${item.title}-${itemIndex}`} className={itemIndex === index ? "active" : itemIndex < index || completed ? "read" : ""} onClick={() => { navigateToSection(itemIndex); }} disabled={saving}><span>{itemIndex + 1}</span>{item.title}</button>)}</aside>
      <article className="lesson-card presenter-content"><div className="lesson-toolbar"><div><span className="lesson-badge">AULA GUIADA</span><p>{documentPath ? "Material disponível na pasta local" : "Material importado e convertido para texto"}</p></div>{documentPath && <details className="presenter-source"><summary>Origem</summary><code>{documentPath}</code></details>}</div>
        <div className="lesson-speech-controls" role="group" aria-label="Controles de narração local">
          <div className="lesson-speech-buttons">
            <button type="button" className="button secondary" onClick={speakAndAdvance} disabled={!speech.available || saving}>Ouvir</button>
            <button type="button" className="button secondary" onClick={speech.pause} disabled={!speech.speaking}>Pausar</button>
            <button type="button" className="button secondary" onClick={speech.resume} disabled={!speech.paused}>Continuar</button>
            <button type="button" className="button secondary" onClick={repeatSpeech} disabled={!speech.available || saving}>Repetir</button>
            <button type="button" className="button secondary" onClick={speech.stop} disabled={!speech.speaking && !speech.paused}>Parar</button>
          </div>
          <p aria-live="polite">{speech.available ? `Voz local: ${speech.voiceLabel}${speech.speaking ? " · narrando; avança ao terminar" : speech.paused ? " · pausada" : " · Ouvir avança ao terminar; Repetir mantém nesta página"}` : "Narração local indisponível neste WebView. O conteúdo continua disponível para leitura e resposta digitada."}</p>
        </div>
        {saveError && <p className="presenter-save-error" role="alert">{saveError}</p>}<div className="presenter-section-heading"><small>SEÇÃO {index + 1} DE {sections.length}</small><h2>{section.title}</h2></div><div className="markdown-body" id={`lesson-source-${index}`} tabIndex={-1}><ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>{section.markdown}</ReactMarkdown></div>
        <section className="lesson-tutor-card" aria-label="Aula interativa local">
          <div className="lesson-tutor-heading"><span>✳</span><div><small>AULA INTERATIVA · TREINO LOCAL</small><p>Suas respostas ficam neste dispositivo; não são sincronizadas nem exportadas.</p></div></div>
          {activity ? <>
            <p className="lesson-tutor-question">{activity.prompt}</p>
            <small className="lesson-tutor-source-label">Fonte local: {activity.sourceHeading}{activity.sourcePage ? ` · página ${activity.sourcePage}` : " · página não informada"}</small>
            <button type="button" className="text-button lesson-source-jump" onClick={goToSource}>Ir ao trecho da fonte acima ↑</button>
            <label className="lesson-tutor-answer" htmlFor={`lesson-answer-${session.id}-${index}`}>Sua resposta
              <textarea id={`lesson-answer-${session.id}-${index}`} value={response} onChange={(event) => { setResponse(event.target.value); setInteraction(null); }} placeholder="Escreva com suas palavras…" maxLength={16_384} rows={4} />
            </label>
            <button className="button primary" type="button" onClick={() => { void saveTutorResponse(); }} disabled={tutorSaving || !response.trim()}>{tutorSaving ? "Salvando…" : interaction ? "Salvar outra tentativa" : "Enviar resposta"}</button>
            {tutorError && <p className="lesson-tutor-error" role="alert">{tutorError}</p>}
            {sourceChanged && <p className="lesson-tutor-changed" role="status">O Markdown foi alterado desde esta tentativa. Confira o trecho salvo antes de usar a resposta na versão atual.</p>}
            {feedback && <div className={`lesson-tutor-feedback ${feedback.tone}`} role="status"><b>{feedback.label}</b>{feedback.showReference && feedback.referenceAnswer && <p><strong>Referência:</strong> {feedback.referenceAnswer}</p>}</div>}
            <details className="lesson-tutor-source"><summary>{sourceChanged ? "Trecho salvo na tentativa anterior" : "Conferir trecho da fonte"}</summary><pre>{currentSourceExcerpt ?? "Trecho de origem indisponível."}</pre></details>
            {interactions.length > 0 && <details className="lesson-tutor-history"><summary>{interactions.length} tentativa(s) salvas nesta sessão</summary><ol>{interactions.map((attempt) => <li key={attempt.id}><p><b>{formatTutorAssessment(attempt.assessment)}</b> · seção {attempt.sectionIndex + 1} · {new Date(attempt.attemptedAt).toLocaleString("pt-BR")}</p><blockquote>{attempt.response || "Sem resposta digitada."}</blockquote><button type="button" className="text-button" onClick={() => { navigateToSection(Math.min(attempt.sectionIndex, sections.length - 1)); }}>Revisar esta seção</button>{activity && attempt.sourceSha256 !== activity.sourceSha256 && attempt.sectionIndex === index && <small>O Markdown mudou desde esta tentativa.</small>}</li>)}</ol></details>}
          </> : <>{tutorError ? <p className="lesson-tutor-error" role="alert">{tutorError}</p> : <p role="status">Preparando atividade local desta seção…</p>}</>}
        </section>
        <footer className="presenter-controls"><button className="button secondary" onClick={() => { navigateToSection(Math.max(0, index - 1)); }} disabled={index === 0 || saving}>← Anterior</button><span>{index + 1} / {sections.length}</span>{index + 1 < sections.length ? <button className="button primary" onClick={() => { navigateToSection(index + 1); }} disabled={saving}>{saving ? "Salvando…" : "Próxima seção →"}</button> : <button className="button primary" onClick={() => { speech.stop(); void saveProgress(index, true); }} disabled={saving || completed}>{completed ? "Aula concluída ✓" : saving ? "Salvando…" : "Concluir aula ✓"}</button>}</footer>
      </article>
    </div>
  </section>;
}

function formatTutorAssessment(assessment: TutorAssessment) {
  return assessment.status === "correct" ? "Referência correspondente" : assessment.status === "pending" ? "Pendente" : "Autoavaliação";
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
