# Rust基礎からの学習ガイド

この教材は「なぜそうなるか」を意識しながらRustを基礎から身につけるための自習ロードマップです。各チャプターは「目的→内容→練習問題」の順で構成され、`cargo`コマンドで手を動かしながら理解を深めるよう設計しています。

---

## 1. 学習全体像

| フェーズ | 期間の目安 | 目標 | キーワード | 成果物 |
| --- | --- | --- | --- | --- |
| 0. 準備 | 0.5日 | 開発環境構築・Cargo操作 | Rustup, VS Code, rust-analyzer | ワークスペース, `cargo run` |
| 1. 言語の芯 | 4日 | 型・所有権・制御フローを理解 | immutable, shadowing, ownership, borrowing | チュートリアルプログラム |
| 2. 実用構文 | 5日 | モジュール化・エラー処理・コレクション | module, Result, match, Vec, HashMap | CLIツール |
| 3. 抽象化 | 5日 | ジェネリクス・トレイト・ライフタイム | trait bound, impl, lifetime | ライブラリ crate |
| 4. 発展 | 継続 | 非同期・FFIなど応用 | async/await, tokio, FFI | ミニプロジェクト |

---

## 2. 事前準備

1. Rustupでツールチェーンを導入  
   ```bash
   rustup default stable
   rustup component add rustfmt clippy
   ```
2. エディタ設定  
   - VS Code + rust-analyzer拡張  
   - `rustfmt`自動整形を保存時に有効化  
3. サンプルプロジェクトを用意  
   ```bash
   cargo new hello-rust
   cd hello-rust
   cargo run
   ```
4. ドキュメント参照元  
   - The Rust Programming Language (公式書籍)  
   - Rust By Example  
   - `std`ドキュメント (`rustup doc --std`)

---

## 3. チャプター別教材

### チャプター1: Rustの第一歩
- **到達目標**: `cargo`の基本操作と`main`関数の書き方を理解する。
- **キーワード**: `fn main`, `println!`, `cargo run`, `cargo check`
- **ミニ講義**:
  1. Rustコンパイルの流れ（ソース→中間表現→バイナリ）
  2. マクロ呼び出しと`println!`の仕組み
  3. デバッグビルドとリリースビルドの違い（`cargo build --release`）
- **練習問題**:
  1. `cargo new intro`でプロジェクトを作り、コマンドライン引数で挨拶文を切り替える。
  2. `println!`のフォーマット指定子 (`{}` / `{:?}`) の違いをコメント付きで出力する。
- **サンプルコード**: `samples/ch01_hello_cli`

---

### チャプター2: 変数・型・所有権の直感
- **到達目標**: 変数束縛と所有権のルールを言語化できる。
- **キーワード**: immutable, `let mut`, `shadowing`, `String`, `&str`, `Copy`
- **ミニ講義**:
  - ヒープとスタック、`String`と`&str`のメモリ管理
  - `Copy`トレイトが付与される型とムーブセマンティクス
  - `drop`タイミングの可視化 (`println!`を使った順序確認)
- **練習問題**:
  1. `String`を関数に渡したときと`&str`を渡したときの違いを出力して説明文を添える。
  2. ムーブを防ぐために`clone`を使用した例と、借用で回避する例を比較する。
- **サンプルコード**: `samples/ch02_ownership_basics`

---

### チャプター3: 制御フローとマッチング思考
- **到達目標**: `if/else`, `match`, `loop/while/for`を適材適所で選択できる。
- **キーワード**: `if`式, `match`, パターン, `while let`
- **ミニ講義**:
  - `if`も`match`も「式」であり値を返すこと
  - `match`ガード (`match x { n if n > 10 => ... }`)
  - `loop`に`break value`を付けて値を返すテクニック
- **練習問題**:
  1. サイコロの目(1-6)を乱数で生成し、`match`で結果を分類してコメントを出す。
  2. `while let Some(x)`構文でスタック風の`Vec`を処理するプログラムを書く。
- **サンプルコード**: `samples/ch03_control_flow`

---

