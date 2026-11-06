import { useCallback, useEffect, useRef, useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { StudyHome } from "./StudyHome";
import { SubjectList } from "./SubjectList";
import { CanvasCapturePanel } from "./CanvasCapturePanel";
import { SubjectDetail } from "./SubjectDetail";
import { StudyHistory } from "./StudyHistory";
import { LessonPresenter } from "./LessonPresenter";
import { invokeWithTimeout } from "./asyncCommand";
import { AcademicPortal, type PortalCapture, type PortalStatus } from "./AcademicPortal";
import { sharedAcademicSubjects, type CatalogCurriculum, type CatalogDiscipline } from "./academicCatalog";
import { documentsForEnrollment, getOrBeginLessonSession, lessonSectionCount, materialsForEnrollment, normalizeSubjectName, type ActiveEnrollment, type ImportedStudyMaterial, type LessonSession, type StudyDocument, type StudyHistoryEntry } from "./studyTypes";
import { validateCanvasCaptureLimits, type CanvasCaptureLimits, type CaptureItem, type CaptureRun } from "./canvasCapture";

type CanvasState = "disconnected" | "waiting_for_login" | "authenticated" | "session_expired" | "browser_error";
type CanvasStatus = { state: CanvasState; message: string; currentUrl: string | null };
type Course = { id: string; title: string; url: string; kind: "current" | "past" };
type ModuleItem = { moduleTitle: string; title: string; href: string; itemType: string };
type ModuleGroup = { title: string; items: ModuleItem[] };
type MaterialChoice = { id: string; title: string; host: string; embedded: boolean; allowed: boolean };
type Material = ImportedStudyMaterial;
type Overview = { currentCourses: number; pastCourses: number; materials: number; studySessions: number; accuracyPercent: number; libraryPath: string };
type PracticePrompt = { position: number; prompt: string; sourcePage: number };
type PracticeResult = { correct: number; total: number; percent: number; answers: { position: number; correct: boolean; expected: string; sourcePage: number }[] };
type ImportedBundle = { material: Material; lessonMarkdown: string };
type CalendarEvent = { id: string; semester: string; courseId: string | null; courseTitle: string | null; title: string; startsAt: string; allDay: boolean; sourceKind: string; sourceUrl: string; sourceLabel: string };
type CalendarSyncReport = { coursesChecked: number; eventsSaved: number; errors: string[] };
type LocalDocument = StudyDocument;
type LocalCatalogSnapshot = { root: string; documents: LocalDocument[]; folders: number; unreadableDirectories: number; unreadableFiles: number; truncated: boolean; status: string; scannedAt: number };
type LocalDocumentContent = { document: LocalDocument; markdown: string; warnings: string[] };
type LocalSearchResponse = { results: { document: LocalDocument; excerpt: string }[]; unreadableFiles: number; truncated: boolean };
type SyncReport = { scanned: number; created: number; updated: number; unchanged: number; conflicts: number; pendingOperations: number; errors: string[] };
type SyncConflict = { id: string; relativePath: string; tombstone: boolean; sqlite: string | null; markdown: string | null; json: string | null; createdAt: string };
type RepositoryStatus = { isRepository: boolean; branch: string | null; trackedChanges: string[]; untrackedFiles: string[]; ignoredFiles: string[]; error: string | null };
type AcademicCurriculum = CatalogCurriculum & { curriculumVersion: string; trajectoryStatus: string };
type AcademicGrade = { assessment: string; score: number | null; scale: number; date: string | null };
type AcademicOutcome = { courseTitle: string; status: string; lyceumGrades?: AcademicGrade[]; lyceumAverage?: number | null; absences?: number | null; attendancePercent?: number | null; canvasCourseCode?: string; canvasAvaTotals?: { earnedPoints: number; possiblePoints: number; percent: number } };
type PriorCanvasCourse = { courseTitle: string; canvasCode: string; earnedPoints: number; possiblePoints: number; percent: number };
type AcademicTrajectory = { programId: string; programTitle: string; programCode: string; status: string; semester?: string; progress?: { totalHours?: number; completedHours?: number; enrolledHours?: number; remainingHours?: number; reportedCompletionPercent?: number; sourceReportedDiscrepancyHours?: number; note?: string }; currentEnrollments?: ActiveEnrollment[]; outcomes?: AcademicOutcome[]; priorCanvasCourses?: PriorCanvasCourse[]; limitations?: string[]; source?: { file?: string; observedAt?: string } };
type AcademicRelation = { id: string; type: string; ccOfferingId?: string; ecOfferingId?: string; equivalenceConfirmed: boolean; note?: string };
type AcademicCatalog = { curriculums: AcademicCurriculum[]; disciplines: CatalogDiscipline[]; relations: AcademicRelation[]; trajectories: AcademicTrajectory[] };
type View = "inicio" | "materias" | "curriculos" | "agenda" | "acervo" | "estudar" | "historico" | "canvas";

const initialStatus: CanvasStatus = { state: "disconnected", message: "Canvas ainda não conectado.", currentUrl: null };
const initialPortalStatus: PortalStatus = { state: "disconnected", message: "Lyceum ainda não conectado.", currentUrl: null, pagesRead: 0, inaccessibleSections: [] };

export function App() {
  const [view, setView] = useState<View>("inicio");
  const [status, setStatus] = useState(initialStatus);
  const [portalStatus, setPortalStatus] = useState(initialPortalStatus);
  const [portalCapture, setPortalCapture] = useState<PortalCapture | null>(null);
  const [overview, setOverview] = useState<Overview | null>(null);
  const [calendarSemester, setCalendarSemester] = useState("2027.1");
  const [calendarEvents, setCalendarEvents] = useState<CalendarEvent[]>([]);
  const [courses, setCourses] = useState<Course[]>([]);
  const [modules, setModules] = useState<ModuleGroup[]>([]);
  const [materials, setMaterials] = useState<Material[]>([]);
  const [captureRun, setCaptureRun] = useState<CaptureRun | null>(null);
  const [captureItems, setCaptureItems] = useState<CaptureItem[]>([]);
  const [captureHasMore, setCaptureHasMore] = useState(false);
  const [captureLimits, setCaptureLimits] = useState<CanvasCaptureLimits | null>(null);
  const [captureItemsLoading, setCaptureItemsLoading] = useState(false);
  const captureRunRef = useRef<CaptureRun | null>(null);
  const captureItemsRequestRef = useRef(0);
  const [localCatalog, setLocalCatalog] = useState<LocalCatalogSnapshot | null>(null);
  const [localDocument, setLocalDocument] = useState<LocalDocumentContent | null>(null);
  const [localQuery, setLocalQuery] = useState("");
  const [localSearch, setLocalSearch] = useState<LocalSearchResponse | null>(null);
  const [localSync, setLocalSync] = useState<SyncReport | null>(null);
  const [localSyncAt, setLocalSyncAt] = useState<string | null>(null);
  const [syncConflicts, setSyncConflicts] = useState<SyncConflict[]>([]);
  const [repositoryStatus, setRepositoryStatus] = useState<RepositoryStatus | null>(null);
  const [academicCatalog, setAcademicCatalog] = useState<AcademicCatalog | null>(null);
  const [academicCatalogError, setAcademicCatalogError] = useState("");
  const [activeEnrollment, setActiveEnrollment] = useState<ActiveEnrollment | null>(null);
  const [studyHistory, setStudyHistory] = useState<StudyHistoryEntry[]>([]);
  const [historyLoading, setHistoryLoading] = useState(false);
  const [studySession, setStudySession] = useState<LessonSession | null>(null);
  const [studyDocumentPath, setStudyDocumentPath] = useState<string | undefined>();
  const [localPage, setLocalPage] = useState(0);
  const [editingLocal, setEditingLocal] = useState(false);
  const [localEditContent, setLocalEditContent] = useState("");
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
      invoke<PortalStatus>("lyceum_status"),
    ]);
    if (settled[0].status === "fulfilled") setStatus(settled[0].value);
    if (settled[1].status === "fulfilled") setOverview(settled[1].value);
    if (settled[2].status === "fulfilled") setMaterials(settled[2].value);
    if (settled[3].status === "fulfilled") setCalendarEvents(settled[3].value);
    if (settled[4].status === "fulfilled") setPortalStatus(settled[4].value);
  }, [calendarSemester]);

  const refreshStudyHistory = useCallback(async () => {
    setHistoryLoading(true);
    try {
      setStudyHistory(await invokeWithTimeout("list_study_history", () => invoke<StudyHistoryEntry[]>("list_study_history", { limit: 200 })));
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : typeof reason === "string" ? reason : "Não foi possível carregar o histórico de estudos.");
    } finally {
      setHistoryLoading(false);
    }
  }, []);

  const refreshLocalLibrary = useCallback(async () => {
    const report = await invoke<SyncReport>("sync_local_catalog");
    setLocalSyncAt(new Date().toISOString());
    const [snapshot, conflicts, academic, repository] = await Promise.allSettled([
      invoke<LocalCatalogSnapshot>("refresh_local_catalog"),
      invoke<SyncConflict[]>("local_sync_conflicts"),
      invoke<AcademicCatalog>("academic_catalog"),
      invoke<RepositoryStatus>("repository_status"),
    ]);
    setLocalSync(report);
    if (snapshot.status === "rejected") throw snapshot.reason;
    setLocalCatalog(snapshot.value);
    if (conflicts.status === "fulfilled") setSyncConflicts(conflicts.value);
    if (repository.status === "fulfilled") setRepositoryStatus(repository.value);
    if (academic.status === "fulfilled") {
      setAcademicCatalog(academic.value);
      setAcademicCatalogError("");
    } else {
      setAcademicCatalog(null);
      setAcademicCatalogError(typeof academic.reason === "string" ? academic.reason : "Catálogos curriculares não encontrados nesta pasta.");
    }
    setLocalSearch(null);
    setLocalPage(0);
  }, []);

  const loadCaptureItemsPage = useCallback(async (runId: string, offset: number, replace: boolean) => {
    const requestId = ++captureItemsRequestRef.current;
    setCaptureItemsLoading(true);
    try {
      const page = await invoke<CaptureItem[]>("canvas_capture_items", { runId, offset, limit: 200 });
      if (requestId !== captureItemsRequestRef.current) return;
      setCaptureItems((current) => replace ? page : [...current, ...page]);
      setCaptureHasMore(page.length === 200);
    } finally {
      if (requestId === captureItemsRequestRef.current) setCaptureItemsLoading(false);
    }
  }, []);

  const refreshCaptureSnapshot = useCallback(async () => {
    const [run, limits] = await Promise.all([
      invoke<CaptureRun | null>("canvas_capture_status", { runId: null }),
      invoke<CanvasCaptureLimits>("get_canvas_capture_limits"),
    ]);
    captureRunRef.current = run;
    setCaptureRun(run);
    setCaptureLimits(limits);
    if (run) await loadCaptureItemsPage(run.id, 0, true);
    else {
      setCaptureItems([]);
      setCaptureHasMore(false);
    }
  }, [loadCaptureItemsPage]);

  useEffect(() => {
    void refresh();
    const timer = window.setInterval(() => { void refresh(); }, 3000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  useEffect(() => { void refreshStudyHistory(); }, [refreshStudyHistory]);

  useEffect(() => { void refreshLocalLibrary().catch((reason) => {
    setError(typeof reason === "string" ? reason : "Não foi possível ler a pasta do acervo.");
  }); }, [refreshLocalLibrary]);

  useEffect(() => {
    void refreshCaptureSnapshot().catch((reason) => {
      setError(typeof reason === "string" ? reason : "Não foi possível carregar o estado da captura do Canvas.");
    });
  }, [refreshCaptureSnapshot]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void listen<CaptureRun>("canvas-capture-updated", (event) => {
      const nextRun = event.payload;
      const previousRun = captureRunRef.current;
      captureRunRef.current = nextRun;
      setCaptureRun(nextRun);
      void loadCaptureItemsPage(nextRun.id, 0, true).catch((reason) => {
        setError(typeof reason === "string" ? reason : "Não foi possível atualizar os itens da captura.");
      });
      const terminal = ["waiting_for_login", "cancelled", "partial", "completed", "failed"].includes(nextRun.status);
      const justReachedTerminal = terminal && (previousRun?.id !== nextRun.id || previousRun.status !== nextRun.status);
      if (justReachedTerminal) {
        void invoke<Material[]>("list_materials").then(setMaterials).catch(() => undefined);
        void refreshLocalLibrary().catch((reason) => {
          setError(typeof reason === "string" ? reason : "Não foi possível atualizar o acervo após a captura.");
        });
      }
    }).then((dispose) => { unlisten = dispose; });
    return () => unlisten?.();
  }, [loadCaptureItemsPage, refreshLocalLibrary]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void listen<SyncReport>("local-sync-updated", (event) => {
      setLocalSync(event.payload);
      setLocalSyncAt(new Date().toISOString());
      void refreshLocalLibrary().catch((reason) => {
        setError(typeof reason === "string" ? reason : "Não foi possível atualizar o acervo monitorado.");
      });
    }).then((dispose) => { unlisten = dispose; });
    return () => unlisten?.();
  }, [refreshLocalLibrary]);

  const withBusy = useCallback(async (key: string, action: () => Promise<void>) => {
    setBusy(key);
    setError("");
    setNotice("");
    try { await action(); }
    catch (reason) { setError(reason instanceof Error ? reason.message : typeof reason === "string" ? reason : "A etapa não foi concluída. Confira a conexão e tente novamente."); }
    finally { setBusy(""); }
  }, []);

  const connectCanvas = () => withBusy("canvas", async () => {
    const next = await invoke<CanvasStatus>("connect_canvas");
    setStatus(next);
    setNotice(next.message);
  });

  const startCanvasCapture = () => withBusy("canvas-capture", async () => {
    if (!captureLimits) throw "Os limites da captura ainda estão carregando.";
    const invalid = validateCanvasCaptureLimits(captureLimits);
    if (invalid.length) throw invalid.join(" · ");
    await invoke<CanvasCaptureLimits>("set_canvas_capture_limits", { limits: captureLimits });
    const next = await invoke<CaptureRun>("start_canvas_capture");
    captureRunRef.current = next;
    setCaptureRun(next);
    setCaptureItems([]);
    await loadCaptureItemsPage(next.id, 0, true);
    setNotice("Captura iniciada. Os materiais publicados serão lidos em sequência e salvos como Markdown.");
  });

  const resumeCanvasCapture = () => withBusy("canvas-capture-resume", async () => {
    if (!captureRun) throw "Não há captura para retomar.";
    const next = await invoke<CaptureRun>("resume_canvas_capture", { runId: captureRun.id });
    captureRunRef.current = next;
    setCaptureRun(next);
    await loadCaptureItemsPage(next.id, 0, true);
    setNotice("Captura retomada; os itens já concluídos foram preservados.");
  });

  const cancelCanvasCapture = () => withBusy("canvas-capture-cancel", async () => {
    if (!captureRun) return;
    await invoke("cancel_canvas_capture", { runId: captureRun.id });
    setNotice("Pedido de cancelamento enviado. O item atual será interrompido com segurança.");
  });

  const retryCanvasCaptureItem = (item: CaptureItem) => withBusy("canvas-capture-retry", async () => {
    if (!captureRun) throw "Não há captura para retomar.";
    const next = await invoke<CaptureRun>("retry_canvas_capture_item", { runId: captureRun.id, referenceKey: item.referenceKey });
    captureRunRef.current = next;
    setCaptureRun(next);
    await loadCaptureItemsPage(next.id, 0, true);
    setNotice(`Nova tentativa iniciada para “${item.title}”.`);
  });

  const saveCanvasCaptureLimits = () => withBusy("canvas-capture-limits", async () => {
    if (!captureLimits) throw "Os limites da captura ainda estão carregando.";
    const invalid = validateCanvasCaptureLimits(captureLimits);
    if (invalid.length) throw invalid.join(" · ");
    setCaptureLimits(await invoke<CanvasCaptureLimits>("set_canvas_capture_limits", { limits: captureLimits }));
    setNotice("Limites da próxima captura salvos.");
  });

  const openLyceum = () => withBusy("lyceum-open", async () => {
    const next = await invoke<PortalStatus>("open_lyceum");
    setPortalStatus(next);
    setNotice(next.message);
  });

  const checkLyceum = () => withBusy("lyceum-check", async () => {
    const next = await invoke<PortalStatus>("lyceum_status");
    setPortalStatus(next);
    setNotice(next.message);
  });

  const captureLyceum = () => withBusy("lyceum-capture", async () => {
    const capture = await invoke<PortalCapture>("refresh_lyceum_catalog");
    setPortalCapture(capture);
    setPortalStatus({
      state: capture.complete ? "authenticated" : "partial",
      message: capture.complete ? "Captura concluída e sincronizada no acervo local." : "Captura parcial salva; páginas inacessíveis continuam identificadas.",
      currentUrl: null,
      pagesRead: capture.pagesRead,
      inaccessibleSections: capture.inaccessibleSections,
    });
    await refreshLocalLibrary();
    setNotice("O registro capturado foi salvo em SQLite, Markdown e JSON.");
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
    await refreshLocalLibrary();
    setLocalDocument(null);
    setNotice("Pasta do acervo atualizada.");
  });

  const loadCourses = () => withBusy("courses", async () => {
    const result = await invoke<Course[]>("list_courses");
    setCourses(result);
    setModules([]);
    setChoices([]);
    setView("canvas");
    setNotice(`${result.length} disciplina(s) carregada(s) diretamente do Canvas.`);
  });

  const refreshSubjectCanvas = (enrollment: ActiveEnrollment) => withBusy("subject-canvas", async () => {
    if (status.state !== "authenticated") throw "Conecte-se ao Canvas pela janela aberta para atualizar as aulas desta matéria.";
    const result = await invoke<Course[]>("list_courses");
    setCourses(result);
    const match = result.find((course) => normalizeSubjectName(course.title) === normalizeSubjectName(enrollment.title));
    if (!match) {
      setActiveCourse(null);
      setModules([]);
      setNotice(`A matéria “${enrollment.title}” não apareceu nas inscrições disponíveis do Canvas. Os materiais locais continuam acessíveis.`);
      return;
    }
    setActiveCourse(match);
    setModules(await invoke<ModuleGroup[]>("load_course", { courseId: match.id }));
    setChoices([]);
    setNotice(`Aulas de ${enrollment.title} atualizadas do Canvas.`);
  });

  const openSubject = (enrollment: ActiveEnrollment) => {
    setActiveEnrollment(enrollment);
    setActiveCourse(null);
    setModules([]);
    setChoices([]);
    setSelectedItem(null);
    setView("materias");
  };

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
    const matchedEnrollment = academicCatalog?.trajectories
      .filter((trajectory) => trajectory.status === "active")
      .flatMap((trajectory) => trajectory.currentEnrollments ?? [])
      .find((enrollment) => normalizeSubjectName(enrollment.title) === normalizeSubjectName(activeCourse.title));
    const semester = activeEnrollment && normalizeSubjectName(activeEnrollment.title) === normalizeSubjectName(activeCourse.title)
      ? activeEnrollment.semester ?? matchedEnrollment?.semester
      : matchedEnrollment?.semester;
    const jobId = crypto.randomUUID();
    setActiveImportId(jobId);
    setCancelPending(false);
    try {
      const bundle = await invoke<ImportedBundle>("import_material", {
        jobId,
        course: activeCourse,
        moduleTitle: item.moduleTitle,
        title: choice.title,
        semester: semester ?? null,
        materialId: choice.id,
      });
      const subject = activeEnrollment && normalizeSubjectName(activeEnrollment.title) === normalizeSubjectName(bundle.material.courseTitle)
        ? activeEnrollment
        : { id: bundle.material.courseId, title: bundle.material.courseTitle, status: "enrolled" };
      const session = await invoke<LessonSession>("begin_lesson_session", {
        resourcePath: `canvas:${bundle.material.id}`,
        courseId: subject.id,
        courseTitle: subject.title,
        materialTitle: bundle.material.title,
        sectionCount: lessonSectionCount(bundle.lessonMarkdown),
      });
      setActiveEnrollment(subject);
      setMaterials((current) => [bundle.material, ...current.filter((material) => material.id !== bundle.material.id)]);
      setActiveMaterial(bundle.material);
      setLocalDocument(null);
      setStudyDocumentPath(undefined);
      setStudySession(session);
      setLesson(bundle.lessonMarkdown);
      setPrompts(await invoke<PracticePrompt[]>("start_practice", { materialId: bundle.material.id }));
      setAnswers({});
      setResult(null);
      setView("estudar");
      await refresh();
      await refreshLocalLibrary();
      await refreshStudyHistory();
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

  const openLocalDocument = (document: LocalDocument | string) => withBusy("local-document", async () => {
    const relativePath = typeof document === "string" ? document : document.relativePath;
    const opened = await invoke<LocalDocumentContent>("read_local_document", { relativePath });
    setLocalDocument(opened);
    setEditingLocal(false);
    setLocalEditContent(opened.markdown);
    setView("acervo");
  });

  const openStudyDocument = (document: LocalDocument | string, enrollment: ActiveEnrollment, existingSessionId?: string) => withBusy("study-document", async () => {
    const relativePath = typeof document === "string" ? document : document.relativePath;
    const opened = await invokeWithTimeout("read_local_document", () => invoke<LocalDocumentContent>("read_local_document", { relativePath }));
    const sectionCount = lessonSectionCount(opened.markdown);
    let session = await getOrBeginLessonSession(existingSessionId,
      () => invokeWithTimeout("begin_lesson_session", () => invoke<LessonSession>("begin_lesson_session", {
        resourcePath: relativePath,
        courseId: enrollment.id,
        courseTitle: enrollment.title,
        materialTitle: opened.document.title,
        sectionCount,
      })),
      (id) => invokeWithTimeout("get_lesson_session", () => invoke<LessonSession | null>("get_lesson_session", { sessionId: id })),
    );
    if (existingSessionId) {
      session = await invokeWithTimeout("save_lesson_progress", () => invoke<LessonSession>("save_lesson_progress", {
        sessionId: session.id,
        sectionIndex: Math.min(session.sectionIndex, sectionCount - 1),
        sectionCount,
        completed: false,
      }));
    }
    setActiveEnrollment(enrollment);
    setLocalDocument(opened);
    setActiveMaterial(null);
    setStudyDocumentPath(opened.document.relativePath);
    setStudySession(session);
    setLesson(opened.markdown);
    setPrompts([]);
    setAnswers({});
    setResult(null);
    setView("estudar");
    void refreshStudyHistory();
  });

  const saveLocalDocument = () => withBusy("local-save", async () => {
    if (!localDocument) return;
    const report = await invoke<SyncReport>("edit_synced_document", {
      relativePath: localDocument.document.relativePath,
      content: localEditContent,
    });
    await refreshLocalLibrary();
    const opened = await invoke<LocalDocumentContent>("read_local_document", { relativePath: localDocument.document.relativePath });
    setLocalDocument(opened);
    setEditingLocal(false);
    setNotice(`Alteração gravada no SQLite, Markdown e espelho JSON.${report.conflicts ? ` ${report.conflicts} conflito(s) aguardam resolução.` : ""}`);
  });

  const resolveLocalConflict = (conflict: SyncConflict, content: string) => withBusy("sync-conflict", async () => {
    const report = await invoke<SyncReport>("resolve_local_sync_conflict", { relativePath: conflict.relativePath, content });
    await refreshLocalLibrary();
    setNotice(`Conflito resolvido em ${conflict.relativePath}.${report.errors.length ? ` ${report.errors.length} item(ns) com erro.` : ""}`);
  });

  const openMaterial = (material: Material, existingSessionId?: string) => withBusy("lesson", async () => {
    const subject = activeEnrollment && normalizeSubjectName(activeEnrollment.title) === normalizeSubjectName(material.courseTitle)
      ? activeEnrollment
      : (academicCatalog?.trajectories.flatMap((trajectory) => trajectory.currentEnrollments ?? []).find((item) => normalizeSubjectName(item.title) === normalizeSubjectName(material.courseTitle))
        ?? { id: material.courseId, title: material.courseTitle, status: "enrolled" });
    const markdown = await invoke<string>("get_lesson", { materialId: material.id });
    const sectionCount = lessonSectionCount(markdown);
    let session = await getOrBeginLessonSession(existingSessionId,
      () => invoke<LessonSession>("begin_lesson_session", {
        resourcePath: `canvas:${material.id}`,
        courseId: subject.id,
        courseTitle: subject.title,
        materialTitle: material.title,
        sectionCount,
      }),
      (id) => invoke<LessonSession | null>("get_lesson_session", { sessionId: id }),
    );
    if (existingSessionId) {
      session = await invoke<LessonSession>("save_lesson_progress", {
        sessionId: session.id,
        sectionIndex: Math.min(session.sectionIndex, sectionCount - 1),
        sectionCount,
        completed: false,
      });
    }
    setActiveEnrollment(subject);
    setActiveMaterial(material);
    setLocalDocument(null);
    setStudyDocumentPath(undefined);
    setStudySession(session);
    setLesson(markdown);
    setPrompts(await invoke<PracticePrompt[]>("start_practice", { materialId: material.id }));
    setAnswers({});
    setResult(null);
    setView("estudar");
    await refreshStudyHistory();
  });

  const searchLocalLibrary = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    void withBusy("local-search", async () => {
      const response = await invoke<LocalSearchResponse>("search_local_library", { query: localQuery });
      setLocalSearch(response);
      setLocalDocument(null);
      setLocalPage(0);
    });
  };

  const submitAnswers = () => withBusy("practice", async () => {
    if (!activeMaterial) throw "Selecione um material para estudar.";
    const submitted = prompts.map((prompt) => answers[prompt.position] ?? "");
    setResult(await invoke<PracticeResult>("submit_practice", { materialId: activeMaterial.id, answers: submitted }));
    await refresh();
    await refreshStudyHistory();
  });

  const localDocumentsToShow = localSearch?.results ?? (localCatalog?.documents ?? []).map((document) => ({ document, excerpt: "" }));
  const localPageSize = 100;
  const localPageCount = Math.max(1, Math.ceil(localDocumentsToShow.length / localPageSize));
  const visibleLocalDocuments = localDocumentsToShow.slice(localPage * localPageSize, (localPage + 1) * localPageSize);
  const activeTrajectory = academicCatalog?.trajectories.find((trajectory) => trajectory.status === "active");
  const activeEnrollments = activeTrajectory?.currentEnrollments ?? [];
  const studyIndexing = !localCatalog || localCatalog.status === "scanning";
  const studyIndexPartial = localCatalog?.status === "partial";
  const currentOutcomes = activeTrajectory?.outcomes ?? [];

  const resumeHistoryEntry = (entry: StudyHistoryEntry) => {
    if (entry.resourcePath?.startsWith("canvas:")) {
      const material = materials.find((item) => item.id === entry.resourcePath?.slice("canvas:".length));
      if (material) void openMaterial(material, entry.id);
      return;
    }
    if (entry.resourcePath) {
      const enrollment = activeEnrollments.find((item) => normalizeSubjectName(item.title) === normalizeSubjectName(entry.courseTitle ?? ""))
        ?? { id: entry.courseId ?? "local-study", title: entry.courseTitle ?? "Matéria local", status: "enrolled" };
      void openStudyDocument(entry.resourcePath, enrollment, entry.id);
    }
  };

  return (
    <div className="app-frame">
      <aside className="sidebar">
        <div className="brand"><span className="brand-mark">E</span><span><b>Estuda UVA</b><small>seu acervo acadêmico</small></span></div>
        <nav aria-label="Navegação principal">
          {([ ["inicio", "Estudar", "⌂"], ["materias", "Matérias", "▤"], ["historico", "Histórico", "◷"], ["agenda", "Agenda", "◷"], ["curriculos", "Acadêmico", "⌘"], ["acervo", "Dados e sync", "↔"] ] as [View, string, string][]).map(([key, label, icon]) => (
            <button key={key} className={`nav-item ${view === key ? "active" : ""}`} onClick={() => setView(key)}>
              <span aria-hidden="true">{icon}</span>{label}
              {key === "acervo" && !!localCatalog?.documents.length && <span className="nav-count">{localCatalog.documents.length}</span>}
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

          {view === "inicio" && <StudyHome
            enrollments={activeEnrollments}
            documents={localCatalog?.documents ?? []}
            materials={materials}
            history={studyHistory}
            loading={!academicCatalog && !academicCatalogError}
            indexing={studyIndexing}
            partial={studyIndexPartial}
            loadError={academicCatalogError}
            semester={activeTrajectory?.semester}
            onOpenSubject={openSubject}
            onContinue={resumeHistoryEntry}
            onOpenHistory={() => setView("historico")}
          />}

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

          {view === "historico" && <section className="content-width study-history-page">
            <PageIntro eyebrow="SEU PERCURSO DE ESTUDO" title="Histórico" copy="Aulas apresentadas e práticas concluídas, separadas dos resultados oficiais da UVA." />
            <div className="study-panel-toolbar"><span>{studyHistory.length} registro(s) local(is)</span><button className="button secondary" disabled={historyLoading} onClick={() => void refreshStudyHistory()}>{historyLoading ? "Atualizando…" : "Atualizar histórico ↻"}</button></div>
            <StudyHistory items={studyHistory} loading={historyLoading} onOpenLesson={resumeHistoryEntry} />
          </section>}

          {view === "materias" && (activeEnrollment ? <SubjectDetail
            enrollment={activeEnrollment}
            documents={documentsForEnrollment(activeEnrollment, localCatalog?.documents ?? [])}
            importedMaterials={materialsForEnrollment(activeEnrollment, materials)}
            modules={modules}
            canvasCourse={activeCourse}
            canvasStatus={status.state}
            canvasBusy={!!busy}
            choices={choices}
            selectedItem={selectedItem}
            outcomes={currentOutcomes.filter((outcome) => normalizeSubjectName(outcome.courseTitle) === normalizeSubjectName(activeEnrollment.title))}
            history={studyHistory}
            historyLoading={historyLoading}
            onBack={() => setActiveEnrollment(null)}
            onOpenDocument={(document) => void openStudyDocument(document, activeEnrollment)}
            onOpenImported={(material) => void openMaterial(material)}
            onConnectCanvas={() => void connectCanvas()}
            onRefreshCanvas={() => void refreshSubjectCanvas(activeEnrollment)}
            onInspectItem={(item) => void inspectItem(item)}
            onImportChoice={(choice, item) => void importChoice(choice, item)}
            onResumeHistory={resumeHistoryEntry}
          /> : <SubjectList
            enrollments={activeEnrollments}
            documents={localCatalog?.documents ?? []}
            materials={materials}
            loading={!academicCatalog && !academicCatalogError}
            indexing={studyIndexing}
            partial={studyIndexPartial}
            loadError={academicCatalogError}
            onOpenSubject={openSubject}
            capturePanel={<CanvasCapturePanel
              run={captureRun}
              items={captureItems}
              hasMoreItems={captureHasMore}
              limits={captureLimits}
              busy={!!busy || captureItemsLoading}
              error={error || undefined}
              canvasAuthenticated={status.state === "authenticated"}
              onStart={() => void startCanvasCapture()}
              onResume={() => void resumeCanvasCapture()}
              onCancel={() => void cancelCanvasCapture()}
              onConnect={() => void connectCanvas()}
              onRetry={(item) => void retryCanvasCaptureItem(item)}
              onLimitsChange={setCaptureLimits}
              onSaveLimits={() => void saveCanvasCaptureLimits()}
              onLoadMore={() => { if (captureRun) void loadCaptureItemsPage(captureRun.id, captureItems.length, false); }}
            />}
          />)}

          {view === "canvas" && <section className="content-width">
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

          {view === "curriculos" && <section className="content-width">
            <PageIntro eyebrow="UVA · TRAJETÓRIA ACADÊMICA" title="Currículos e disciplinas" copy="Consulte as duas trajetórias e seus registros separados. Disciplinas candidatas a conteúdo comum não representam equivalência nem aproveitamento confirmado." />
            {academicCatalogError && <div className="catalog-note" role="status">{academicCatalogError}</div>}
            <AcademicPortal status={portalStatus} capture={portalCapture} busy={!!busy} onOpen={openLyceum} onCheck={checkLyceum} onCapture={captureLyceum} />
            {academicCatalog && <AcademicCurricula catalog={academicCatalog} />}
          </section>}

          {view === "acervo" && <section className="content-width">
            <PageIntro eyebrow="MONITORAMENTO LOCAL" title="Acervo da pasta" copy="Consulte os documentos que já estão salvos no computador. A leitura é somente local e não exige login no Canvas." />
            <div className="data-tools-card"><div><p className="eyebrow">FONTES ACADÊMICAS</p><b>{status.state === "authenticated" ? "Canvas conectado" : "Canvas aguarda login"}</b><small>Login necessário apenas para atualizar dados do Canvas. Materiais locais continuam disponíveis.</small></div><div className="data-tools-actions"><button className="button secondary" onClick={() => setView("canvas")}>Abrir cursos e importação</button>{status.state === "authenticated" ? <button className="button primary" disabled={!!busy} onClick={() => void loadCourses()}>{busy === "courses" ? "Carregando…" : "Atualizar Canvas ↻"}</button> : <button className="button primary" disabled={!!busy} onClick={() => void connectCanvas()}>{busy === "canvas" ? "Abrindo…" : "Conectar Canvas"}</button>}<button className="text-button" onClick={() => void pickChromium()}>Escolher Chromium</button></div></div>
            <div className="library-location"><div className="folder-icon">▱</div><div><b>Pasta raiz do acervo</b><small>{localCatalog?.root ?? overview?.libraryPath ?? "Carregando…"}</small></div><button className="button primary" disabled={!!busy} onClick={() => void withBusy("local-refresh", async () => { await refreshLocalLibrary(); setNotice("SQLite, Markdown e JSON foram sincronizados."); })}>{busy === "local-refresh" ? "Sincronizando…" : "Sincronizar agora"}</button><button className="button secondary" disabled={!!busy} onClick={() => void pickLibrary()}>Escolher pasta</button></div>
            {localSync && <div className="catalog-sync-status" role="status"><b>Sincronização local</b><span>{localSync.scanned} arquivos · {localSync.created} novos · {localSync.updated} atualizados · {localSync.pendingOperations} pendência(s) · {localSync.conflicts} conflito(s)</span>{localSyncAt && <small>Última reconciliação: {new Date(localSyncAt).toLocaleString("pt-BR")}</small>}{localSync.errors.length > 0 && <small>{localSync.errors.length} arquivo(s) preservados com erro. Primeiro: {localSync.errors[0]}</small>}</div>}
            {repositoryStatus && <section className="repository-status"><div className="local-catalog-heading"><b>Repositório Git</b><small>{repositoryStatus.isRepository ? repositoryStatus.branch ? `branch ${repositoryStatus.branch}` : "repositório local" : "indisponível"}</small></div>{repositoryStatus.isRepository ? <><span>{repositoryStatus.trackedChanges.length} alteração(ões) rastreada(s) · {repositoryStatus.untrackedFiles.length} arquivo(s) novo(s) · {repositoryStatus.ignoredFiles.length} item(ns) ignorado(s)</span>{repositoryStatus.trackedChanges.slice(0, 8).map((path) => <small className="repository-path" key={`tracked-${path}`}>Alterado · {path}</small>)}{repositoryStatus.untrackedFiles.slice(0, 8).map((path) => <small className="repository-path" key={`untracked-${path}`}>Novo · {path}</small>)}{repositoryStatus.ignoredFiles.slice(0, 4).map((path) => <small className="repository-path muted" key={`ignored-${path}`}>Ignorado · {path}</small>)}{repositoryStatus.trackedChanges.length + repositoryStatus.untrackedFiles.length > 16 && <small>A lista mostra uma amostra; o Git continua sendo a fonte completa do status.</small>}</> : <span>{repositoryStatus.error ?? "Esta pasta não contém um repositório Git."}</span>}</section>}
            {syncConflicts.length > 0 && <section className="sync-conflicts"><div className="local-catalog-heading"><b>Conflitos para revisar</b><small>As versões divergentes foram preservadas.</small></div>{syncConflicts.map((conflict) => <details className="sync-conflict" key={conflict.id}><summary>{conflict.relativePath}{conflict.tombstone ? " · arquivo excluído" : ""}</summary><p>{conflict.tombstone ? "O arquivo foi removido da pasta, mas o conteúdo e o espelho continuam salvos. Escolha uma versão para restaurar as três representações." : "Escolha a versão que deve prevalecer nas três representações."}</p><div>{conflict.sqlite !== null && <button className="button secondary" disabled={!!busy} onClick={() => void resolveLocalConflict(conflict, conflict.sqlite!)}>{conflict.tombstone ? "Restaurar do SQLite" : "Usar SQLite"}</button>}{conflict.markdown !== null && <button className="button secondary" disabled={!!busy} onClick={() => void resolveLocalConflict(conflict, conflict.markdown!)}>{conflict.tombstone ? "Restaurar do Markdown" : "Usar arquivo"}</button>}{conflict.json !== null && <button className="button secondary" disabled={!!busy} onClick={() => void resolveLocalConflict(conflict, conflict.json!)}>{conflict.tombstone ? "Restaurar do JSON" : "Usar JSON"}</button>}</div></details>)}</section>}
            <form className="local-search" onSubmit={searchLocalLibrary}><input type="search" aria-label="Buscar nos arquivos locais" placeholder="Buscar em nomes e conteúdo dos arquivos…" value={localQuery} onChange={(event) => setLocalQuery(event.target.value)} /><button className="button primary" disabled={!!busy || !localQuery.trim()}>{busy === "local-search" ? "Buscando…" : "Buscar"}</button>{localSearch && <button type="button" className="text-button" onClick={() => { setLocalSearch(null); setLocalQuery(""); setLocalPage(0); }}>Limpar</button>}</form>
            {localCatalog?.unreadableDirectories ? <p className="catalog-note">Não foi possível listar {localCatalog.unreadableDirectories} pasta(s) sem permissão de leitura.</p> : null}
            {localCatalog?.unreadableFiles ? <p className="catalog-note">Não foi possível ler os metadados de {localCatalog.unreadableFiles} arquivo(s) compatível(is).</p> : null}
            {localCatalog?.truncated && <p className="catalog-note">O limite de 10.000 documentos compatíveis ou a profundidade de pastas foi atingido; parte da pasta ainda não foi indexada.</p>}
            {localSearch?.unreadableFiles ? <p className="catalog-note">{localSearch.unreadableFiles} arquivo(s) não puderam ser pesquisados; os demais resultados continuam disponíveis.</p> : null}
            {localSearch?.truncated && <p className="catalog-note">A busca encontrou mais resultados; mostrando os primeiros 100.</p>}
            <div className="local-catalog-heading"><b>{localSearch ? `${localSearch.results.length} resultado(s) na busca` : localCatalog ? `${localCatalog.documents.length} documento(s) na pasta` : "Lendo a pasta…"}</b><small>Markdown · TXT · ALG · CSV · JSON · PDF · DOCX · Pages</small></div>
            {localDocumentsToShow.length ? <div className="material-list large">{visibleLocalDocuments.map((result) => <LocalDocumentRow key={result.document.relativePath} document={result.document} excerpt={result.excerpt} onOpen={() => void openLocalDocument(result.document)} busy={!!busy} />)}</div> : <EmptyCard title={localSearch ? "Nenhum resultado encontrado" : localCatalog ? "Nenhum documento compatível encontrado" : "Lendo a pasta do acervo…"} copy={localSearch ? "Tente outro termo; a busca verifica nomes e texto dos documentos compatíveis." : "Escolha a pasta em que estão seus materiais acadêmicos."} action="Escolher pasta" onClick={() => void pickLibrary()} />}
            {localDocumentsToShow.length > localPageSize && <nav className="catalog-pagination" aria-label="Páginas do acervo"><button type="button" className="button secondary" disabled={localPage === 0} onClick={() => setLocalPage((page) => Math.max(0, page - 1))}>Anterior</button><span>Página {localPage + 1} de {localPageCount}</span><button type="button" className="button secondary" disabled={localPage + 1 >= localPageCount} onClick={() => setLocalPage((page) => Math.min(localPageCount - 1, page + 1))}>Próxima</button></nav>}
            {localDocument && <article className="lesson-card local-document-viewer"><div className="lesson-toolbar"><div><span className="lesson-badge">LEITURA LOCAL</span><p>{localDocument.document.relativePath} · {localDocument.document.format.toUpperCase()}</p></div>{["markdown", "plain_text", "alg", "csv", "json"].includes(localDocument.document.format) && <button className="text-button" onClick={() => { setEditingLocal((editing) => !editing); setLocalEditContent(localDocument.markdown); }}>{editingLocal ? "Cancelar edição" : "Editar conteúdo"}</button>}<button className="text-button" onClick={() => setLocalDocument(null)}>Fechar <span>×</span></button></div>{localDocument.warnings.map((warning) => <p className="catalog-note" key={warning}>{warning}</p>)}{editingLocal ? <div className="local-edit"><textarea aria-label="Editar conteúdo sincronizado" value={localEditContent} onChange={(event) => setLocalEditContent(event.target.value)} /><button className="button primary" disabled={!!busy} onClick={() => void saveLocalDocument()}>{busy === "local-save" ? "Salvando…" : "Salvar nas três representações"}</button></div> : <div className="markdown-body"><ReactMarkdown remarkPlugins={[remarkGfm]} components={{ img: ({ src, alt }) => <LocalMarkdownImage documentPath={localDocument.document.relativePath} src={src} alt={alt} />, a: ({ href, children }) => {
              const target = href ? resolveLocalAssetPath(localDocument.document.relativePath, href) : null;
              if (target && /\.(?:md|markdown|txt|alg|csv|json|pdf|docx|pages)$/i.test(target)) return <a href={href} onClick={(event) => { event.preventDefault(); void openLocalDocument(target); }}>{children}</a>;
              if (href && /^(?:https?:|mailto:)/i.test(href)) return <a href={href} target={/^https?:/i.test(href) ? "_blank" : undefined} rel="noreferrer">{children}</a>;
              return <span>{children}</span>;
            } }}>{localDocument.markdown}</ReactMarkdown></div>}</article>}
            {materials.length > 0 && <><div className="section-heading compact"><div><p className="eyebrow">AULAS GUIADAS</p><h2>Materiais importados pelo app</h2></div></div><div className="material-list large">{materials.map((material) => <MaterialRow key={material.id} material={material} onOpen={() => void openMaterial(material)} busy={!!busy} />)}</div></>}
          </section>}

          {view === "estudar" && <section className="study-content">
            {studySession && lesson ? <>
              <LessonPresenter
                title={activeMaterial?.title ?? localDocument?.document.title ?? studySession.materialTitle}
                courseTitle={activeEnrollment?.title ?? activeMaterial?.courseTitle ?? studySession.courseTitle}
                markdown={lesson}
                documentPath={studyDocumentPath}
                session={studySession}
                onBack={() => setView("materias")}
                onSessionChanged={(session) => { setStudySession(session); void refreshStudyHistory(); }}
                onRelatedDocument={(path) => {
                  const doc = localCatalog?.documents.find((item) => item.relativePath === path);
                  const subject = doc?.course ? activeEnrollments.find((item) => normalizeSubjectName(item.title) === normalizeSubjectName(doc.course!)) : activeEnrollment;
                  if (subject) void openStudyDocument(path, subject);
                  else void openLocalDocument(path);
                }}
                onCompleted={() => { void refreshStudyHistory(); setNotice("Aula concluída e registrada no histórico local."); }}
              />
              {activeMaterial && <aside className="practice-card lesson-practice-card"><div className="practice-top"><span className="practice-icon">✳</span><p className="eyebrow">PRÁTICA DO MATERIAL IMPORTADO</p><h3>Quer testar o que aprendeu?</h3><p>Esta prática fica separada da aula interativa. O resultado é salvo no histórico local e não altera suas notas oficiais.</p></div>
                {prompts.length === 0 ? <div className="no-questions">Este texto não gerou perguntas automáticas confiáveis. Você pode usar a aula e conferir as páginas da fonte.</div> : <div className="question-list">{prompts.map((prompt) => <label className="question" key={prompt.position}><span>{prompt.prompt}</span><small>Fonte: página {prompt.sourcePage}</small><textarea rows={3} value={answers[prompt.position] ?? ""} onChange={(event) => setAnswers((current) => ({ ...current, [prompt.position]: event.target.value }))} placeholder="Escreva sua resposta…" disabled={!!result} /></label>)}<button className="button primary full" disabled={!!busy || !!result} onClick={() => void submitAnswers()}>{busy === "practice" ? "Conferindo…" : "Conferir respostas"}<span>→</span></button></div>}
                {result && <div className="score-result" role="status"><div className="score-circle">{result.percent}<small>%</small></div><div><b>{result.correct} de {result.total} acerto(s)</b><p>Prática salva no histórico local.</p></div><button className="text-button" onClick={() => { setAnswers({}); setResult(null); }}>Tentar novamente</button>{result.answers.map((answer) => <div key={answer.position} className={`answer-note ${answer.correct ? "right" : "wrong"}`}><b>{answer.correct ? "Correto" : "Resposta de referência"} · pág. {answer.sourcePage}</b><p>{answer.expected}</p></div>)}</div>}
              </aside>}
            </> : <div className="content-width"><EmptyCard title="Nenhuma aula aberta" copy="Entre em uma matéria e escolha uma aula para apresentar." action="Ver matérias" onClick={() => setView("materias")} /></div>}
          </section>}
        </div>
      </main>
    </div>
  );
}

function viewTitle(view: View) { return ({ inicio: "Estudar", materias: "Matérias", historico: "Histórico", curriculos: "Acadêmico", agenda: "Agenda", acervo: "Dados e sincronização", estudar: "Aula guiada", canvas: "Canvas e importação" })[view]; }
function statusLabel(state: CanvasState) { return ({ disconnected: "Canvas · desconectado", waiting_for_login: "Canvas · aguardando login", authenticated: "Canvas · conectado", session_expired: "Canvas · sessão expirada", browser_error: "Canvas · erro no navegador" })[state]; }
function PageIntro({ eyebrow, title, copy }: { eyebrow: string; title: string; copy: string }) { return <div className="page-intro"><p className="eyebrow">{eyebrow}</p><h1>{title}</h1><p className="lead">{copy}</p></div>; }
function EmptyCard({ title, copy, action, onClick }: { title: string; copy: string; action: string; onClick: () => void }) { return <div className="empty-card"><span className="empty-icon">▤</span><h3>{title}</h3><p>{copy}</p><button className="button secondary" onClick={onClick}>{action}<span>→</span></button></div>; }

function AcademicCurricula({ catalog }: { catalog: AcademicCatalog }) {
  const [query, setQuery] = useState("");
  const needle = query.trim().toLocaleLowerCase("pt-BR");
  const disciplinesById = new Map(catalog.disciplines.map((discipline) => [discipline.id, discipline]));
  const sharedSubjects = sharedAcademicSubjects(catalog);
  return <div className="academic-curricula">
    <label className="curriculum-search"><span>Buscar disciplina ou código</span><input type="search" value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Ex.: cálculo, E10300048…" /></label>
    <div className="curriculum-grid">{catalog.curriculums.map((curriculum) => {
      const trajectory = catalog.trajectories.find((item) => item.programId === curriculum.programId);
      const offerings = curriculum.offerings.filter((offering) => {
        const canonicalTitle = offering.canonicalDisciplineId ? disciplinesById.get(offering.canonicalDisciplineId)?.title ?? "" : "";
        return `${offering.title} ${offering.code} ${canonicalTitle}`.toLocaleLowerCase("pt-BR").includes(needle);
      });
      const pending = curriculum.offerings.filter((offering) => offering.status === "pending");
      const pendingHours = pending.reduce((sum, offering) => sum + offering.workloadHours, 0);
      const enrollments = trajectory?.currentEnrollments ?? [];
      const outcomes = trajectory?.outcomes ?? [];
      const priorCanvasCourses = trajectory?.priorCanvasCourses ?? [];
      return <article className="curriculum-card" key={curriculum.id}>
        <header className="curriculum-header"><div><p className="eyebrow">{curriculum.programCode} · {curriculum.curriculumVersion}</p><h2>{curriculum.programTitle}</h2></div><span className={`soft-tag ${trajectory?.status === "evadido" ? "muted" : ""}`}>{trajectory?.status ?? curriculum.trajectoryStatus}</span></header>
        <div className="curriculum-stats"><div><b>{pending.length}</b><small>pendências listadas</small></div><div><b>{pendingHours.toLocaleString("pt-BR")} h</b><small>soma das pendências exibidas</small></div><div><b>{enrollments.length}</b><small>matrículas atuais</small></div></div>
        {trajectory?.progress && <div className="trajectory-progress"><b>Progresso informado pela origem: {trajectory.progress.reportedCompletionPercent ?? "—"}%</b><small>{trajectory.progress.completedHours ?? 0} h concluídas · {trajectory.progress.enrolledHours ?? 0} h matriculadas · {trajectory.progress.remainingHours ?? 0} h restantes de {trajectory.progress.totalHours ?? "—"} h</small>{trajectory.progress.note && <p>{trajectory.progress.note}</p>}</div>}
        {pending.length > 0 && <details className="pending-details"><summary>Disciplinas pendentes · {pending.length} · sem ordem oficial registrada</summary><div className="offering-list">{pending.map((offering) => <div className="offering-row" key={offering.id}><div><b>{offering.title}</b><small>{offering.code} · {offering.workloadHours} h</small></div><span className="soft-tag">Pendente</span></div>)}</div><small>O portal não registra nesta captura uma sequência futura ou pré-requisitos; a lista mostra as pendências do currículo.</small></details>}
        {enrollments.length > 0 && <div className="enrollment-list"><b>Matrículas atuais{trajectory?.semester ? ` · ${trajectory.semester}` : ""}</b>{enrollments.map((enrollment) => <div key={enrollment.id}><span>{enrollment.title}</span><small>{[enrollment.section, enrollment.startDate].filter(Boolean).join(" · ")}</small></div>)}</div>}
        {(outcomes.length > 0 || priorCanvasCourses.length > 0 || (trajectory?.limitations?.length ?? 0) > 0) && <section className="academic-outcomes">
          <div className="academic-outcomes-heading"><b>Notas e frequência</b>{trajectory?.source?.observedAt && <small>Fonte: {trajectory.source.file ?? "registro local"} · {formatAcademicDate(trajectory.source.observedAt)}</small>}</div>
          {outcomes.map((outcome) => <article className="outcome-card" key={outcome.courseTitle}>
            <header className="outcome-header"><b>{outcome.courseTitle}</b><span className="soft-tag">{outcome.status === "in_progress" ? "Em andamento" : outcome.status}</span></header>
            <div className="outcome-metrics">
              <div><b>{outcome.lyceumAverage == null ? "—" : outcome.lyceumAverage.toLocaleString("pt-BR")}</b><small>Média do Lyceum</small></div>
              <div><b>{outcome.absences == null ? "—" : outcome.absences}</b><small>Faltas</small></div>
              <div><b>{outcome.attendancePercent == null ? "—" : `${outcome.attendancePercent.toLocaleString("pt-BR")}%`}</b><small>Frequência</small></div>
            </div>
            {!!outcome.lyceumGrades?.length && <details className="grade-details"><summary>{outcome.lyceumGrades.length} avaliação(ões) no Lyceum</summary><div className="grade-list">{outcome.lyceumGrades.map((grade, index) => <div className="grade-row" key={`${grade.assessment}-${index}`}><span>{grade.assessment}</span><b>{grade.score == null ? "Sem nota" : `${grade.score.toLocaleString("pt-BR")} / ${grade.scale}`}</b><small>{grade.date ? formatAcademicDate(grade.date) : "Data não exibida"}</small></div>)}</div></details>}
            {outcome.canvasAvaTotals && <p className="canvas-grade-note">Canvas · {outcome.canvasCourseCode}: {outcome.canvasAvaTotals.earnedPoints} / {outcome.canvasAvaTotals.possiblePoints} pontos ({outcome.canvasAvaTotals.percent.toLocaleString("pt-BR")}%) nas atividades AVA; esse total não é a média da disciplina.</p>}
          </article>)}
          {!!priorCanvasCourses.length && <div className="prior-canvas"><b>Atividades nas inscrições anteriores do Canvas</b>{priorCanvasCourses.map((course) => <div className="prior-canvas-row" key={course.canvasCode}><span>{course.courseTitle}<small>{course.canvasCode} · {course.earnedPoints} / {course.possiblePoints} pontos</small></span><b>{course.percent.toLocaleString("pt-BR")}%</b></div>)}<small>Totais de atividades AVA disponíveis no Canvas; não representam médias finais oficiais do Lyceum.</small></div>}
          {trajectory?.limitations?.map((limitation) => <p className="catalog-note" key={limitation}>{limitation}</p>)}
        </section>}
        <details className="offering-details"><summary>Ver {offerings.length} disciplina(s) listada(s)</summary>{offerings.length ? <div className="offering-list">{offerings.map((offering) => <div className="offering-row" key={offering.id}><div><b>{offering.title}</b><small>{offering.code} · {offering.workloadHours} h</small></div><span className="soft-tag">{offering.status}</span></div>)}</div> : <p className="catalog-note">Nenhuma disciplina corresponde à busca.</p>}</details>
      </article>;
    })}</div>
    <section className="curriculum-relations"><div className="local-catalog-heading"><b>Possíveis matérias em comum</b><small>{sharedSubjects.length} candidata(s) · situação independente em cada curso</small></div>{sharedSubjects.length ? sharedSubjects.map((subject) => <article className="relation-row" key={subject.id}><div><b>{subject.title}</b><p>Assunto ligado explicitamente às ofertas abaixo. Isso não confirma equivalência, aproveitamento ou conclusão.</p><div className="offering-list">{subject.offerings.map(({ curriculumId, programCode, programTitle, offering }) => <div className="offering-row" key={`${curriculumId}-${offering.id}`}><div><b>{programCode} · {programTitle}</b><small>{offering.code} · {offering.workloadHours} h · {offering.status}</small></div></div>)}</div></div><span className="soft-tag">Candidata</span></article>) : <p className="catalog-note">Nenhuma disciplina está ligada explicitamente a ofertas de mais de um curso.</p>}</section>
    <section className="curriculum-relations"><div className="local-catalog-heading"><b>Relações entre os currículos</b><small>{catalog.relations.length} registro(s) documentado(s)</small></div>{catalog.relations.map((relation) => <article className="relation-row" key={relation.id}><div><b>{relation.type === "same_subject_candidate" ? "Candidata a mesma disciplina" : "Sobreposição parcial candidata"}</b><p>{relation.note ?? "Relação registrada para revisão."}</p></div><span className="soft-tag">{relation.equivalenceConfirmed ? "Equivalência confirmada" : "Sem equivalência confirmada"}</span></article>)}</section>
    <p className="catalog-note">Os totais refletem as linhas presentes nos registros locais. Divergências informadas pela origem continuam identificadas; horas agregadas não foram atribuídas a disciplinas específicas.</p>
  </div>;
}

function formatAcademicDate(value: string) {
  const match = /^(\d{4})-(\d{2})-(\d{2})$/.exec(value);
  return match ? `${match[3]}/${match[2]}/${match[1]}` : value;
}

function LocalMarkdownImage({ documentPath, src, alt }: { documentPath: string; src?: string; alt?: string }) {
  const [url, setUrl] = useState("");
  const [unavailable, setUnavailable] = useState(false);
  useEffect(() => {
    let disposed = false;
    let objectUrl = "";
    setUrl("");
    setUnavailable(false);
    const relativePath = src ? resolveLocalAssetPath(documentPath, src) : null;
    if (!relativePath) {
      setUnavailable(true);
      return () => { disposed = true; };
    }
    void invoke<number[]>("read_local_asset", { relativePath }).then((bytes) => {
      if (disposed) return;
      const extension = relativePath.split(".").pop()?.toLowerCase();
      const mime = extension === "png" ? "image/png" : extension === "webp" ? "image/webp" : "image/jpeg";
      const data = Uint8Array.from(bytes);
      objectUrl = URL.createObjectURL(new Blob([data.buffer as ArrayBuffer], { type: mime }));
      setUrl(objectUrl);
    }).catch(() => {
      if (!disposed) setUnavailable(true);
    });
    return () => {
      disposed = true;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [documentPath, src]);
  if (url) return <img src={url} alt={alt ?? "Imagem do documento"} loading="lazy" />;
  if (unavailable) return <span className="local-image-unavailable" role="note">Imagem local indisponível: {alt || src || "sem descrição"}</span>;
  return <span className="local-image-loading" role="status">Carregando imagem local…</span>;
}

function resolveLocalAssetPath(documentPath: string, source: string): string | null {
  if (/^(?:[a-z][a-z\d+.-]*:|\/\/|\/)/i.test(source)) return null;
  let decoded: string;
  try { decoded = decodeURIComponent(source.split(/[?#]/, 1)[0]).replace(/\\/g, "/"); }
  catch { return null; }
  const parts = documentPath.split("/").slice(0, -1);
  for (const part of decoded.split("/")) {
    if (!part || part === ".") continue;
    if (part === "..") {
      if (!parts.length) return null;
      parts.pop();
    } else {
      parts.push(part);
    }
  }
  return parts.length ? parts.join("/") : null;
}

function MaterialRow({ material, onOpen, busy }: { material: Material; onOpen: () => void; busy: boolean }) { return <button className="material-row" onClick={onOpen} disabled={busy}><span className="material-cover"><span>MD</span></span><span className="material-info"><b>{material.title}</b><small>{material.courseTitle} <i>·</i> {material.moduleTitle}</small><small>{material.pages} páginas · Markdown</small></span><span className="material-open">Abrir aula <b>→</b></span></button>; }
function LocalDocumentRow({ document, excerpt, onOpen, busy }: { document: LocalDocument; excerpt?: string; onOpen: () => void; busy: boolean }) {
  return <button className="material-row local-document-row" onClick={onOpen} disabled={busy}>
    <span className="material-cover"><span>{document.format.toUpperCase().slice(0, 3)}</span></span>
    <span className="material-info"><b>{document.title}</b><small>{document.relativePath}</small>{excerpt && <small className="search-excerpt">{excerpt}</small>}</span>
    <span className="material-open">{formatBytes(document.sizeBytes)} <b>→</b></span>
  </button>;
}
function formatBytes(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
function CalendarRow({ event }: { event: CalendarEvent }) {
  return <article className="calendar-event">
    <div className="calendar-event-date"><span>◷</span><time dateTime={event.startsAt}>{event.startsAt}</time></div>
    <div className="calendar-event-copy"><b>{event.title}</b><small>{event.courseTitle ?? event.sourceLabel}</small></div>
    <a className="calendar-event-source" href={event.sourceUrl} target="_blank" rel="noreferrer">Abrir fonte ↗</a>
  </article>;
}
