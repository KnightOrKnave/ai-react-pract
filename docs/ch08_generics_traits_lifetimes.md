# チャプター8講座資料: ジェネリクス・トレイト・ライフタイム応用

## 目的と持ち帰り
- ジェネリック型/関数の基本シンタックスと`where`句を使い分けられる。
- トレイト境界の設計意図を説明できる。
- トレイトオブジェクトと静的ディスパッチの違いを理解する。
- ライフタイム注釈が必要になる代表パターンを挙げられる。

## 講義アウトライン
1. ジェネリクスの基本
   - `fn foo<T>(x: T) -> T`と`impl<T> Type<T>`.
   - `where`句で長い境界を整理。
2. トレイト境界とディスパッチ
   - 静的ディスパッチ: `impl Trait` / `T: Trait`.
   - 動的ディスパッチ: `Box<dyn Trait>` と vtable。
3. トレイトオブジェクトの制約
   - オブジェクトセーフティ: `Self: Sized`が絡むメソッドは不可など。
4. ライフタイムの応用
   - 参照を保持する構造体に明示ライフタイム。
   - 返り値の参照が入力どれと結びつくかを注釈。

## デモコード（ライブ用）
```rust
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self { Self { left, right } }
}

impl<T: std::ops::Add<Output = T> + Copy> Pair<T> {
    fn sum(&self) -> T { self.left + self.right }
}

trait Renderer {
    fn render(&self) -> String;
}

struct Html(String);
struct Json(String);

impl Renderer for Html {
    fn render(&self) -> String { format!("<html>{}</html>", self.0) }
}
impl Renderer for Json {
    fn render(&self) -> String { format!("{{\"data\":\"{}\"}}", self.0) }
}

fn render_all(items: &[Box<dyn Renderer>]) -> Vec<String> {
    items.iter().map(|r| r.render()).collect()
}
```

## ハンズオン課題（講義中）
1. `Pair<T>`に`swap`と`as_tuple`を実装し、`T: Clone`制約を付けた場合と外した場合を比較する。
2. `trait Drawable { fn draw(&self) -> String; }`を複数型で実装し、`Vec<Box<dyn Drawable>>`をループで描画。
3. 参照を保持する構造体にライフタイム注釈を付け、`'static`文字列と一時文字列で挙動の違いを確認。

## 練習問題（宿題）
1. `impl Trait`引数とジェネリック型パラメータ引数で同じ関数を書き、シグネチャ差分を比較する。
2. `Fn`トレイト境界を持つ高階関数を1つ実装し、クロージャを渡して動かす。
3. トレイトオブジェクトを返す関数（`-> Box<dyn Renderer>`）と、ジェネリックで返す関数（`-> impl Renderer`）の使い分けをコメントにまとめる。

## チェックリスト
- [ ] ジェネリクス基本シンタックスと`where`句を示した
- [ ] 静的/動的ディスパッチの違いを説明した
- [ ] トレイトオブジェクトの制約（オブジェクトセーフティ）に触れた
- [ ] ライフタイム注釈が必要なケースを紹介した

## 補足資料
- Rust Book 10章「ジェネリクス、トレイト、ライフタイム」
- Rust By Example: `generics`, `trait`, `fn/closures`
- サンプル: `samples/ch08_generics_traits`
