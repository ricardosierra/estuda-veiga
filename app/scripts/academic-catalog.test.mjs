import assert from 'node:assert/strict';
import test from 'node:test';
import { sharedAcademicSubjects } from '../src/ui/academicCatalog.ts';

test('lists only explicit canonical subjects linked to offerings in multiple programs', () => {
  const catalog = {
    disciplines: [
      {
        id: 'shared-subject',
        title: 'Cálculo Numérico',
        offeringIds: ['cc-calculo', 'ec-calculo'],
        identityStatus: 'same_title_and_workload_candidate',
      },
      {
        id: 'single-program-subject',
        title: 'Estruturas de Dados',
        offeringIds: ['cc-structures', 'ec-structures'],
        identityStatus: 'partial_overlap_candidate',
      },
    ],
    curriculums: [
      {
        id: 'cc',
        programId: 'cc-program',
        programCode: 'CC',
        programTitle: 'Ciência da Computação',
        offerings: [
          { id: 'cc-calculo', canonicalDisciplineId: 'shared-subject', code: 'CC1', title: 'Cálculo Numérico', workloadHours: 66, status: 'pending' },
          { id: 'cc-structures', canonicalDisciplineId: 'single-program-subject', code: 'CC2', title: 'Estruturas de Dados', workloadHours: 66, status: 'pending' },
        ],
      },
      {
        id: 'ec',
        programId: 'ec-program',
        programCode: 'EC',
        programTitle: 'Engenharia de Computação',
        offerings: [
          { id: 'ec-calculo', canonicalDisciplineId: 'shared-subject', code: 'EC1', title: 'Cálculo Numérico', workloadHours: 66, status: 'pending' },
        ],
      },
    ],
  };

  const shared = sharedAcademicSubjects(catalog);

  assert.equal(shared.length, 1);
  assert.equal(shared[0].title, 'Cálculo Numérico');
  assert.deepEqual(shared[0].offerings.map(({ programCode, offering }) => [programCode, offering.id]), [
    ['CC', 'cc-calculo'],
    ['EC', 'ec-calculo'],
  ]);
});

test('does not treat mismatched offering references as a shared subject', () => {
  const catalog = {
    disciplines: [{ id: 'shared-subject', title: 'Cálculo Numérico', offeringIds: ['cc-calculo', 'ec-calculo'] }],
    curriculums: [
      { id: 'cc', programId: 'cc-program', programCode: 'CC', programTitle: 'Ciência da Computação', offerings: [{ id: 'cc-calculo', canonicalDisciplineId: 'shared-subject', code: 'CC1', title: 'Cálculo Numérico', workloadHours: 66, status: 'pending' }] },
      { id: 'ec', programId: 'ec-program', programCode: 'EC', programTitle: 'Engenharia de Computação', offerings: [{ id: 'ec-calculo', canonicalDisciplineId: 'different-subject', code: 'EC1', title: 'Cálculo Numérico', workloadHours: 66, status: 'pending' }] },
    ],
  };

  assert.deepEqual(sharedAcademicSubjects(catalog), []);
});