### チャプター4: 所有権・借用・ライフタイムの基礎
- **到達目標**: 不変借用と可変借用のルールを説明でき、借用チェッカのエラーを読める。
- **キーワード**: `&T`, `&mut T`, ライフタイム省略規則, `slice`
- **ミニ講義**:
  - 借用規則「同時に複数の不変借用」 vs 「唯一の可変借用」
  - `slice`参照 (`&[T]`, `&str`) と境界チェック
  - コンパイラのライフタイム推論の考え方（`'a`を明示する例）
- **練習問題**:
  1. 可変借用が2つ衝突するサンプルを敢えて書き、エラーメッセージをコメントで整理する。
  2. スライスを受け取って最大値を返す関数`fn max_in(slice: &[i32]) -> Option<i32>`を実装する。
- **サンプルコード**: `samples/ch04_borrowing_lifetimes`

---

### チャプター5: コレクションとイテレーション
- **到達目標**: 標準コレクションと`iterator`チェーンでデータを処理できる。
- **キーワード**: `Vec`, `HashMap`, `Iterator`, `collect`, `into_iter`
- **ミニ講義**:
  - `Vec`と`VecDeque`の違い、`HashMap`の所有権仕様 (`insert`でムーブ)
  - `iter` / `iter_mut` / `into_iter` の使い分け
  - `map`, `filter`, `fold`, `enumerate`を使った宣言的ループ
- **練習問題**:
  1. 文章から単語頻度を数え、`HashMap`と`Vec`でランキングを表示するCLIを作る。
  2. `Iterator`チェーンだけでフィボナッチ数列の先頭10個を生成して出力する。
- **サンプルコード**: `samples/ch05_collections_iter`

---

### チャプター6: エラー処理とResult
- **到達目標**: `Result`/`Option`を使い分け、`?`演算子で伝搬させられる。
- **キーワード**: `Result<T, E>`, `thiserror`, `anyhow`, `unwrap`の危険性
- **ミニ講義**:
  - パニック vs リカバリ可能なエラー
  - `map_err`, `and_then`による合成
  - `thiserror`で独自エラー型を定義する例
- **練習問題**:
  1. ファイルを読み込み数値を合計するCLIを作り、エラーを`Result`で返す。
  2. `Option`を`Result`に変換しつつ`?`で早期リターンする関数を実装する。
- **サンプルコード**: `samples/ch06_error_handling`

---

### チャプター7: モジュール・テスト・Cargo活用
- **到達目標**: モジュール分割とテスト駆動の基本パターンを押さえる。
- **キーワード**: `mod`, `pub`, `use`, `cargo test`, `cfg`
- **ミニ講義**:
  - ファイル構成 (`lib.rs`, `mod.rs`) と公開範囲
  - 単体テストと結合テスト (`tests/`ディレクトリ)
  - `cargo fmt`, `cargo clippy`で品質を保つルーチン
- **練習問題**:
  1. `src/lib.rs`を2つのモジュールに分け、`pub use`で公開APIを整える。
  2. `tests/`配下に統合テストを1本追加し、テストに`assert_eq!`以外のアサーションマクロを使ってみる。
- **サンプルコード**: `samples/ch07_modules_tests`

---

### チャプター8: ジェネリクス・トレイト・ライフタイム応用
- **到達目標**: トレイト境界を設計し、再利用可能な抽象を作れる。
- **キーワード**: `impl<T>`, `trait bound`, `where`句, `From`/`Into`, `Iterator`実装
- **ミニ講義**:
  - トレイトオブジェクト(`dyn Trait`)と静的ディスパッチ(`impl Trait`)の違い
  - `lifetime`を明示するケース（構造体が参照を保持する場合など）
  - `PhantomData`と所有権セマンティクス
- **練習問題**:
  1. ジェネリックな`Pair<T>`構造体に`new`, `swap`, `sum`（`T: std::ops::Add<Output=T> + Copy`）を実装する。
  2. `trait Drawable { fn draw(&self) -> String; }`を定義し、複数型で実装＋`Vec<Box<dyn Drawable>>`を描画する。
