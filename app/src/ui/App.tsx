import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

type CanvasState = "disconnected" | "waiting_for_login" | "authenticated" | "session_expired" | "browser_error";
type CanvasStatus = { state: CanvasState; message: string; currentUrl: string | null };
type Course = { id: string; title: string; url: string; kind: "current" | "past" };
type ModuleItem = { moduleTitle: string; title: string; href: string; itemType: string };
type ModuleGroup = { title: string; items: ModuleItem[] };
type MaterialChoice = { id: string; title: string; host: string; embedded: boolean; allowed: boolean };
type Material = { id: string; courseId: string; courseTitle: string; moduleTitle: string; title: string; path: string; lessonPath: string; origin: string; pages: number; importedAt: string };
type Overview = { currentCourses: number; pastCourses: number; materials: number; studySessions: number; accuracyPercent: number; libraryPath: string };
type PracticePrompt = { position: number; prompt: string; sourcePage: number };
type PracticeResult = { correct: number; total: number; percent: number; answers: { position: number; correct: boolean; expected: string; sourcePage: number }[] };
type ImportedBundle = { material: Material; lessonMarkdown: string };
type CalendarEvent = { id: string; semester: string; courseId: string | null; courseTitle: string | null; title: string; startsAt: string; allDay: boolean; sourceKind: string; sourceUrl: string; sourceLabel: string };
type CalendarSyncReport = { coursesChecked: number; eventsSaved: number; errors: string[] };
type View = "inicio" | "materias" | "agenda" | "acervo" | "estudar";

const initialStatus: CanvasStatus = { state: "disconnected", message: "Canvas ainda não conectado.", currentUrl: null };

