# チャプター9講座資料: 仕上げのミニプロジェクト

## 目的と持ち帰り
- これまでの要素を組み合わせて200行規模のCLIを完走する。
- 仕様策定→設計→実装→テスト→品質チェックの流れを体験する。
- `lib.rs`/`main.rs`の責務分離を徹底し、エラー処理・テストを組み込む。

## プロジェクト例
1. TODOリストマネージャ (`serde_json`で永続化)
2. Markdownワードカウンタ (`pulldown-cmark`)
3. CSVフィルタリングツール (`csv` + `clap`)

## 講義アウトライン
1. 仕様を書き起こす
   - 目的、入力、出力、CLIフラグ、簡易ユーザーストーリー。
2. モジュール設計
   - ドメインは`src/lib.rs`、I/OとCLIは`src/main.rs`。
   - テスト可能な純粋関数をlib側に寄せる。
3. エラー設計
   - `thiserror`で`Error`enumを作り、`Result<T, Error>`を返す。
   - `anyhow`でアプリ層のエラーをラップする選択肢も紹介。
4. 品質ルーチン
   - `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`。
   - サブコマンド/フラグの結合テストを`tests/`に置く。

## デモの進め方（ライブ）
1. テーマ決定→`README`に目的/入出力/想定ユーザーを書く。
2. `cargo new mytool` → `src/lib.rs`にコアロジック、`src/main.rs`で`clap`や`std::env::args`処理。
3. 1〜2本の統合テストを`tests/`に置き、`cargo test`で回す。
4. `cargo clippy -- -D warnings`で警告を潰す。

## ハンズオン課題（講義中）
1. テーマを1つ選び、`README`に要求仕様を箇条書きでまとめる。
2. `src/lib.rs`にドメイン関数を置き、`main.rs`は薄いラッパーにするリファクタを実施。
3. `cargo clippy -- -D warnings`と`cargo test`が通るまでブラッシュアップ。

## 練習問題（宿題）
1. 入力/出力の例を3ケース用意し、統合テストに落とし込む。
2. エラー型を`thiserror`で定義し、ユーザー向けメッセージを`Display`で整える。
3. パフォーマンスが気になる箇所を`cargo build --release`で確認し、差分をメモ。

## チェックリスト
- [ ] 仕様を文章でまとめた
- [ ] lib/mainの責務を分離した
- [ ] エラー型を定義し、ユーザー向けメッセージを整えた
- [ ] fmt/clippy/test/releaseビルドを回した

## 補足資料
- Rust Book 12章「I/Oプロジェクト」
- クレートドキュメント: `clap`, `serde_json`, `csv`, `anyhow`, `thiserror`
- サンプル: `samples/ch09_mini_project`