- **サンプルコード**: `samples/ch08_generics_traits`

---

### チャプター9: 仕上げのミニプロジェクト
- **目標**: これまでの要素を組み合わせ、200行程度のCLIツールを完成させる。
- **テーマ例**:
  1. TODOリストマネージャ (`serde_json`で永続化)
  2. Markdownワードカウンタ (`pulldown-cmark`を活用)
  3. CSVフィルタリングツール (`csv` crate + `clap` CLI)
- **チェックリスト**:
  - `src/lib.rs`にドメインロジック、`src/main.rs`はCLI処理に分離
  - エラー型を`thiserror`でまとめる
  - `cargo fmt`, `cargo clippy`, `cargo test`を通す
- **サンプルコード**: `samples/ch09_mini_project`

---

## 4. ハンズオンラボ（手を動かす教材）

この章では、コーディング→コンパイル→実行までを一連で体験する3本のラボを用意しました。各ラボは`cargo`コマンドをそのままコピーして実行できるようになっており、章ごとの学びをすぐに手で確認できます。

### ラボ1: 引数で挨拶するCLI
- **ゴール**: コマンドライン引数の読み取りと条件分岐を実装し、`cargo run`で動作を確認する。
- **想定時間**: 30〜45分
- **セットアップ**
  ```bash
  cargo new lab01-hello-cli
  cd lab01-hello-cli
  ```
- **実装 (`src/main.rs`)**
  ```rust
  use std::env;

  fn main() {
      let mut args = env::args().skip(1);
      let name = args.next().unwrap_or_else(|| "Rustacean".to_string());
      let loud = args.next().unwrap_or_else(|| "off".to_string());

      let greeting = if loud == "on" {
          format!("HELLO, {name}!")
      } else {
          format!("Hello, {name}.")
      };

      println!("{greeting}");
      println!("arguments supplied: {}", env::args().count() - 1);
  }
  ```
- **ビルド & 実行**
  ```bash
  cargo fmt
  cargo run -- "Naomi" on
  cargo run -- "Taro"
  ```
  実行結果の例:
  ```
  HELLO, Naomi!
  arguments supplied: 2
  ```
- **拡張課題**
  - `--help`や`--upper`などのフラグ形式を`match`で処理してみる。
  - `std::env::args_os`を使ってUnicode/マルチバイト引数でも動作するか確認する。

### ラボ2: 所有権ログツール
- **ゴール**: 所有権をムーブする関数と借用する関数を分け、`cargo test`で関数単体の振る舞いを検証する。
- **想定時間**: 60分
- **セットアップ**
  ```bash
  cd /path/to/workspace
  cargo new lab02-ownership-log
  cd lab02-ownership-log
  ```
- **実装 (`src/lib.rs`)**
  ```rust
  pub fn amplify_owned(mut text: String) -> String {
      text.push_str(" (owned + mutated)");
      text
  }

  pub fn annotate_borrow(text: &str) -> String {
      format!("borrowed: {} chars", text.chars().count())
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn owned_text_gets_suffix() {
          let result = amplify_owned(String::from("Rust"));
          assert!(result.contains("owned"));
      }

      #[test]
      fn borrowed_text_counts_chars() {
          assert_eq!(annotate_borrow("Ferris"), "borrowed: 6 chars");
      }
  }
  ```
- **実装 (`src/main.rs`)**
  ```rust
  use lab02_ownership_log::{amplify_owned, annotate_borrow};

  fn main() {
      let phrase = String::from("Ownership makes data race free");
      let moved = amplify_owned(phrase.clone());
      let borrowed = annotate_borrow(&phrase);

      println!("{moved}");
      println!("{borrowed}");
      println!("original still usable: {phrase}");
  }
  ```
- **ビルド & テスト & 実行**
  ```bash
  cargo fmt
  cargo check
  cargo test
  cargo run
  ```
  期待する出力例:
  ```
  Ownership makes data race free (owned + mutated)
  borrowed: 34 chars
  original still usable: Ownership makes data race free
  ```