export function App() {
  const [view, setView] = useState<View>("inicio");
  const [status, setStatus] = useState(initialStatus);
  const [overview, setOverview] = useState<Overview | null>(null);
  const [calendarSemester, setCalendarSemester] = useState("2027.1");
  const [calendarEvents, setCalendarEvents] = useState<CalendarEvent[]>([]);
  const [courses, setCourses] = useState<Course[]>([]);
  const [modules, setModules] = useState<ModuleGroup[]>([]);
  const [materials, setMaterials] = useState<Material[]>([]);
  const [choices, setChoices] = useState<MaterialChoice[]>([]);
  const [activeCourse, setActiveCourse] = useState<Course | null>(null);
  const [selectedItem, setSelectedItem] = useState<ModuleItem | null>(null);
  const [activeMaterial, setActiveMaterial] = useState<Material | null>(null);
  const [lesson, setLesson] = useState("");
  const [prompts, setPrompts] = useState<PracticePrompt[]>([]);
  const [answers, setAnswers] = useState<Record<number, string>>({});
  const [result, setResult] = useState<PracticeResult | null>(null);
  const [busy, setBusy] = useState("");
  const [activeImportId, setActiveImportId] = useState<string | null>(null);
  const [cancelPending, setCancelPending] = useState(false);
  const [notice, setNotice] = useState("");
  const [error, setError] = useState("");

  const refresh = useCallback(async () => {
    const settled = await Promise.allSettled([
      invoke<CanvasStatus>("canvas_status"),
      invoke<Overview>("overview"),
      invoke<Material[]>("list_materials"),
      invoke<CalendarEvent[]>("list_calendar_events", { semester: calendarSemester }),
    ]);
    if (settled[0].status === "fulfilled") setStatus(settled[0].value);
    if (settled[1].status === "fulfilled") setOverview(settled[1].value);
    if (settled[2].status === "fulfilled") setMaterials(settled[2].value);
    if (settled[3].status === "fulfilled") setCalendarEvents(settled[3].value);
  }, [calendarSemester]);

  useEffect(() => {
    void refresh();
    const timer = window.setInterval(() => { void refresh(); }, 3000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  const withBusy = useCallback(async (key: string, action: () => Promise<void>) => {
    setBusy(key);
    setError("");
    setNotice("");
    try { await action(); }
    catch (reason) { setError(typeof reason === "string" ? reason : "A etapa não foi concluída. Confira a conexão e tente novamente."); }
    finally { setBusy(""); }
  }, []);

  const connectCanvas = () => withBusy("canvas", async () => {
    const next = await invoke<CanvasStatus>("connect_canvas");
    setStatus(next);
    setNotice(next.message);
  });

  const pickChromium = () => withBusy("chromium", async () => {
    const selection = await open({ title: "Selecione o executável do Chromium ou Chrome", multiple: false, directory: false });
    if (typeof selection !== "string") return;
    await invoke("set_chromium_path", { path: selection });
    setNotice("Navegador salvo. Agora conecte-se ao Canvas.");
  });

  const pickLibrary = () => withBusy("library", async () => {
    const selection = await open({ title: "Selecione a pasta raiz do acervo Faculdade", multiple: false, directory: true });
    if (typeof selection !== "string") return;
    await invoke("set_library_path", { path: selection });
    await refresh();
    setNotice("Pasta do acervo atualizada.");
  });

  const loadCourses = () => withBusy("courses", async () => {
    const result = await invoke<Course[]>("list_courses");
    setCourses(result);
    setModules([]);
    setChoices([]);
    setView("materias");
    setNotice(`${result.length} disciplina(s) carregada(s) diretamente do Canvas.`);
  });

  const syncCalendar = () => withBusy("calendar", async () => {
    const targetSemester = calendarSemester;
    const report = await invoke<CalendarSyncReport>("sync_calendar", { semester: targetSemester });
    setCalendarEvents(await invoke<CalendarEvent[]>("list_calendar_events", { semester: targetSemester }));
    const failures = report.errors.length ? ` ${report.errors.length} matéria(s) não puderam ser atualizadas.` : "";
    setNotice(`${report.eventsSaved} prazo(s) encontrados em ${report.coursesChecked} matéria(s) atuais do Canvas.${failures}`);
    if (report.errors.length) setError(report.errors.join(" · "));
  });

  const selectCourse = (course: Course) => withBusy("course", async () => {
    setActiveCourse(course);
    setModules([]);
    setChoices([]);
    setSelectedItem(null);
    const result = await invoke<ModuleGroup[]>("load_course", { courseId: course.id });
    setModules(result);
    setNotice(`Módulos carregados de ${course.title}.`);
  });

  const inspectItem = (item: ModuleItem) => withBusy("item", async () => {
    if (!activeCourse) throw "Escolha uma disciplina primeiro.";
    setSelectedItem(item);
    const result = await invoke<MaterialChoice[]>("open_module_item", { courseId: activeCourse.id, href: item.href });
    setChoices(result);
    if (result.length === 0) setNotice("Este item não apresentou link de PDF reconhecido. O app não abriu tarefas, fóruns ou avaliações.");
    else setNotice(`${result.length} material(is) identificado(s) neste item.`);
  });

  const importChoice = (choice: MaterialChoice, item: ModuleItem) => withBusy(`import-${choice.id}`, async () => {
    if (!activeCourse) throw "Escolha uma disciplina primeiro.";
    const jobId = crypto.randomUUID();
    setActiveImportId(jobId);
    setCancelPending(false);
    try {
      const bundle = await invoke<ImportedBundle>("import_material", {
        jobId,
        course: activeCourse,
        moduleTitle: item.moduleTitle,
        title: choice.title,
        materialId: choice.id,
      });
      setMaterials((current) => [bundle.material, ...current.filter((material) => material.id !== bundle.material.id)]);
      setActiveMaterial(bundle.material);
      setLesson(bundle.lessonMarkdown);
      setPrompts(await invoke<PracticePrompt[]>("start_practice", { materialId: bundle.material.id }));
      setAnswers({});
      setResult(null);
      setView("estudar");
      await refresh();
      setNotice("PDF convertido em Markdown por página. A aula foi montada localmente, com referências ao material.");
    } finally {
      setActiveImportId((current) => current === jobId ? null : current);
      setCancelPending(false);
    }
  });

  const cancelImport = async () => {
    if (!activeImportId || cancelPending) return;
    setCancelPending(true);
    try {
      await invoke("cancel_import", { jobId: activeImportId });
      setNotice("Cancelamento solicitado. O arquivo não será salvo se a importação ainda estiver em andamento.");
    } catch (reason) {
      setError(typeof reason === "string" ? reason : "Não foi possível cancelar esta importação.");
      setCancelPending(false);
    }
  };

  const openMaterial = (material: Material) => withBusy("lesson", async () => {
    setActiveMaterial(material);
    setLesson(await invoke<string>("get_lesson", { materialId: material.id }));
    setPrompts(await invoke<PracticePrompt[]>("start_practice", { materialId: material.id }));
    setAnswers({});
    setResult(null);
    setView("estudar");
  });

  const submitAnswers = () => withBusy("practice", async () => {
    if (!activeMaterial) throw "Selecione um material para estudar.";
    const submitted = prompts.map((prompt) => answers[prompt.position] ?? "");
    setResult(await invoke<PracticeResult>("submit_practice", { materialId: activeMaterial.id, answers: submitted }));
    await refresh();
  });

  return (
    <div className="app-frame">
      <aside className="sidebar">
        <div className="brand"><span className="brand-mark">E</span><span><b>Estuda UVA</b><small>seu acervo acadêmico</small></span></div>
        <nav aria-label="Navegação principal">
          {([ ["inicio", "Visão geral", "⌂"], ["materias", "Matérias", "▤"], ["agenda", "Agenda", "◷"], ["acervo", "Acervo", "▧"], ["estudar", "Aula", "✳"] ] as [View, string, string][]).map(([key, label, icon]) => (
            <button key={key} className={`nav-item ${view === key ? "active" : ""}`} onClick={() => setView(key)}>
              <span aria-hidden="true">{icon}</span>{label}
              {key === "acervo" && materials.length > 0 && <span className="nav-count">{materials.length}</span>}
            </button>
          ))}
        </nav>
        <div className="sidebar-spacer" />
        <div className="local-note"><span className="status-dot online" /> Seus dados ficam neste computador.<br />Sem IA, nuvem ou cobrança por uso.</div>
        <div className="profile-chip"><span className="avatar">S</span><span><b>Estudo pessoal</b><small>sessão local</small></span><span className="more">•••</span></div>
      </aside>

      <main className="main-area">
        <header className="topbar">
          <div className="breadcrumb">Faculdade <span>/</span> <b>{viewTitle(view)}</b></div>
          <div className="top-actions">
            <span className={`connection-pill ${status.state === "authenticated" ? "connected" : ""}`}><i />{status.state === "authenticated" ? "Canvas conectado" : statusLabel(status.state)}</span>
            <button className="icon-button" title="Atualizar dados locais" onClick={() => void refresh()} aria-label="Atualizar dados locais">↻</button>
          </div>
        </header>

        <div className="page-scroll">
          {notice && <div className="notice" role="status"><span>✓</span>{notice}<button onClick={() => setNotice("")} aria-label="Fechar aviso">×</button></div>}
          {error && <div className="error-banner" role="alert"><span>!</span>{error}<button onClick={() => setError("")} aria-label="Fechar erro">×</button></div>}
          {activeImportId && <div className="import-progress" role="status"><span>Importação em andamento</span><button className="button secondary small" onClick={() => void cancelImport()} disabled={cancelPending}>{cancelPending ? "Cancelando…" : "Cancelar importação"}</button></div>}

          {view === "inicio" && <section className="content-width">
            <div className="welcome-row">
              <div><p className="eyebrow">ACOMPANHAMENTO LOCAL</p><h1>Seu espaço de estudo</h1><p className="lead">Materiais organizados, aulas guiadas e seu progresso — tudo local.</p></div>
              <div className="welcome-art" aria-hidden="true"><span className="art-circle circle-a" /><span className="art-circle circle-b" /><span className="art-book">▤</span><span className="art-star">✳</span></div>
            </div>

            <div className="connection-card">
              <div className="connection-icon">◉</div><div className="connection-copy"><b>{status.state === "authenticated" ? "Canvas pronto para estudar" : "Conecte seu Canvas UVA"}</b><p>{status.message}</p></div>
              {status.state === "authenticated" ? <button className="button primary" disabled={!!busy} onClick={() => void loadCourses()}>{busy === "courses" ? "Atualizando…" : "Atualizar matérias"}<span>→</span></button> : <button className="button primary" disabled={!!busy} onClick={() => void connectCanvas()}>{busy === "canvas" ? "Abrindo Chromium…" : "Conectar ao Canvas"}<span>→</span></button>}
            </div>
            {status.state !== "authenticated" && <div className="inline-tools"><span>O login e o MFA acontecem na janela própria do Chromium.</span><button className="text-button" onClick={() => void pickChromium()}>Escolher navegador</button></div>}

            <div className="section-heading"><div><p className="eyebrow">ACOMPANHAMENTO</p><h2>Seu progresso</h2></div><button className="text-button" onClick={() => setView("acervo")}>Ver acervo <span>→</span></button></div>
            <div className="stats-grid">
              <StatCard label="Matérias atuais" value={overview?.currentCourses ?? 0} caption="carregadas do Canvas" icon="▤" color="mint" />
              <StatCard label="Materiais salvos" value={overview?.materials ?? 0} caption="em Markdown por página" icon="▧" color="blue" />
              <StatCard label="Sessões de estudo" value={overview?.studySessions ?? 0} caption="registradas neste aparelho" icon="◷" color="gold" />
              <StatCard label="Acertos" value={`${overview?.accuracyPercent ?? 0}%`} caption="nas práticas concluídas" icon="↗" color="lilac" />
            </div>

            <div className="section-heading compact"><div><p className="eyebrow">SEU ACERVO</p><h2>Continue de onde parou</h2></div></div>
            {materials.length ? <div className="material-list">{materials.map((material) => <MaterialRow key={material.id} material={material} onOpen={() => void openMaterial(material)} busy={!!busy} />)}</div> : <EmptyCard title="Seu acervo está pronto" copy="Conecte-se ao Canvas, escolha uma matéria e importe o primeiro PDF." action="Ver matérias" onClick={() => setView("materias")} />}
          </section>}

          {view === "agenda" && <section className="content-width">
            <PageIntro eyebrow="DATAS COM FONTE OFICIAL" title="Agenda acadêmica" copy="Prazos de avaliações publicados no Canvas, organizados por semestre e ligados à página de origem." />
            <div className="calendar-toolbar">
              <label className="semester-picker">Semestre
                <select value={calendarSemester} onChange={(event) => setCalendarSemester(event.target.value)}>
                  <option value="2027.1">2027.1 · próximo semestre</option>
                  <option value="2026.2">2026.2 · atual</option>
                </select>
              </label>
              {calendarSemester === "2026.2" && <button className="button primary" disabled={!!busy} onClick={() => void syncCalendar()}>
                {busy === "calendar" ? "Buscando prazos…" : "Atualizar prazos do Canvas"}<span>↻</span>
              </button>}
              <a className="text-button calendar-source-link" href="https://www.uva.br/vida-academica/calendario-academico/" target="_blank" rel="noreferrer">Calendário oficial UVA <span>↗</span></a>
            </div>
            {calendarSemester === "2027.1" && <div className="calendar-source-note">
              <span className="calendar-note-icon">i</span>
              <div><b>Calendário 2027.1 ainda não confirmado</b><p>Não encontrei o calendário acadêmico oficial desse período nas páginas públicas consultadas. A agenda fica sem datas até a publicação ou confirmação pelo Portal/Canvas.</p></div>
            </div>}
            {calendarEvents.length > 0 ? <div className="calendar-event-list">
              <div className="calendar-list-heading"><b>Prazos confirmados</b></div>
              {calendarEvents.map((event) => <CalendarRow key={event.id} event={event} />)}
            </div> : <div className="empty-card calendar-empty">
              <span className="empty-icon">◷</span>
              <h3>{calendarSemester === "2027.1" ? "Nenhuma data oficial confirmada" : "Ainda sem prazos sincronizados"}</h3>
              <p>{calendarSemester === "2027.1" ? "Quando a UVA publicar o calendário de 2027.1 ou as disciplinas aparecerem no Canvas, as datas entram aqui com a fonte." : "Conecte-se ao Canvas e atualize para importar os prazos das disciplinas atuais."}</p>
              {calendarSemester === "2026.2" && <button className="button secondary" onClick={() => void connectCanvas()} disabled={!!busy}>Conectar ao Canvas<span>→</span></button>}
            </div>}
          </section>}

          {view === "materias" && <section className="content-width">
            <PageIntro eyebrow="CANVAS UVA" title="Matérias" copy="Escolha uma disciplina e percorra os módulos que o Canvas apresenta." />
            <div className="toolbar-card"><div className="toolbar-copy"><span className="mini-icon">◉</span><div><b>{status.state === "authenticated" ? "Sessão autenticada" : "Acesso ao Canvas"}</b><small>{status.message}</small></div></div><button className="button secondary" disabled={!!busy} onClick={() => void loadCourses()}>{busy === "courses" ? "Carregando…" : "Atualizar lista"}</button></div>
            {courses.length === 0 ? <EmptyCard title="Carregue suas disciplinas" copy="A lista vem diretamente da sua conta Canvas e separa o período atual dos anteriores." action="Carregar do Canvas" onClick={() => void loadCourses()} /> : <div className="course-layout">
              <div className="course-list"><div className="list-head"><span>DISCIPLINAS</span><span>{courses.length}</span></div>{courses.map((course) => <button key={course.id} className={`course-item ${activeCourse?.id === course.id ? "selected" : ""}`} onClick={() => void selectCourse(course)}>
                <span className="course-icon">{course.kind === "current" ? "▤" : "◷"}</span><span className="course-copy"><b>{course.title}</b><small>{course.kind === "current" ? "Período atual" : "Inscrição anterior"}</small></span><span className="course-arrow">›</span>
              </button>)}</div>
              <div className="module-panel"><div className="panel-title"><div><p className="eyebrow">CONTEÚDO DO CANVAS</p><h3>{activeCourse?.title ?? "Selecione uma matéria"}</h3></div>{activeCourse && <span className="soft-tag">ID dinâmico · {activeCourse.id}</span>}</div>
                {!activeCourse && <div className="panel-placeholder"><span>▤</span><b>Os módulos aparecem aqui</b><p>Selecione uma disciplina para buscar unidades, páginas e materiais.</p></div>}
                {activeCourse && modules.length === 0 && !busy && <div className="panel-placeholder"><span>◌</span><b>Nenhum módulo encontrado</b><p>Atualize ou abra a disciplina na janela Canvas para conferir o conteúdo.</p></div>}
                {modules.map((module) => <div className="module-group" key={module.title}><div className="module-heading"><span className="unit-number">▤</span><b>{module.title}</b></div>{module.items.map((item) => <div className="module-item" key={item.href}><span className="file-icon">{item.itemType === "file" ? "PDF" : "↗"}</span><div className="module-copy"><b>{item.title}</b><small>{item.itemType === "file" ? "Arquivo do curso" : "Item de módulo"}</small></div><button className="text-button" disabled={!!busy} onClick={() => void inspectItem(item)}>{busy === "item" ? "Abrindo…" : "Localizar PDF"}<span>→</span></button></div>)}</div>)}
                {choices.length > 0 && <div className="choice-box"><div className="choice-head"><div><p className="eyebrow">MATERIAL ENCONTRADO</p><h3>Confira a origem antes de importar</h3></div><button className="close-button" onClick={() => setChoices([])} aria-label="Fechar">×</button></div>{choices.map((choice) => <div className="choice-row" key={choice.id}><span className={`file-icon ${choice.allowed ? "pdf-icon" : "blocked-icon"}`}>{choice.allowed ? "PDF" : "!"}</span><div className="module-copy"><b>{choice.title}</b><small>{choice.host}{choice.embedded ? " · incorporado no módulo" : " · link do módulo"}</small></div><button className="button primary small" disabled={!choice.allowed || !!busy || !selectedItem} onClick={() => { if (selectedItem) void importChoice(choice, selectedItem); }}>{busy.startsWith("import-") ? "Importando…" : choice.allowed ? "Importar PDF" : "Origem bloqueada"}</button></div>)}</div>}
              </div>
            </div>}
          </section>}

          {view === "acervo" && <section className="content-width">
            <PageIntro eyebrow="ARQUIVO LOCAL" title="Acervo" copy="PDFs viram arquivos Markdown por página. O PDF bruto é usado apenas em memória e descartado." />
            <div className="library-location"><div className="folder-icon">▱</div><div><b>Pasta raiz do acervo</b><small>{overview?.libraryPath ?? "Carregando…"}</small></div><button className="button secondary" disabled={!!busy} onClick={() => void pickLibrary()}>Escolher pasta</button></div>
            {materials.length ? <div className="material-list large">{materials.map((material) => <MaterialRow key={material.id} material={material} onOpen={() => void openMaterial(material)} busy={!!busy} />)}</div> : <EmptyCard title="Ainda não há materiais" copy="Os arquivos Markdown serão agrupados por disciplina, com links para a origem oficial." action="Ir para matérias" onClick={() => setView("materias")} />}
          </section>}

          {view === "estudar" && <section className="content-width study-content">
            <PageIntro eyebrow="AULA LOCAL · SEM IA" title={activeMaterial?.title ?? "Aula guiada"} copy={activeMaterial ? `${activeMaterial.courseTitle} · ${activeMaterial.moduleTitle} · ${activeMaterial.pages} páginas` : "Importe um material para montar sua primeira aula guiada."} />
            {!activeMaterial ? <EmptyCard title="Escolha um material" copy="Abra um PDF importado no acervo ou encontre um novo nos módulos do Canvas." action="Abrir acervo" onClick={() => setView("acervo")} /> : <div className="study-layout">
              <article className="lesson-card"><div className="lesson-toolbar"><div><span className="lesson-badge">ROTEIRO DETERMINÍSTICO</span><p>Extraído do texto do material, com as páginas da fonte.</p></div><a className="origin-link" href={activeMaterial.origin} target="_blank" rel="noreferrer">Origem oficial ↗</a></div><div className="markdown-body"><ReactMarkdown remarkPlugins={[remarkGfm]}>{lesson}</ReactMarkdown></div></article>
              <aside className="practice-card"><div className="practice-top"><span className="practice-icon">✳</span><p className="eyebrow">PRÁTICA DE RECUPERAÇÃO</p><h3>Teste o que aprendeu</h3><p>Responda sem consultar o roteiro. A conferência e a pontuação são calculadas no Rust.</p></div>
                {prompts.length === 0 ? <div className="no-questions">O texto não apresentou definições claras o bastante para perguntas automáticas. Use o roteiro de leitura e confira diretamente as páginas.</div> : <div className="question-list">{prompts.map((prompt) => <label className="question" key={prompt.position}><span>{prompt.prompt}</span><small>Fonte: página {prompt.sourcePage}</small><textarea rows={3} value={answers[prompt.position] ?? ""} onChange={(event) => setAnswers((current) => ({ ...current, [prompt.position]: event.target.value }))} placeholder="Escreva sua resposta…" disabled={!!result} /></label>)}<button className="button primary full" disabled={!!busy || !!result} onClick={() => void submitAnswers()}>{busy === "practice" ? "Conferindo…" : "Conferir respostas"}<span>→</span></button></div>}
                {result && <div className="score-result" role="status"><div className="score-circle">{result.percent}<small>%</small></div><div><b>{result.correct} de {result.total} acerto(s)</b><p>Resultado salvo neste aparelho.</p></div><button className="text-button" onClick={() => { setAnswers({}); setResult(null); }}>Tentar novamente</button>{result.answers.map((answer) => <div key={answer.position} className={`answer-note ${answer.correct ? "right" : "wrong"}`}><b>{answer.correct ? "Correto" : "Resposta de referência"} · pág. {answer.sourcePage}</b><p>{answer.expected}</p></div>)}</div>}
              </aside>
            </div>}
          </section>}
        </div>
      </main>
    </div>
  );
}

function viewTitle(view: View) { return ({ inicio: "Visão geral", materias: "Matérias", agenda: "Agenda", acervo: "Acervo", estudar: "Aula guiada" })[view]; }
function statusLabel(state: CanvasState) { return ({ disconnected: "Desconectado", waiting_for_login: "Aguardando login", authenticated: "Conectado", session_expired: "Sessão expirada", browser_error: "Erro no navegador" })[state]; }
function PageIntro({ eyebrow, title, copy }: { eyebrow: string; title: string; copy: string }) { return <div className="page-intro"><p className="eyebrow">{eyebrow}</p><h1>{title}</h1><p className="lead">{copy}</p></div>; }
function StatCard({ label, value, caption, icon, color }: { label: string; value: string | number; caption: string; icon: string; color: string }) { return <div className="stat-card"><span className={`stat-icon ${color}`}>{icon}</span><div className="stat-label">{label}</div><div className="stat-value">{value}</div><div className="stat-caption">{caption}</div></div>; }
function EmptyCard({ title, copy, action, onClick }: { title: string; copy: string; action: string; onClick: () => void }) { return <div className="empty-card"><span className="empty-icon">▤</span><h3>{title}</h3><p>{copy}</p><button className="button secondary" onClick={onClick}>{action}<span>→</span></button></div>; }
function MaterialRow({ material, onOpen, busy }: { material: Material; onOpen: () => void; busy: boolean }) { return <button className="material-row" onClick={onOpen} disabled={busy}><span className="material-cover"><span>MD</span></span><span className="material-info"><b>{material.title}</b><small>{material.courseTitle} <i>·</i> {material.moduleTitle}</small><small>{material.pages} páginas · Markdown</small></span><span className="material-open">Abrir aula <b>→</b></span></button>; }
function CalendarRow({ event }: { event: CalendarEvent }) {
  return <article className="calendar-event">
    <div className="calendar-event-date"><span>◷</span><time dateTime={event.startsAt}>{event.startsAt}</time></div>
    <div className="calendar-event-copy"><b>{event.title}</b><small>{event.courseTitle ?? event.sourceLabel}</small></div>
    <a className="calendar-event-source" href={event.sourceUrl} target="_blank" rel="noreferrer">Abrir fonte ↗</a>
  </article>;
}
