# チャプター6講座資料: エラー処理とResult

## 目的と持ち帰り
- パニックとリカバリ可能なエラーの違いを説明できる。
- `Result`/`Option`の使い分けと`?`演算子の挙動を理解する。
- カスタムエラー型とエラーラッパ（`thiserror`/`anyhow`）の導入意図を説明できる。

## 講義アウトライン
1. 失敗の2分類
   - パニック（バグ/想定外） vs リカバリ可能（IO失敗など）。
2. `Result<T, E>`と伝搬
   - `?`は`return Err(...)`へデシュガーされる。
   - `map_err`, `and_then`で合成。
3. `Option`との関係
   - 値がないだけなら`Option`、失敗理由が必要なら`Result`。
4. カスタムエラー
   - 素のenumでの定義と`thiserror`による簡潔な実装。
   - `anyhow`でアプリ層のエラーをまとめるパターン。

## デモコード（ライブ用）
```rust
use std::{fs::File, io::{self, Read}};

fn read_first_line(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?; // ?で早期リターン
    f.read_to_string(&mut s)?;
    Ok(s.lines().next().unwrap_or_default().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line = read_first_line("Cargo.toml")?;
    println!("{line}");
    Ok(())
}
```

## ハンズオン課題（講義中）
1. ファイルの数値を読み込み合計する関数を`Result<i64, io::Error>`で実装（`?`を使用）。
2. `Option`を`ok_or`/`ok_or_else`で`Result`に変換する2通りを実演。
3. `Result`を`match`で処理し、`Err`のときにユーザー向けメッセージを出すCLIを作る。

## 練習問題（宿題）
1. `anyhow::Result`で同じ関数を書き換え、`?`だけで伝搬できる箇所を比較する。
2. `thiserror`で独自エラー(enum)を定義し、表示メッセージを整える。
3. `map_err`を使ってIOエラーをアプリ独自のエラー型に変換する例を作る。

## チェックリスト
- [ ] `?`演算子のデシュガーを説明した
- [ ] `Result`と`Option`の使い分けを示した
- [ ] パニックとリカバリ可能エラーの違いに触れた
- [ ] カスタムエラー/anyhowの導入意図を説明した

## 補足資料
- Rust Book 9章「エラー処理」
- Rust By Example: `error/result`, `error/multiple-error-types`
- サンプル: `samples/ch06_error_handling`
