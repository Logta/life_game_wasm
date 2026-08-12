# Conway's Game of Life - WebAssembly 版

Rust と WebAssembly で構築された Conway's Game of Life の高性能実装。Vite によるモダンなウェブインターフェースを搭載。

## 🚀 特徴

- **高性能**: Rust + WebAssembly による超高速セルオートマトンシミュレーション
- **インタラクティブ UI**: セルのクリック切り替え、リアルタイム速度制御、再生/一時停止/ステップ制御
- **モダンツール**: 高速開発のための Vite、統合タスク管理のための mise
- **包括的テスト**: ゲームロジックのユニットテスト、GitHub Actions による CI/CD パイプライン
- **プロダクション対応**: 最適化ビルド、セキュリティ監査、依存関係管理

## 🎮 ライブデモ

開発サーバーを起動して `http://localhost:5173` にアクセス:

```bash
mise run dev
```

### 操作方法

- **再生/一時停止**: シミュレーションの開始・停止
- **ステップ**: 手動で 1 世代進める
- **クリア**: グリッドを空の状態にリセット
- **ランダム**: ランダムな初期パターンを生成
- **速度スライダー**: シミュレーション速度を調整 (1-60 FPS)
- **グリッドをクリック**: 個別のセルを切り替え

## 🛠️ 開発環境のセットアップ

### 必要な環境

- [Rust](https://rustup.rs/) (stable toolchain)
- [Bun](https://bun.sh/) (v1.2+)
- [mise](https://mise.jdx.dev/) (推奨) または手動ツールインストール

### mise を使ったクイックスタート

```bash
# 全ての依存関係をインストールして開発サーバーを起動
mise run dev
```

### 手動セットアップ

```bash
# WebAssembly用のRustターゲットをインストール
rustup target add wasm32-unknown-unknown

# wasm-packをインストール
cargo install wasm-pack

# 依存関係をインストール
bun install

# WebAssemblyパッケージをビルド
wasm-pack build --target web --out-dir pkg --dev

# 開発サーバーを起動
bun run dev
```

## 📋 利用可能なコマンド

### mise を使用 (推奨)

```bash
mise run dev          # 完全な開発環境セットアップとサーバー起動
mise run build        # プロダクションビルド
mise run test          # 全てのテストを実行 (Rust + WebAssembly)
mise run test-rust     # Rustテストのみ実行
mise run wasm-build    # 開発用WebAssemblyビルド
mise run clean         # 全てのビルド成果物をクリア
mise run check         # フォーマットとlintingをチェック
mise run fix           # コードフォーマットの問題を修正
mise run preview       # プロダクションビルドをプレビュー
```

### bun を使用

```bash
bun run dev            # 開発サーバーを起動
bun run build          # プロダクション用ビルド
bun run preview        # プロダクションビルドをプレビュー
bun test               # テストを実行
```

## 🏗️ プロジェクト構造

Rust(WASM)側とTypeScript側のソースは同じ `src/` に共存しています。

```
├── src/                    # Rust/WebAssembly + TypeScriptソースコード
│   ├── lib.rs             # メインWebAssemblyインターフェース
│   ├── field.rs           # ゲームフィールドの実装
│   ├── renderer.rs        # Canvas描画ロジック
│   ├── error.rs           # エラー型定義
│   ├── constants.rs       # 描画設定の定数
│   ├── rules.rs           # ルールトレイト定義
│   ├── rules/             # ゲームルールの実装
│   ├── index.ts           # フロントエンドのエントリーポイント
│   ├── coordinates.ts     # 座標変換ロジック(純粋関数)
│   └── coordinates.test.ts # coordinates.tsのユニットテスト
├── tests/                 # Rust統合テスト(wasm-bindgen-test)
├── index.html             # HTMLテンプレート
├── .github/workflows/     # CI/CDパイプライン
├── mise.toml              # タスクランナー設定 (node/rust/bunバージョンの単一の真実の源泉)
├── vite.config.ts         # Vite設定
├── Cargo.toml             # Rust依存関係
└── package.json           # Bun依存関係
```

## 🧪 テスト

```bash
# 全てのテストを実行 (Rust + WebAssembly + TypeScript)
bun run test

# Rustユニットテストのみ実行
cargo test

# ブラウザでWebAssemblyテストを実行
wasm-pack test --headless --firefox

# TypeScriptユニットテストのみ実行
bun run test:ts
```

## 🔧 技術スタック

- **コア**: パフォーマンス重視のシミュレーションのための Rust + WebAssembly
- **フロントエンド**: レンダリングに Canvas API を使用した TypeScript
- **ビルドツール**: 高速開発のための Vite、WebAssembly パッケージングのための wasm-pack
- **パッケージマネージャー**: 高速な依存関係インストールと実行のための Bun
- **タスク管理**: 統合開発ワークフローのための mise
- **テスト**: 包括的カバレッジのための Cargo test + wasm-pack test
- **CI/CD**: セキュリティ監査付きの GitHub Actions

## 🎯 ゲームルール

Conway's Game of Life は以下のシンプルなルールに従います:

1. **誕生**: 死んでいるセルがちょうど 3 つの生きた隣接セルを持つと生きる
2. **生存**: 生きているセルが 2 つまたは 3 つの生きた隣接セルを持つと生き続ける
3. **死亡**: その他の全てのセルは死ぬか死んだままになる

これらのシンプルなルールにも関わらず、ゲームは信じられないほど複雑で美しいパターンを生み出すことができます！

このプロジェクトのフィールドは**トーラス状（上下左右がループする）**の実装になっています。つまり右端の隣接セルは左端、下端の隣接セルは上端として扱われ、グリッドの境界は存在しません（`src/field.rs` の `count_live_neighbors`）。

## 🚀 パフォーマンス

この実装は Rust のゼロコスト抽象化と WebAssembly のネイティブに近いパフォーマンスを活用して、大きなグリッドを高フレームレートで処理します。レンダリングは効率的な Canvas API 使用と JavaScript オーバーヘッドの最小化により最適化されています。

## 📝 ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。

## 🙏 謝辞

- [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) - 数学的基盤
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/) - 優秀な学習リソース
- [wasm-pack](https://github.com/rustwasm/wasm-pack) - 必須の WebAssembly ツール
