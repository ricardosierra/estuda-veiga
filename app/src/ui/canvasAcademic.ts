// Tipos e utilitários da leitura acadêmica do Canvas (notas, fóruns, avisos, Caixa de Entrada)
// e da nota final ponderada. Espelham `canvas_api.rs` e `grading.rs`.

export type CanvasAcademicOptions = {
  grades: boolean;
  forums: boolean;
  announcements: boolean;
  inbox: boolean;
};

export type TextEntry = { author: string | null; createdAt: string | null; text: string; depth: number };

export type GradeItem = {
  assignmentId: number;
  name: string;
  pointsPossible: number | null;
  dueAt: string | null;
  score: number | null;
  grade: string | null;
  workflowState: string | null;
  submittedAt: string | null;
  gradedAt: string | null;
  late: boolean;
  missing: boolean;
  comments: TextEntry[];
  commentsChecked: boolean;
  sourceUrl: string | null;
};

export type ForumTopic = {
  topicId: number;
  title: string;
  author: string | null;
  postedAt: string | null;
  lastReplyAt: string | null;
  replyCount: number | null;
  graded: boolean;
  locked: boolean;
  opening: string;
  entries: TextEntry[];
  entriesComplete: boolean;
  sourceUrl: string | null;
};

export type Announcement = {
  topicId: number;
  title: string;
  author: string | null;
  postedAt: string | null;
  text: string;
  sourceUrl: string | null;
};

export type Conversation = {
  conversationId: number;
  subject: string;
  context: string | null;
  lastMessageAt: string | null;
  workflowState: string | null;
  participants: string[];
  messages: TextEntry[];
  messagesChecked: boolean;
};

export type CourseAcademic = {
  courseId: string;
  title: string;
  kind: "current" | "past";
  sourceUrl: string;
  grades: GradeItem[];
  forums: ForumTopic[];
  announcements: Announcement[];
  warnings: string[];
};

export type CanvasAcademicCapture = {
  capturedAt: string;
  options: CanvasAcademicOptions;
  courses: CourseAcademic[];
  conversations: Conversation[];
  warnings: string[];
  complete: boolean;
  jsonPath: string | null;
  markdownPath: string | null;
};

export const defaultCanvasAcademicOptions: CanvasAcademicOptions = {
  grades: true,
  forums: true,
  announcements: true,
  inbox: false,
};

export const canvasAcademicOptionLabels: Record<keyof CanvasAcademicOptions, string> = {
  grades: "Notas e feedback das AVAs",
  forums: "Fóruns",
  announcements: "Avisos",
  inbox: "Caixa de Entrada",
};

export function hasAnyOption(options: CanvasAcademicOptions): boolean {
  return options.grades || options.forums || options.announcements || options.inbox;
}

export function captureCounts(capture: CanvasAcademicCapture) {
  let grades = 0;
  let graded = 0;
  let feedback = 0;
  let forums = 0;
  let forumEntries = 0;
  let announcements = 0;
  for (const course of capture.courses) {
    grades += course.grades.length;
    graded += course.grades.filter((item) => item.score !== null || item.grade !== null).length;
    feedback += course.grades.filter((item) => item.comments.length > 0).length;
    forums += course.forums.length;
    forumEntries += course.forums.reduce((sum, topic) => sum + topic.entries.length, 0);
    announcements += course.announcements.length;
  }
  return {
    courses: capture.courses.length,
    grades,
    graded,
    feedback,
    forums,
    forumEntries,
    announcements,
    conversations: capture.conversations.length,
  };
}

function decimal(value: number): string {
  return value.toLocaleString("pt-BR", { maximumFractionDigits: 2 });
}

export function formatGrade(item: GradeItem): string {
  if (item.grade !== null && item.grade !== "") {
    return item.pointsPossible !== null ? `${item.grade} / ${decimal(item.pointsPossible)}` : item.grade;
  }
  if (item.score !== null) {
    return item.pointsPossible !== null ? `${decimal(item.score)} / ${decimal(item.pointsPossible)}` : decimal(item.score);
  }
  if (item.missing) return "Pendente";
  return "Sem nota";
}

export function formatDate(value: string | null): string {
  if (!value) return "—";
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString("pt-BR");
}

// --- Nota final ponderada -------------------------------------------------

export type DisciplineKind = "regular" | "special";
export type GradeStatus = "approved" | "failed" | "incomplete";

export type FinalGradeInput = {
  kind: DisciplineKind;
  a1: number | null;
  a2: number | null;
  a3: number | null;
  a4: number | null;
};

export type FinalGradeResult = {
  status: GradeStatus;
  finalGrade: number | null;
  passingGrade: number;
  examUsed: string | null;
  neededExamGrade: number | null;
  formula: string;
  notes: string[];
};

export const gradeStatusLabel: Record<GradeStatus, string> = {
  approved: "Aprovação estimada",
  failed: "Reprovação estimada",
  incomplete: "Faltam notas",
};

/** "8,5" → 8.5; vazio → null; texto inválido ou fora de 0–10 → erro. */
export function parseGradeField(label: string, raw: string): number | null {
  const text = raw.trim().replace(",", ".");
  if (text === "") return null;
  if (!/^\d{1,2}(\.\d{1,2})?$/.test(text)) throw new Error(`${label}: use um número como 7,5.`);
  const value = Number(text);
  if (value < 0 || value > 10) throw new Error(`${label} deve estar entre 0 e 10.`);
  return value;
}

export function buildFinalGradeInput(
  kind: DisciplineKind,
  fields: { a1: string; a2: string; a3: string; a4: string },
): FinalGradeInput {
  return {
    kind,
    a1: parseGradeField("A1", fields.a1),
    a2: kind === "regular" ? parseGradeField("A2", fields.a2) : null,
    a3: kind === "regular" ? parseGradeField("A3", fields.a3) : null,
    a4: kind === "special" ? parseGradeField("A4", fields.a4) : null,
  };
}

export function formatGradeValue(value: number | null): string {
  return value === null ? "—" : value.toLocaleString("pt-BR", { minimumFractionDigits: 1, maximumFractionDigits: 2 });
}
