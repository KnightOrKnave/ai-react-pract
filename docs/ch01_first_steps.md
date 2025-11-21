# チャプター1講座資料: Rustの第一歩

## 目的と持ち帰り
- `cargo new/run/check`の基本ワークフローを自力で回せる。
- `fn main`と`println!`の最低限の書き方を説明できる。
- デバッグビルドとリリースビルドの違いを自分の言葉で言える。

## 前提
- Rustupでstableインストール済み (`rustup default stable`)。
- エディタでrust-analyzerが有効。

## 講義アウトライン
1. Rustコンパイルの流れ: ソース → HIR/MIR → バイナリ（最適化・デバッグ情報の有無）。
2. Cargoがやっていること: 依存解決・ビルドパイプライン・`target/`の役割。
3. `println!`はマクロ: 展開後にフォーマットがコンパイル時チェックされる。
4. デバッグビルド vs リリースビルド: 最適化レベルと実行ファイルの配置 (`target/debug` / `target/release`)。

## デモ手順（ライブコーディング用）
```bash
cargo new ch01-hello-cli
cd ch01-hello-cli
cargo run
```
`src/main.rs`を次のように編集して実行:
```rust
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "Rustacean".to_string());
    println!("Hello, {name}!");
}
```

### 実行確認
```bash
cargo check            # 型検査のみ
cargo run -- Alice     # 実行
cargo run --release -- Bob
time cargo run --release -- Carol
```

## ハンズオン課題（講義中に実施）
1. 挨拶CLI: 引数がなければデフォルト名、2番目の引数が`on`なら大文字で挨拶。
2. フォーマット指定子の確認: `{}`と`{:?}`を同じ値で出し、コメントで違いを書く。
3. ビルド成果物の比較: `ls target/debug`と`ls target/release`のファイルサイズを比べ、差をメモ。

## 練習問題（宿題）
1. `cargo run --release`と`cargo run`の時間を`time`で測り、差分を`README`に短くまとめる。
2. 左寄せ・右寄せ・ゼロ埋め (`{:>8}`, `{:<8}`, `{:08}`) を試すミニプログラムを書く。
3. `cargo check`は通るが`cargo run`で失敗する例（例: `panic!`を仕込む）を用意し、違いを説明するコメントを書く。

## チェックリスト
- [ ] `cargo new`でプロジェクト作成を実演した
- [ ] `cargo check`と`cargo run`の違いを説明できる
- [ ] デバッグ/リリースビルドの出力場所を示した
- [ ] フォーマット指定子 `{}`, `{:?}`, `{:0width$}` を触れた

## 補足資料
- 公式ドキュメント: The Rust Programming Language 1章〜2章
- Rust By Example: Hello World, Formatted print
- 既存サンプル: `samples/ch01_hello_cli`
