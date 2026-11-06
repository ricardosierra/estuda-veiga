export type ActiveEnrollment = {
  id: string;
  title: string;
  section?: string;
  startDate?: string;
  status: string;
  semester?: string;
  instructorAsDisplayed?: string;
};

export type SubjectSummaryRecord = {
  id: string;
  courseId: string;
  courseTitle: string;
  semester: string;
  sourceHash: string;
  sourceManifestJson: string;
  shortText: string;
  longText: string;
  model: string;
  inputTokens: number;
  outputTokens: number;
  createdAt: string;
};

export type SubjectSummarySource = {
  relativePath: string;
  title: string;
  format: string;
  sizeBytes: number;
};

export type SubjectSummarySnapshot = {
  sources: SubjectSummarySource[];
  currentSourceHash: string;
  latestSummary: SubjectSummaryRecord | null;
};

export type StudyDocument = {
  relativePath: string;
  title: string;
  format: string;
  sizeBytes: number;
  modifiedAt?: number | null;
  course?: string | null;
  semester?: string | null;
  category?: string;
  readStatus?: string;
  readReason?: string | null;
};

export type ImportedStudyMaterial = {
  id: string;
  courseId: string;
  courseTitle: string;
  semester?: string | null;
  moduleTitle: string;
  title: string;
  path: string;
  lessonPath: string;
  origin: string;
  sourceKind?: "pdf" | "canvas_page" | string;
  pages: number;
  importedAt: string;
  associations?: MaterialAssociation[];
};

export type MaterialAssociation = {
  materialId: string;
  courseId: string;
  courseTitle: string;
  moduleTitle: string;
  itemTitle: string;
  semester?: string | null;
};

export type LessonSession = {
  id: string;
  resourcePath: string;
  courseId: string | null;
  courseTitle: string;
  materialTitle: string;
  sectionIndex: number;
  sectionCount: number;
  startedAt: string;
  updatedAt: string;
  completedAt: string | null;
};

export type LessonSection = {
  index: number;
  title: string;
  markdown: string;
  sourcePage?: number;
};

export type TutorSectionInput = Pick<LessonSection, "index" | "title" | "markdown" | "sourcePage">;

export type TutorActivity = {
  id: string;
  sessionId: string;
  sectionIndex: number;
  kind: "verifiable" | "self_assessment";
  prompt: string;
  referenceAnswer?: string | null;
  sourcePage?: number | null;
  sourceHeading: string;
  sourceExcerpt: string;
  sourceSha256: string;
};

export type TutorAssessment = {
  status: "correct" | "self_assessed" | "pending";
  correct?: boolean;
};

export type LessonInteraction = {
  id: string;
  sessionId: string;
  activityId: string;
  sectionIndex: number;
  kind: "verifiable" | "self_assessment";
  prompt: string;
  response: string;
  assessment: TutorAssessment;
  sourcePage?: number | null;
  sourceHeading: string;
  sourceExcerpt: string;
  sourceSha256: string;
  attemptedAt: string;
  sourceChanged?: boolean;
};

export type TutorFeedback = {
  status: TutorAssessment["status"];
  tone: "success" | "review" | "neutral";
  label: string;
  referenceAnswer?: string;
  showReference: boolean;
};

export function tutorSectionInput(section: LessonSection): TutorSectionInput {
  return {
    index: section.index,
    title: section.title,
    markdown: section.markdown,
    ...(section.sourcePage === undefined ? {} : { sourcePage: section.sourcePage }),
  };
}

export function buildTutorFeedback(
  activity: TutorActivity,
  assessment: TutorAssessment | null,
  hasAttempt: boolean,
): TutorFeedback | null {
  if (!assessment || !hasAttempt) return null;
  if (assessment.status === "pending") {
    return { status: "pending", tone: "neutral", label: "Digite uma resposta antes de avaliar.", showReference: false };
  }
  if (assessment.status === "correct") {
    return {
      status: "correct",
      tone: "success",
      label: "Resposta correspondente à referência do material.",
      ...(activity.referenceAnswer ? { referenceAnswer: activity.referenceAnswer } : {}),
      showReference: !!activity.referenceAnswer,
    };
  }
  const openQuestion = activity.kind === "self_assessment";
  return {
    status: "self_assessed",
    tone: "review",
    label: openQuestion ? "Releia o trecho e autoavalie sua explicação." : "Compare sua resposta com o trecho de referência.",
    ...(activity.referenceAnswer ? { referenceAnswer: activity.referenceAnswer } : {}),
    showReference: !!activity.referenceAnswer,
  };
}

export async function getOrBeginLessonSession(
  sessionId: string | undefined,
  begin: () => Promise<LessonSession>,
  get: (id: string) => Promise<LessonSession | null>,
): Promise<LessonSession> {
  if (!sessionId) return begin();
  const existing = await get(sessionId);
  if (!existing) throw new Error("A sessão salva não está mais disponível para revisão.");
  return existing;
}

