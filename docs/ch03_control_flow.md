# チャプター3講座資料: 制御フローとマッチング思考

## 目的と持ち帰り
- `if/else`, `match`, `loop/while/for`を「式」として使えることを理解する。
- パターンマッチングとガードで条件分岐を表現できる。
- `loop`から値を返すテクニックを説明できる。

## 講義アウトライン
1. `if`は式
   - `let label = if cond { "yes" } else { "no" };`
   - ミスマッチな型はコンパイルエラーで弾かれる。
2. `match`の網羅性
   - すべてのパターンを列挙しないとコンパイルエラー。
   - ワイルドカード `_` とガード `if` の組み合わせ。
3. 繰り返し
   - `loop`は`break value`で値を返せる。
   - `while let`でOption/Iteratorを安全に処理。
   - `for`はIteratorベースで`into_iter/iter/iter_mut`の違いを意識する。

## デモコード（ライブ用）
```rust
fn classify(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        1..=9 => "small positive",
        _ => "large positive",
    }
}

fn main() {
    let mut count = 0;
    let first_over_10 = loop {
        count += 3;
        if count > 10 {
            break count; // 値を返すloop
        }
    };
    println!("{first_over_10} is {}", classify(first_over_10));
}
```

## ハンズオン課題（講義中）
1. 1〜6の乱数を`match`で分類し、1なら"One", 6なら"Six!", その他は"Middle"を出力。
2. `while let Some(x)`で`Vec`をスタックとして消費するコードを書き、`for`版との違いをコメントで整理。
3. `loop`を使って`Vec`から最初の偶数を見つけ、その値を`break`で返す関数を実装。

## 練習問題（宿題）
1. FizzBuzzを`match`中心で実装し、`if`版と可読性を比較するコメントを付ける。
2. `Option<Result<T, E>>`を`match`で分解し、`Ok`のときに値を取り出し、それ以外は別メッセージを返す関数を作る。
3. `match`ガードを使って、整数を「10未満」「10〜99」「100以上」に分類する関数を実装。

## チェックリスト
- [ ] `if`が式であることを示した
- [ ] `match`の網羅性チェックをエラーで確認した
- [ ] `loop { break value; }`のパターンを紹介した
- [ ] `while let`と`for`の違いを示した

## 補足資料
- Rust Book 6章「Enumとパターンマッチング」、3章「制御フロー」
- Rust By Example: `flow-control`, `match`
- サンプル: `samples/ch03_control_flow`
