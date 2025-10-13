// Worker configuration types
declare global {
  interface Env {
    DB: D1Database;
  }

  interface D1Database {
    prepare(query: string): D1PreparedStatement;
    exec(query: string): Promise<D1ExecResult>;
    batch(statements: D1PreparedStatement[]): Promise<D1BatchResult[]>;
  }

  interface D1PreparedStatement {
    bind(...values: any[]): D1PreparedStatement;
    first<T = any>(): Promise<T | null>;
    run(): Promise<D1Result>;
    all<T = any>(): Promise<D1Result<T>>;
  }

  interface D1Result<T = any> {
    results: T[];
    success: boolean;
    meta: {
      changes: number;
      last_row_id: number;
      duration: number;
    };
  }

  interface D1ExecResult {
    count: number;
    duration: number;
  }

  interface D1BatchResult {
    results: any[];
    success: boolean;
    meta: {
      changes: number;
      last_row_id: number;
      duration: number;
    };
  }
}

export {};