- **拡張課題**
  - `annotate_borrow`を`&[i32]`を受け取るバージョンに書き換え、借用スライスでも動くか確認。
  - `cargo clippy`で警告が出た場合は修正し、警告ゼロを目指す。

### ラボ3: ファイル合計CLI
- **ゴール**: `Result`と`?`演算子を用い、ファイルから数値を読み込んで合計するCLIを完成させる。
- **想定時間**: 90分
- **セットアップ**
  ```bash
  cd /path/to/workspace
  cargo new lab03-file-sum
  cd lab03-file-sum
  ```
- **実装 (`src/main.rs`)**
  ```rust
  use std::{
      env,
      error::Error,
      fs::File,
      io::{self, BufRead, BufReader},
  };

  fn main() -> Result<(), Box<dyn Error>> {
      let path = env::args().nth(1).expect("usage: cargo run -- <file>");
      let sum = sum_file(&path)?;
      println!("sum = {sum}");
      Ok(())
  }

  fn sum_file(path: &str) -> Result<i64, io::Error> {
      let file = File::open(path)?;
      let reader = BufReader::new(file);
      let mut total = 0;

      for line in reader.lines() {
          let line = line?;
          if line.trim().is_empty() {
              continue;
          }
          let value: i64 = line.trim().parse().expect("数値を入力してください");
          total += value;
      }

      Ok(total)
  }

  #[cfg(test)]
  mod tests {
      use super::*;
      use std::fs::write;

      #[test]
      fn sums_three_lines() {
          let tmp = "tmp_numbers.txt";
          write(tmp, "10\n20\n30\n").unwrap();
          assert_eq!(sum_file(tmp).unwrap(), 60);
          std::fs::remove_file(tmp).unwrap();
      }
  }
  ```
- **ビルド & 実行**
  ```bash
  printf "1\n2\n3\n" > numbers.txt
  cargo fmt
  cargo test
  cargo run -- numbers.txt
  ```
  表示例:
  ```
  sum = 6
  ```
- **拡張課題**
  - `line.trim().parse()`部分を`match`で書き直し、不正な入力行をスキップして警告を表示する。
  - `struct CliArgs { path: String, min: Option<i64> }`のような構造体を導入し、`clap`で引数をパースする。

---

## 5. 実践的な練習セット

1. **デイリードリル**  
   - `rustlings`で所有権と借用の問題を1日5問  
   - `cargo add`で外部crateを追加する練習  
2. **コードリーディング**  
   - `regex`や`serde`など、興味のあるcrateのソースを読み、所有権パターンをメモ
3. **ミニブログ記録**  
   - 各チャプターの学びを`docs/notes/`にMarkdownでまとめる  
   - エラーで詰まった箇所は`What happened / Why / Fix`の3項目で整理
4. **週次アウトプット**  
   - 週末に学んだ要素を活用した小さなCLIを1本作り、`README`にGIFやログを掲載

---

## 6. 参考リソース

- 公式: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 実用例: [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- 練習: [rustlings](https://github.com/rust-lang/rustlings), [Exercism Rust Track](https://exercism.org/tracks/rust)
- コミュニティ: Rust JP Discord, QiitaのRustタグ, Rust Cookbook
- 書籍（日本語）:  
  - 『実践Rust入門』  
  - 『プログラミングRust 第2版』

---

## 7. 学習の進め方Tips

1. **アウトプットファースト**: 教材を読む→手を動かす→理解を言語化、の順で進める。  
2. **エラーを歓迎**: コンパイラメッセージを必ず和訳し、自分の言葉でメモ。  
3. **借用規則は図解**: 変数の所有権フローを矢印で書き出すと整理しやすい。  
4. **毎週振り返り**: できること／できないことリストを更新し、次週のフォーカスを設定。  
5. **コミュニティ参加**: 質問を投げる・回答することで理解が定着する。

---

このロードマップをベースに、自分の興味に沿った題材（Webサーバ、CLIツール、数値計算など）に置き換えて取り組むことでRustの基礎体力を身につけていきましょう。頑張ってください！
