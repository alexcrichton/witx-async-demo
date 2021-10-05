export type Result<T, E> = { tag: "ok", val: T } | { tag: "err", val: E };
export type Error = ErrorFailure | ErrorAborted;
export interface ErrorFailure {
  tag: "Failure",
  val: string,
}
export interface ErrorAborted {
  tag: "Aborted",
}
export function addImportsToImports(imports: any, obj: Imports, get_export: (name: string) => WebAssembly.ExportValue): void;
export interface Imports {
  fetch(url: string): Promise<Result<Response, Error>>;
  // helper functions to print messages
  log(msg: string): void;
  logErr(msg: string): void;
  dropResponse?: (val: Response) => void;
  dropHeaders?: (val: Headers) => void;
}
export interface Response {
  headers(): Headers;
  body(): Promise<Uint8Array>;
  status(): number;
  statusText(): string;
}
export interface Headers {
  get(name: string): string[];
  entries(): [string, string[]][];
}