export async function submitTutorAnswer(
  activity: TutorActivity,
  response: string,
  commands: {
    assess: (activity: TutorActivity, response: string) => Promise<TutorAssessment>;
    save: (sessionId: string, activity: TutorActivity, response: string, assessment: TutorAssessment) => Promise<LessonInteraction>;
  },
): Promise<{ assessment: TutorAssessment; interaction: LessonInteraction }> {
  const assessment = await commands.assess(activity, response);
  const interaction = await commands.save(activity.sessionId, activity, response, assessment);
  return { assessment, interaction };
}

export type StudyHistoryEntry = {
  id: string;
  kind: "lesson" | "practice";
  courseId: string | null;
  courseTitle: string | null;
  materialId: string | null;
  resourcePath: string | null;
  materialTitle: string;
  correct: number | null;
  total: number | null;
  percent: number | null;
  sectionIndex: number | null;
  sectionCount: number | null;
  startedAt: string;
  occurredAt: string;
  completedAt: string | null;
};

export type StudyModuleItem = { moduleTitle: string; title: string; href: string; itemType: string };
export type StudyModuleGroup = { title: string; items: StudyModuleItem[] };
export type CanvasStudyCourse = { id: string; title: string; url: string; kind: "current" | "past" };
export type CanvasMaterialChoice = { id: string; title: string; host: string; embedded: boolean; allowed: boolean };
export type StudyOutcome = {
  courseTitle: string;
  status: string;
  lyceumAverage?: number | null;
  absences?: number | null;
  attendancePercent?: number | null;
  lyceumGrades?: { assessment: string; score: number | null; scale: number; date: string | null }[];
  canvasAvaTotals?: { earnedPoints: number; possiblePoints: number; percent: number };
};

export type StudyTrajectory = {
  programId: string;
  programTitle: string;
  programCode: string;
  status: string;
  semester?: string;
  currentEnrollments?: ActiveEnrollment[];
  outcomes?: { courseTitle: string; status: string }[];
};

export function normalizeSubjectName(value: string): string {
  return value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLocaleLowerCase("pt-BR")
    .replace(/[^a-z0-9]+/g, " ").trim();
}

export function documentsForEnrollment(enrollment: ActiveEnrollment, documents: StudyDocument[]): StudyDocument[] {
  const title = normalizeSubjectName(enrollment.title);
  return documents.filter((document) => {
    if (!document.course || normalizeSubjectName(document.course) !== title) return false;
    return !!enrollment.semester && !!document.semester && document.semester === enrollment.semester;
  });
}

export function materialsForEnrollment(enrollment: ActiveEnrollment, materials: ImportedStudyMaterial[]): ImportedStudyMaterial[] {
  const title = normalizeSubjectName(enrollment.title);
  if (!enrollment.semester) return [];
  return materials.filter((material) => {
    const matchingAssociation = material.associations?.some((association) =>
      normalizeSubjectName(association.courseTitle) === title
      && association.semester === enrollment.semester,
    );
    if (matchingAssociation) return true;
    return normalizeSubjectName(material.courseTitle) === title
      && !!material.semester
      && material.semester === enrollment.semester;
  });
}

export function lessonSectionCount(markdown: string): number {
  return splitLessonSections(markdown).length;
}

export function splitLessonSections(markdown: string): LessonSection[] {
  const normalized = markdown.replace(/\r\n/g, "\n");
  const documentTitle = /^#\s+(.+?)\s*#*\s*$/m.exec(normalized)?.[1] ?? "";
  const explicitPages = splitExplicitPageSections(normalized, documentTitle);
  const pages = normalized.split(/\f+/);
  const sections: Omit<LessonSection, "index">[] = explicitPages ?? (pages.length > 1
    ? pages.map((page, pageIndex) => {
      const explicitPage = pageHeadingNumberFromMarkdown(page);
      return {
        title: explicitPage === undefined ? `Página ${pageIndex + 1}` : `Página ${explicitPage}`,
        markdown: page.trim(),
        sourcePage: explicitPage ?? pageIndex + 1,
      };
    })
    : splitMarkdownSections(normalized));
  const usableSections = sections.filter((section) => hasTeachingContent(section.markdown, documentTitle));
  const presentedSections = usableSections.length ? usableSections : sections;
  return presentedSections.map((section, index) => {
    const sourcePage = section.sourcePage ?? pageHeadingNumber(section.title);
    return {
      ...section,
      index,
      ...(sourcePage === undefined ? {} : { sourcePage }),
    };
  });
}

function pageHeadingNumber(title: string): number | undefined {
  const match = /^página\s+(\d+)$/i.exec(title.trim());
  if (!match) return undefined;
  const page = Number(match[1]);
  return Number.isSafeInteger(page) && page > 0 ? page : undefined;
}

