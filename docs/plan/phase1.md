## Phase 1: ActivityPub 基盤と基本SNS機能の構築

このフェーズでは、ActivityPub プロトコルに準拠した最小限の SNS 基盤を構築する。
過度な独自機能は避け、他サーバー（Mastodon, Misskey等）と「存在を確認しあえる」「最低限のメッセージ交換ができる」状態を目指す。

### 目標
- ローカルユーザーの作成と WebFinger / Actor Endpoint の公開。
- ActivityPub の基本的な Inbox / Outbox の実装。
- ローカルおよび連合タイムラインの表示。
- React + Rust (Loco) による基本アーキテクチャの確立。

### 実装機能詳細

#### 1. ActivityPub Core (Backend)
- **WebFinger**: `/.well-known/webfinger` の実装。ユーザーアカウントの検索応答。
- **Actor Model**: ユーザー情報を ActivityPub Actor (Person) JSON として返却する機能。
- **Inbox / Outbox**:
    - `inbox`: 外部からの Activity (Follow, Create, Undo 等) を受け取り、検証して DB に保存、またはキューに入れる。
    - `outbox`: ユーザーの投稿を Activity (Create Note) として公開し、フォロワーのサーバーへ配送する（基本的な配送処理）。
- **HTTP Signature**: サーバー間通信の署名検証および署名付与の実装。

#### 2. アプリケーション機能 (Backend & Frontend)
- **認証 (Auth)**: ローカルユーザーの登録・ログイン（JWT等）。
- **投稿 (Note)**: テキスト投稿の作成、保存、表示。
- **フォロー管理**:
    - ローカルユーザー間のフォロー。
    - リモートユーザーへのフォローリクエスト送信。
    - リモートからのフォローリクエストの受諾処理。
- **タイムライン**:
    - ホーム（フォローしているユーザーの投稿）。
    - ローカル（自サーバーの全投稿）。

### データスキーマ (ActivityPub 対応)

ActivityPub の語彙に合わせてテーブルを設計する。

| テーブル名 | 説明 | 主要カラム |
| --- | --- | --- |
| `users` | ローカルユーザーアカウント | `username`, `email`, `password_hash`, `private_key`, `public_key` |
| `actors` | ローカルおよびリモートのActor情報キャッシュ | `uri` (ID), `inbox_url`, `outbox_url`, `username`, `domain`, `public_key` |
| `activities` | ActivityPub アクティビティログ | `uri` (ID), `type`, `actor_id`, `object_id`, `payload` (JSON) |
| `objects` | 投稿内容 (Note) 等のオブジェクト | `uri` (ID), `type` (Note, etc), `content`, `attributed_to` |
| `follows` | フォロー関係 | `follower_id` (Actor), `following_id` (Actor), `state` (Pending/Accepted) |

### 技術スタック

#### Frontend
- **Framework**: React, TypeScript, Tailwind CSS
- **Build Tool**: Rsbuild

#### Backend
- **Language**: Rust
- **Framework**: Loco (Rust on Rails style framework)
- **Database**: PostgreSQL
- **Worker**: Loco Background Workers (for ActivityPub delivery)
