/*
ライブラリーは Rust では「クレート」と呼ばれます。
wasm_bindgen::prelude モジュールにあるものすべてをインポート
wasm-bindgen を利用して、JavaScript と Rust の型を繋いでいます。wasm-bindgen によって JavaScript が文字列に関する Rust API を呼び出すことや Rust の関数が JavaScript の例外をキャッチすることができるようになります。
*/
use wasm_bindgen::prelude::*;

/*
#[ ] の内側は「アトリビュート」と呼ばれ、次に来る文を修飾します。
externは外部で定義された関数を呼び出す時に使う。
*/
#[wasm_bindgen]
extern "C" {
    //このalert関数はJavascriptによって提供される関数
    pub fn alert(s: &str);
}

/*
外部で使いたいものをfnで定義
externブロックで要求したalert関数を呼び出します。
*/
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
