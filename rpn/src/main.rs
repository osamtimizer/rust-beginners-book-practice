fn main() {
    //letは新しい変数を用意し、右辺式の評価後に得られた値に変数を束縛する
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);

    //デバッグビルド時のみ展開され、リリースビルド時は無視される
    //テストみたいなもの
    debug_assert_eq!("26.2840", format!("{:.4}", ans));
    println!("{} = {:.4}", exp, ans);
}

// 文字列expを受け取り、f64を返す関数
fn rpn(exp: &str) -> f64 {
    //mutableな変数を指定したい場合は、mutキーワードを使う
    //Vector型と言うのは、要素数が可変の配列のようなデータ構造
    //push/popでスタックのように扱える
    let mut stack = Vec::new();

    //split_whitespace()はenumerableなもの（多分Array）を返す
    //exp.split_whitespace()はメソッド呼び出し構文と呼ばれる
    //コンパイル時にsplit_whitespace(&exp)と言う関数呼び出しとして解釈される
    //&expとすると、expが束縛された文字列へのポインタが得られる
    for token in exp.split_whitespace() {
        // parseではtokenがf64型に変換できるかをチェックしている
        // たとえば、"6.1"なら、parseメソッドはOk(6.1)を返す
        // これをif letで受け取ると、数値の場合は条件が成立し、
        // ベクタのpushメソッドでその値をスタックの最後に追加する
        // parseはtokenが数値でないなら、Err(エラーの詳細を表す値)を返す
        if let Ok(num) = token.parse::<f64>() {
            //スタックに積む
            stack.push(num);
        } else {
            // tokenが数値ではないなら、演算子なのか調べる
            match token {
                // &must stackと書くことで、stackが束縛されたベクタへの参照を、変更可能で渡している
                // 第二引数はクロージャ（無名関数の一種）
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // セミコロンがついていない場合、その式が返した値を、関数の戻り値とする
    stack.pop().expect("Stack underflow")
}

// スタックから数値を2つ取り出し、F型のクロージャfunで計算し、結果をスタックに積む
// ミュータブルなVec<f64>型のstackと、F型のfunを取るメソッド。Fはこの関数がジェネリクスであることを表す。
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
// Fは、where節で指定したトレイト境界を満たす型であれば、何にでもなれる
// ジェネリクスな関数で、渡ってきた型によって分岐させたいときに使う模様
// Javaとかだと<T extends HogeClass>みたいに書けるやつ
where
    F: Fn(f64, f64) -> f64,
    {
        // 変数yとxをスタックの最後の2要素に束縛する
        if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
            let z = fun(x, y);
            stack.push(z);
        } else {
            panic!("Stack underflow");
        }
    }