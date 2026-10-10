export type ActiveEnrollment = {
  id: string;
  title: string;
  section?: string;
  startDate?: string;
  status: string;
  semester?: string;
  instructorAsDisplayed?: string;
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
  pages: number;
  importedAt: string;
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
  return materials.filter((material) =>
    normalizeSubjectName(material.courseTitle) === title
    && !!enrollment.semester
    && !!material.semester
    && material.semester === enrollment.semester,
  );
}

export function lessonSectionCount(markdown: string): number {
  return splitLessonSections(markdown).length;
}

export function splitLessonSections(markdown: string): { title: string; markdown: string }[] {
  const pages = markdown.replace(/\r\n/g, "\n").split(/\f+/);
  if (pages.length > 1) {
    return pages.flatMap((page, pageIndex) => {
      if (!page.trim()) return [];
      return splitMarkdownSections(page).map((section) => ({
        ...section,
        title: section.title === "Aula" ? `Página ${pageIndex + 1}` : section.title,
      }));
    });
  }
  return splitMarkdownSections(markdown);
}

function splitMarkdownSections(markdown: string): { title: string; markdown: string }[] {
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
  const nonEmpty = sections.filter((section) => section.lines.join("\n").trim());
  return nonEmpty.length ? nonEmpty.map((section) => ({ title: section.title, markdown: section.lines.join("\n").trim() })) : [{ title: "Aula", markdown: markdown.trim() }];
}
