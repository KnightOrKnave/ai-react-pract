# チャプター7講座資料: モジュール・テスト・Cargo活用

## 目的と持ち帰り
- Rustのモジュール構成（ファイルと`mod`/`pub`の対応）を説明できる。
- 単体テストと結合テストの書き方をひととおり体験する。
- `cargo fmt`/`cargo clippy`/`cargo test`の基本ルーチンを身につける。

## 講義アウトライン
1. モジュールと公開範囲
   - `mod foo;`は`foo.rs`または`foo/mod.rs`を探す。
   - `pub`で外部公開、`pub(crate)`でcrate内公開。
   - `pub use`で公開APIを整理するパターン。
2. テストの種類
   - 単体テスト（`#[cfg(test)] mod tests {}`）と統合テスト（`tests/`ディレクトリ）。
   - `cargo test name`で特定テストのみ実行。
3. Cargoツールチェーン
   - `cargo fmt`で整形、`cargo clippy`で静的解析、`cargo test`で回帰確認。
   - 開発ループの推奨順序：fmt → clippy → test。

## デモコード（ライブ用）
```rust
// src/lib.rs
pub mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

pub use math::add;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_two() {
        assert_eq!(add(2, 3), 5);
    }
}
```

統合テスト例:
```rust
// tests/api.rs
use mycrate::add;

#[test]
fn add_works() {
    assert_eq!(add(1, 1), 2);
}
```

## ハンズオン課題（講義中）
1. `src/lib.rs`を2つのモジュールに分割し、`pub use`で外部に公開するAPIを整える。
2. `tests/`配下に統合テストを1本追加し、`cargo test -- --nocapture`で出力を観察する。
3. `cargo fmt`と`cargo clippy -- -D warnings`を実行し、警告修正の流れを体験。

## 練習問題（宿題）
1. サブルーチンを`math`, `util`など複数モジュールに整理し、`pub(crate)`を使って内部公開を制御する。
2. 失敗するテストを1つ追加し、`cargo test`の失敗出力を読み解く練習をする。
3. `cfg(test)`内でのみ有効なヘルパ関数を定義し、テストコードの重複を減らす。

## チェックリスト
- [ ] `mod`とファイル配置の対応を示した
- [ ] `pub`/`pub(crate)`/`pub use`の使い所を説明した
- [ ] 単体テストと統合テストの違いを紹介した
- [ ] fmt/clippy/testの開発ルーチンを実演した

## 補足資料
- Rust Book 7章「モジュール」, 11章「テスト」
- Rust By Example: `mod`, `testing`
- サンプル: `samples/ch07_modules_tests`
