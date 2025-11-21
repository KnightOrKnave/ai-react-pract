# チャプター5講座資料: コレクションとイテレーション

## 目的と持ち帰り
- `Vec`/`HashMap`など標準コレクションの所有権ルールを理解する。
- `iter`/`iter_mut`/`into_iter`の違いと使い分けを説明できる。
- イテレータチェーンで宣言的に処理を書く感覚を掴む。

## 講義アウトライン
1. `Vec`の基本とムーブ
   - `push/pop`は要素をムーブする。
   - `for x in vec {}`は`into_iter`で所有権を消費する。
2. `HashMap`の所有権
   - `insert`でキー/値はムーブされる。
   - `get`は借用を返すため、取り出した値は参照。
   - `entry`APIで存在確認と挿入を一度に行う。
3. イテレータ
   - `iter`（不変参照）/`iter_mut`（可変参照）/`into_iter`（所有権）。
   - 代表的メソッド: `map`, `filter`, `fold`, `enumerate`, `collect`.
   - `collect`の型注釈例：`let v: Vec<_> = iter.collect();`

## デモコード（ライブ用）
```rust
use std::collections::HashMap;

fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for w in text.split_whitespace() {
        *map.entry(w.to_string()).or_insert(0) += 1;
    }
    map
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let squares: Vec<_> = nums.iter().map(|n| n * n).collect();
    println!("{squares:?}");

    let freq = word_freq("hello hello rust");
    for (k, v) in freq.iter() {
        println!("{k}: {v}");
    }
}
```

## ハンズオン課題（講義中）
1. `VecDeque`を使い、`push_front/push_back`と`pop_front/pop_back`でキュー/スタック両方を試す。
2. `HashMap<String, usize>`で単語頻度を取り、`Vec`に`collect`して上位3件を表示する。
3. イテレータチェーンのみでフィボナッチ数列の先頭10個を生成する（ループ禁止）。

## 練習問題（宿題）
1. `map`と`filter`を組み合わせ、1〜20の偶数だけを2倍して合計をとる。
2. `iter_mut`を使い、`Vec<String>`をすべて大文字化する関数を実装する。
3. `HashMap`の`entry`を使って、存在しないキーにデフォルト値を入れる処理をまとめる。

## チェックリスト
- [ ] `Vec`の所有権と`for`ループの関係を示した
- [ ] `HashMap`の`insert/get/entry`の所有権挙動を説明した
- [ ] `iter`/`iter_mut`/`into_iter`の違いを確認した
- [ ] 代表的なイテレータメソッドをデモした

## 補足資料
- Rust Book 8章「コレクション」
- Rust By Example: `vec`, `hashmap`, `iterators`
- サンプル: `samples/ch05_collections_iter`
