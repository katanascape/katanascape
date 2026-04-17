export class KatanascapeError extends Error {
  public readonly code: string;

  public constructor(code: string, message: string) {
    super(message);
    this.name = "KatanascapeError";
    this.code = code;
  }
}
