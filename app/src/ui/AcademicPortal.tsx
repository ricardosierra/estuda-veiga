type PortalState = "disconnected" | "waiting_for_login" | "authenticated" | "partial" | "error";

export type PortalStatus = {
  state: PortalState;
  message: string;
  currentUrl: string | null;
  pagesRead: number;
  inaccessibleSections: string[];
};

export type PortalCapture = {
  capturedAt: string;
  pagesRead: number;
  pages: {
    sourceUrl: string;
    title: string;
    headings: string[];
    fields: { label: string; value: string }[];
    tables: { caption: string | null; headers: string[]; rows: string[][] }[];
    links: { label: string; url: string }[];
    capturedAt: string;
  }[];
  inaccessibleSections: string[];
  warnings: string[];
  complete: boolean;
  jsonPath?: string;
  markdownPath?: string;
};

type Props = {
  status: PortalStatus;
  capture: PortalCapture | null;
  busy: boolean;
  onOpen: () => void;
  onCheck: () => void;
  onCapture: () => void;
};

const stateLabel: Record<PortalState, string> = {
  disconnected: "Desconectado",
  waiting_for_login: "Aguardando acesso",
  authenticated: "Acesso ativo",
  partial: "Captura parcial",
  error: "Falha de acesso",
};

export function AcademicPortal({ status, capture, busy, onOpen, onCheck, onCapture }: Props) {
  return <section className="academic-portal-card" aria-labelledby="lyceum-title">
    <div className="academic-portal-header">
      <div><p className="eyebrow">REGISTRO OFICIAL</p><h2 id="lyceum-title">Portal UVA e Lyceum</h2></div>
      <span className={`portal-state portal-state-${status.state}`}><i />{stateLabel[status.state]}</span>
    </div>
    <p className="academic-portal-copy">Capture o histórico, as avaliações, as notas, a frequência e as demais tabelas acadêmicas visíveis. A navegação usa uma janela Chromium do app, em modo de leitura.</p>
    <p className="academic-portal-message" role="status">{status.message}</p>
    {status.currentUrl && <small className="portal-source-url">Página atual: {status.currentUrl}</small>}
    <div className="academic-portal-actions">
      {status.state === "disconnected" || status.state === "error"
        ? <button className="button primary" disabled={busy} onClick={onOpen}>{busy ? "Abrindo…" : "Abrir janela do Lyceum"}</button>
        : status.state === "waiting_for_login"
          ? <button className="button secondary" disabled={busy} onClick={onCheck}>{busy ? "Verificando…" : "Verificar acesso"}</button>
          : <button className="button primary" disabled={busy} onClick={onCapture}>{busy ? "Lendo páginas oficiais…" : "Capturar histórico e notas"}</button>}
      {status.state === "authenticated" && <button className="button secondary" disabled={busy} onClick={onCheck}>Atualizar status</button>}
    </div>
    {(status.state === "waiting_for_login" || status.state === "disconnected") && <p className="portal-login-help">Se necessário, conclua o login ou o MFA manualmente na janela que o app abriu. O Estuda UVA não salva nem registra sua senha.</p>}
    {capture && <div className="portal-capture-result">
      <div className="portal-capture-summary">
        <b>{capture.complete ? "Captura concluída" : "Captura parcial"}</b>
        <span>{capture.pagesRead} página(s) · {new Date(capture.capturedAt).toLocaleString("pt-BR")}</span>
      </div>
      {capture.jsonPath && <small>JSON sincronizado: {capture.jsonPath}</small>}
      {capture.markdownPath && <small>Markdown sincronizado: {capture.markdownPath}</small>}
      {capture.inaccessibleSections.map((item) => <p className="catalog-note" key={item}>Não lida: {item}</p>)}
      {capture.warnings.map((item) => <p className="catalog-note" key={item}>{item}</p>)}
      {capture.pages.map((page, pageIndex) => <details className="portal-page" key={`${page.sourceUrl}-${pageIndex}`}>
        <summary><b>{page.title}</b><small>{page.tables.length} tabela(s) · {page.fields.length} campo(s) · captura preservada</small></summary>
        <small className="portal-source-url">{page.sourceUrl}</small>
        {page.fields.length > 0 && <dl className="portal-facts">{page.fields.map((field, index) => <div key={`${field.label}-${index}`}><dt>{field.label}</dt><dd>{field.value || "Não informado na origem"}</dd></div>)}</dl>}
        {page.tables.map((table, tableIndex) => <div className="portal-table-cards" key={`${table.caption ?? tableIndex}`}>
          <h4>{table.caption || `Tabela ${tableIndex + 1}`}</h4>
          {table.rows.map((row, rowIndex) => <article className="portal-row-card" key={rowIndex}>{table.headers.map((header, cellIndex) => <div key={`${header}-${cellIndex}`}><small>{header}</small><b>{row[cellIndex] || "—"}</b></div>)}</article>)}
        </div>)}
      </details>)}
    </div>}
  </section>;
}
