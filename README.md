# 📊 Dashboard Playground

このプロジェクトは、Rust + htmx の機能を試す、プレイグラウンド用のダッシュボードです。

## ✨ 主な機能

- 📈 タスク統計情報の表示（総数、ステータス別）
- ✏️ タスクの作成、編集、削除
- 🔍 タスクの検索とフィルタリング（部分一致検索・ステータス/優先度フィルター）

## 📷 ショーケース

## 🛠️ 技術スタック

### フロントエンド

- **htmx** - SPAフレームワークなしの動的UI
- **Tailwind CSS v4** - ユーティリティファーストCSS
- **daisyUI v5** - UIコンポーネント

### バックエンド

- **Axum** - Webフレームワーク
- **Askama** - テンプレートエンジン
- **SQLx** - SQLクエリ（SQLite）
- **Tokio** - 非同期ランタイム

### 開発ツール

- **tower-livereload** - ライブリロード
- **tracing** - ログ・トレース
- **Prettier** - テンプレートフォーマッター
- **Clippy** - Rustリンター

## 🚀 セットアップ手順

### 前提条件

- **Rust** (edition 2024)
- **pnpm**

### 1️⃣ 依存関係のインストール

```bash
pnpm install
```

### 2️⃣ 環境変数の設定

`.env` ファイルを作成してください：

```bash
DATABASE_URL=sqlite:data/db.sqlite
```

### 3️⃣ CSS のビルド

```bash
pnpm dev:css
```

### 4️⃣ 開発サーバーの起動

別ターミナルで：

```bash
cargo run
```

ファイル変更時に自動再起動したい場合：

```bash
cargo watch -x run
```

ブラウザで [http://localhost:3000](http://localhost:3000) を開いてアプリケーションを確認できます。

> **注意**: データベースとテーブルはアプリ起動時にマイグレーションで自動作成されます。シードデータも初回起動時に投入されます。

## 🧪 テスト・リント

```bash
cargo test              # テスト
cargo clippy            # Rustリント
pnpm format:check       # テンプレートフォーマットチェック
```

## 🐳 Docker

### Docker Compose（推奨）

```bash
docker compose up
```

ブラウザで [http://localhost:3000](http://localhost:3000) でアクセスできます。データベースは名前付きボリューム（`db-data`）に永続化されます。

### Docker 単体

```bash
# ビルド
docker build -t rust-htmx-dashboard .

# 起動
docker run -p 3000:3000 \
  -e DATABASE_URL=sqlite:/app/data/db.sqlite?mode=rwc \
  -v db-data:/app/data \
  rust-htmx-dashboard
```

> Dockerfile はマルチステージビルド（CSS ビルド → Rust ビルド → ランタイム）で構成されており、最終イメージに不要なビルドツールは含まれません。

## 🏭 本番ビルド

```bash
cargo build --release
pnpm build:css
```

## 📝 利用可能なコマンド

- `cargo run` - 🚀 開発サーバーを起動
- `cargo build` - 📦 ビルド
- `cargo clippy` - 🔍 リンターを実行
- `cargo test` - 🧪 テストを実行
- `pnpm dev:css` - 🎨 Tailwind CSSウォッチビルド
- `pnpm build:css` - 📦 CSS本番ビルド（ミニファイ）
- `pnpm format` - ✨ テンプレートをフォーマット

## 📁 プロジェクト構造

```
rust-htmx-dashboard/
├── src/
│   ├── main.rs             # ルーター定義・サーバー起動
│   ├── handlers.rs         # リクエストハンドラ・テンプレート構造体
│   ├── models.rs           # データモデル（Task, TaskFilter 等）
│   └── db.rs               # SQLiteプール初期化・マイグレーション実行
├── templates/
│   ├── base.html           # レイアウトテンプレート
│   ├── dashboard.html      # ダッシュボードページ
│   ├── tasks.html          # タスク一覧ページ
│   ├── tasks_table_body.html  # テーブル行の部分テンプレート（htmx用）
│   ├── tasks_create.html   # タスク作成ページ
│   └── tasks_edit.html     # タスク編集ページ
├── assets/
│   └── tailwind.css        # Tailwind CSSソース（@theme定義含む）
├── public/                 # 静的ファイル配信ディレクトリ
│   └── style.css           # ビルド済みCSS（生成物）
├── migrations/             # SQLxマイグレーションファイル
└── data/                   # SQLiteデータベースファイル
```
