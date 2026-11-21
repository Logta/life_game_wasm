// Stryker（TS の mutation testing）の設定。`mise run mutants:ts` / `bun run mutants:ts` で実行する。
// テストは bun test で書いているため、Stryker のコマンドランナーで `bun test` を回す。
/** @type {import('@stryker-mutator/api/core').PartialStrykerOptions} */
export default {
  packageManager: "npm",
  testRunner: "command",
  commandRunner: {
    command: "bun test src",
  },
  // 中核ロジック（純粋関数）のみを対象にする。index.ts は DOM/Canvas と WASM の結線なので除外。
  mutate: ["src/**/*.ts", "!src/**/*.test.ts", "!src/index.ts"],
  // TypeScript 7（ネイティブ版）は Stryker が tsconfig の書き換えに使う JS API
  // （ts.parseConfigFileTextToJson）を持たないため、サンドボックスに tsconfig.json を
  // コピーしない。bun test は tsconfig なしでも TS を実行できる。
  ignorePatterns: ["tsconfig.json"],
  coverageAnalysis: "off",
  reporters: ["clear-text", "progress", "html"],
  htmlReporter: {
    fileName: "reports/mutation/mutation.html",
  },
  tempDirName: ".stryker-tmp",
  concurrency: 4,
};
