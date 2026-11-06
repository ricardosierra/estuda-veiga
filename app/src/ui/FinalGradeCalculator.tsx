import { invoke } from "@tauri-apps/api/core";
import { useState, type FormEvent } from "react";
import {
  buildFinalGradeInput,
  formatGradeValue,
  gradeStatusLabel,
  type DisciplineKind,
  type FinalGradeResult,
} from "./canvasAcademic";

export function FinalGradeCalculator() {
  const [kind, setKind] = useState<DisciplineKind>("regular");
  const [fields, setFields] = useState({ a1: "", a2: "", a3: "", a4: "" });
  const [result, setResult] = useState<FinalGradeResult | null>(null);
  const [error, setError] = useState("");

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    setError("");
    try {
      const input = buildFinalGradeInput(kind, fields);
      setResult(await invoke<FinalGradeResult>("calculate_final_grade", { input }));
    } catch (failure) {
      setResult(null);
      setError(failure instanceof Error ? failure.message : String(failure));
    }
  };

  const field = (key: keyof typeof fields, label: string) => <label>
    <span>{label}</span>
    <input inputMode="decimal" placeholder="0 a 10" value={fields[key]} onChange={(event) => setFields({ ...fields, [key]: event.target.value })} />
  </label>;

  return <section className="academic-portal-card" aria-labelledby="final-grade-title">
    <div className="academic-portal-header">
      <div><p className="eyebrow">CÁLCULO LOCAL · SEM REDE</p><h2 id="final-grade-title">Nota final ponderada</h2></div>
    </div>
    <p className="academic-portal-copy">Regra dos critérios de avaliação da graduação EAD: A1 com peso 4 e a maior nota entre A2 e A3 com peso 6; aprovação com 6,0. Disciplinas especiais usam A4 no lugar das provas e aprovam com 7,0. É uma estimativa: vale o boletim do Lyceum.</p>
    <form className="final-grade-form" onSubmit={(event) => void submit(event)}>
      <label>
        <span>Tipo</span>
        <select value={kind} onChange={(event) => { setKind(event.target.value as DisciplineKind); setResult(null); }}>
          <option value="regular">Regular (A1, A2, A3)</option>
          <option value="special">Especial: estágio, TCC (A1, A4)</option>
        </select>
      </label>
      {field("a1", "A1 (média das AVAs)")}
      {kind === "regular" ? <>{field("a2", "A2")}{field("a3", "A3")}</> : field("a4", "A4")}
      <button className="button primary">Calcular</button>
    </form>
    {error && <p className="catalog-note" role="alert">{error}</p>}
    {result && <div className="portal-capture-result" role="status">
      <div className="portal-capture-summary">
        <b>{gradeStatusLabel[result.status]}</b>
        <span>NFp {formatGradeValue(result.finalGrade)} · mínimo {formatGradeValue(result.passingGrade)}{result.examUsed ? ` · prova usada: ${result.examUsed}` : ""}</span>
      </div>
      {result.neededExamGrade !== null && <p className="academic-portal-message">Nota mínima necessária na prova: <b>{formatGradeValue(result.neededExamGrade)}</b></p>}
      <small>{result.formula}</small>
      {result.notes.map((note) => <p className="catalog-note" key={note}>{note}</p>)}
    </div>}
  </section>;
}