function pageHeadingNumberFromMarkdown(markdown: string): number | undefined {
  for (const line of markdown.split("\n")) {
    const heading = /^\s*#{1,6}\s+(Página\s+\d+)\s*#*\s*$/i.exec(line);
    if (heading) return pageHeadingNumber(heading[1]);
  }
  return undefined;
}

function splitExplicitPageSections(markdown: string, documentTitle: string): Omit<LessonSection, "index">[] | null {
  const marker = /^\s*#{1,6}\s+Página\s+(\d+)\s*#*\s*$/i;
  const sections: Omit<LessonSection, "index">[] = [];
  let preamble: string[] = [];
  let current: { page: number; lines: string[] } | null = null;
  let foundMarker = false;

  const finishPage = () => {
    if (!current) return;
    sections.push({
      title: `Página ${current.page}`,
      markdown: current.lines.join("\n").trim(),
      sourcePage: current.page,
    });
    current = null;
  };

  for (const line of markdown.split("\n")) {
    const match = marker.exec(line);
    if (match) {
      foundMarker = true;
      finishPage();
      current = { page: Number(match[1]), lines: [line] };
    } else if (current) {
      current.lines.push(line);
    } else {
      preamble.push(line);
    }
  }
  finishPage();
  if (!foundMarker) return null;

  const introduction = preamble
    .filter((line) => !/^\s*#{1,6}\s+/.test(line) && !isSourceMetadataLine(line))
    .join("\n")
    .trim();
  if (hasTeachingContent(introduction, documentTitle)) {
    sections.unshift({ title: "Introdução", markdown: introduction });
  }
  return sections;
}

function hasTeachingContent(markdown: string, documentTitle: string): boolean {
  const lines = markdown.split("\n").filter((line) =>
    !/^\s*#{1,6}\s+/.test(line)
    && !isSourceMetadataLine(line)
    && !isPublicationCreditLine(line),
  );
  const content = plainText(lines.join(" "));
  if (!content) return false;

  const normalizedContent = normalizeForComparison(content);
  const normalizedTitle = normalizeForComparison(documentTitle);
  if (normalizedContent.length <= 80 && normalizedTitle && normalizedTitle.includes(normalizedContent)) return false;

  const containsPublicationCredit = markdown.split("\n").some(isPublicationCreditLine);
  return !(containsPublicationCredit && content.length < 100);
}

function isSourceMetadataLine(line: string): boolean {
  const normalized = normalizeForComparison(line.replace(/^\s*>\s?/, ""));
  return /^(?:disciplina|canvas|pdf oficial|fonte oficial|baixado em|paginas (?:(?:do|no) )?pdf|pdf original|original preservado)\b/.test(normalized);
}

function isPublicationCreditLine(line: string): boolean {
  const normalized = normalizeForComparison(line);
  return /\bcopyright\b/.test(normalized)
    || /nenhuma parte desta publicacao pode ser reproduzida/.test(normalized)
    || /texto de acordo com as normas do novo acordo ortografico/.test(normalized)
    || /autoria do conteudo|projeto grafico|diagramacao/.test(normalized)
    || /^(?:revisao|uva)$/.test(normalized);
}

function plainText(value: string): string {
  return value
    .replace(/!?\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/[`*_~|]/g, " ")
    .replace(/<[^>]*>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function normalizeForComparison(value: string): string {
  return value.normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, " ")
    .trim();
}

function splitMarkdownSections(markdown: string): Omit<LessonSection, "index" | "sourcePage">[] {
  const lines = markdown.replace(/\r\n/g, "\n").split("\n");
  const sections: { title: string; lines: string[] }[] = [];
  let beforeFirstHeading: string[] = [];
  let current: { title: string; lines: string[] } | null = null;
  let fence: string | null = null;
  for (const line of lines) {
    const fenceMarker = /^\s*(`{3,}|~{3,})/.exec(line)?.[1];
    if (fenceMarker) {
      if (!fence) fence = fenceMarker[0];
      else if (fenceMarker[0] === fence[0]) fence = null;
      if (current) current.lines.push(line);
      else beforeFirstHeading.push(line);
      continue;
    }
    const heading = !fence && /^(#{1,6})\s+(.+?)\s*#*\s*$/.exec(line);
    if (heading) {
      if (current) sections.push(current);
      else if (beforeFirstHeading.some((item) => item.trim())) {
        sections.push({ title: "Introdução", lines: beforeFirstHeading });
      }
      current = { title: heading[2], lines: [line] };
    } else if (current) current.lines.push(line);
    else beforeFirstHeading.push(line);
  }
  if (current) sections.push(current);
  else if (beforeFirstHeading.some((item) => item.trim())) sections.push({ title: "Aula", lines: beforeFirstHeading });
  const nonEmpty = sections.filter((section) => hasTeachingContent(section.lines.join("\n"), ""));
  return nonEmpty.length ? nonEmpty.map((section) => ({ title: section.title, markdown: section.lines.join("\n").trim() })) : [{ title: "Aula", markdown: markdown.trim() }];
}
