# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust + htmx によるタスク管理ダッシュボードアプリケーション。Axum (Web フレームワーク) + Askama (テンプレートエンジン) + htmx + Tailwind CSS v4 + daisyUI v5 の構成。

## Commands

### Build & Run

```bash
cargo build                  # ビルド
cargo run                    # サーバー起動 (http://0.0.0.0:3000)
cargo clippy                 # リント
cargo test                   # テスト
cargo test <test_name>       # 単一テスト実行
```

### CSS (Tailwind)

```bash
pnpm dev:css                 # Tailwind CSSのウォッチビルド (assets/tailwind.css → public/style.css)
pnpm build:css               # 本番用ミニファイビルド
```

サーバーと CSS ウォッチは別ターミナルで並行実行する。

### Formatting

```bash
pnpm format                  # Prettier でテンプレート等をフォーマット
pnpm format:check            # フォーマットチェック
```

## Architecture

### サーバーサイドレンダリング構成

Axum ルーターで HTTP リクエストを処理し、Askama テンプレートで HTML をレンダリングして返す。フロントエンドの動的操作は htmx で行い、SPA 的な JS framework は使用しない。

### コードの流れ

`src/main.rs` → ルーター定義・サーバー起動
`src/handlers.rs` → 各ルートのハンドラ（テンプレート構造体定義もここ）
`src/models.rs` → データモデル（`Task`, `TaskFilter`, `CreateTask`, `AppState` 等）
`src/db.rs` → SQLiteプール初期化・マイグレーション自動実行

### テンプレートとhtmxパターン

- テンプレートは `templates/` に配置し、Askama の `#[derive(Template)]` マクロで Rust 構造体にバインド
- `base.html` がレイアウトテンプレート。各ページは `{% extends "base.html" %}` で継承
- htmxリクエスト時は部分テンプレート（例: `tasks_table_body.html`）だけ返し、通常アクセス時はフルページを返す（`HX-Request` ヘッダーで判定）
- htmx属性は `templates/` 内のHTML側に記述する

### データベース

- SQLite を使用。`DATABASE_URL` 環境変数で接続先を指定（`.env` ファイルを参照）
- マイグレーションは `migrations/` ディレクトリに配置。`sqlx::migrate!()` でアプリ起動時に自動実行
- SQL は `handlers.rs` 内で直接記述（ORMは使わない）

### スタイリング

- Tailwind CSS v4 + daisyUI v5。ソース CSS (`assets/tailwind.css`) から `public/style.css` を生成
- Tailwind のコンテンツスキャン対象に `templates/` を `@source` ディレクティブで指定済み
- カスタムカラー変数（status-*, priority-*）は `assets/tailwind.css` の `@theme` ブロックで定義
- 静的ファイルは `/public` パスでサーブ（`tower-http::ServeDir`）

### ログ

`tracing` + `tracing-subscriber` + `tower-http::TraceLayer` で構成。`RUST_LOG` 環境変数で制御（デフォルト: debug）。

### 開発時ライブリロード

`tower-livereload` が有効。テンプレートはAskamaのデバッグビルドで自動再読み込みされるが、Rustコード変更時は `cargo run` の再起動が必要。

### その他

- Rust edition 2024 を使用
