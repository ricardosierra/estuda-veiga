export type CatalogOffering = {
  id: string;
  code: string;
  title: string;
  workloadHours: number;
  status: string;
  canonicalDisciplineId?: string;
};

export type CatalogCurriculum = {
  id: string;
  programId: string;
  programCode: string;
  programTitle: string;
  offerings: CatalogOffering[];
};

export type CatalogDiscipline = {
  id: string;
  title: string;
  offeringIds: string[];
  identityStatus?: string;
};

export type SharedAcademicSubject = CatalogDiscipline & {
  offerings: Array<{
    curriculumId: string;
    programId: string;
    programCode: string;
    programTitle: string;
    offering: CatalogOffering;
  }>;
};

export function sharedAcademicSubjects(
  catalog: { curriculums: CatalogCurriculum[]; disciplines: CatalogDiscipline[] },
): SharedAcademicSubject[] {
  return catalog.disciplines
    .map((discipline) => {
      const offeringIds = new Set(discipline.offeringIds);
      const offerings = catalog.curriculums.flatMap((curriculum) =>
        curriculum.offerings
          .filter(
            (offering) =>
              offeringIds.has(offering.id) && offering.canonicalDisciplineId === discipline.id,
          )
          .map((offering) => ({
            curriculumId: curriculum.id,
            programId: curriculum.programId,
            programCode: curriculum.programCode,
            programTitle: curriculum.programTitle,
            offering,
          })),
      );

      return { ...discipline, offerings };
    })
    .filter((discipline) => new Set(discipline.offerings.map(({ programId }) => programId)).size > 1);
}
