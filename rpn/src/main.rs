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
    let mut stack = Vec::new();

    //split_whitespace()はenumerableなもの（多分Array）を返す
    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            //スタックに積む
            stack.push(num);
        } else {
            // tokenが数値ではないなら、演算子なのか調べる
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
    {
    }