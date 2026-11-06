export type CaptureRunStatus = "running" | "waiting_for_login" | "cancelled" | "partial" | "completed" | "failed";
export type CaptureItemStatus = "pending" | "running" | "captured" | "reused" | "ignored" | "failed" | "needs_review";

export type CaptureCounts = {
  coursesTotal: number;
  coursesDone: number;
  itemsTotal: number;
  captured: number;
  reused: number;
  ignored: number;
  failed: number;
  needsReview: number;
  pending: number;
};

export type CaptureRun = {
  id: string;
  scope: string;
  limitsSnapshot: CanvasCaptureLimits;
  status: CaptureRunStatus;
  counts: CaptureCounts;
  currentCourseId: string | null;
  currentCourseTitle: string | null;
  currentModuleTitle: string | null;
  currentItemTitle: string | null;
  warning: string | null;
  startedAt: string;
  updatedAt: string;
  finishedAt: string | null;
};

export type CaptureItem = {
  position: number;
  referenceKey: string;
  contentKey: string | null;
  canvasItemId: string | null;
  courseId: string;
  courseTitle: string;
  moduleTitle: string;
  title: string;
  sourceKind: "pdf" | "canvas_page" | "unsupported";
  sourceUrl: string;
  status: CaptureItemStatus;
  retryable: boolean;
  attempts: number;
  materialId: string | null;
  message: string | null;
  updatedAt: string;
};

export type CanvasCaptureLimits = {
  maxCoursePages: number;
  maxCourses: number;
  maxItems: number;
  maxPdfBytes: number;
  maxRedirects: number;
  itemTimeoutSeconds: number;
};

export type CourseCaptureGroup = {
  courseId: string;
  courseTitle: string;
  items: CaptureItem[];
};

const CAPTURE_LIMITS: { key: keyof CanvasCaptureLimits; min: number; max: number }[] = [
  { key: "maxCoursePages", min: 1, max: 100 },
  { key: "maxCourses", min: 1, max: 500 },
  { key: "maxItems", min: 1, max: 20_000 },
  { key: "maxPdfBytes", min: 1, max: 60 * 1024 * 1024 },
  { key: "maxRedirects", min: 0, max: 4 },
  { key: "itemTimeoutSeconds", min: 5, max: 120 },
];

export function validateCanvasCaptureLimits(limits: CanvasCaptureLimits): string[] {
  return CAPTURE_LIMITS.flatMap(({ key, min, max }) => {
    const value = limits[key];
    return Number.isInteger(value) && value >= min && value <= max
      ? []
      : [`${key} deve ficar entre ${min} e ${max}`];
  });
}

export function captureRunActions(run: CaptureRun | null): { canStart: boolean; canResume: boolean; canCancel: boolean } {
  return {
    canStart: run?.status !== "running",
    canResume: !!run && ["waiting_for_login", "cancelled", "partial", "failed"].includes(run.status),
    canCancel: run?.status === "running",
  };
}

const RUN_LABELS: Record<CaptureRunStatus, string> = {
  running: "Captura em andamento",
  waiting_for_login: "Aguardando login do Canvas",
  cancelled: "Captura cancelada",
  partial: "Captura parcial",
  completed: "Captura concluída",
  failed: "Captura interrompida",
};

export function captureProgressLabel(run: CaptureRun): string {
  const { counts } = run;
  if (run.status === "running" && counts.itemsTotal > 0) {
    return `${counts.captured + counts.reused + counts.ignored + counts.failed + counts.needsReview} de ${counts.itemsTotal} itens processados`;
  }
  return RUN_LABELS[run.status];
}

export function captureItemStatusLabel(status: CaptureItemStatus): string {
  switch (status) {
    case "pending": return "Na fila";
    case "running": return "Lendo";
    case "captured": return "Capturado";
    case "reused": return "Já disponível localmente";
    case "ignored": return "Ignorado com motivo";
    case "failed": return "Falhou";
    case "needs_review": return "Precisa de revisão";
  }
}

export function capturePrivacyCopy(): string {
  return "A captura apenas lê materiais publicados. O acesso pode aparecer no histórico comum do Canvas; ele não registra presença, conclusão de aula, notas ou entregas.";
}

export function groupCaptureItemsByCourse(items: CaptureItem[]): CourseCaptureGroup[] {
  const groups = new Map<string, CourseCaptureGroup>();
  for (const item of [...items].sort((left, right) => left.position - right.position)) {
    let group = groups.get(item.courseId);
    if (!group) {
      group = { courseId: item.courseId, courseTitle: item.courseTitle, items: [] };
      groups.set(item.courseId, group);
    }
    group.items.push(item);
  }
  return [...groups.values()];
}
