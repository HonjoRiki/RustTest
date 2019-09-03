// インポート
// std::ioに関する多様な機能をインポートしている
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// fn: 関数
fn main() {
    // println!(): 文字列をスクリーンに出力するマクロ
    println!("Guess the Number!");

    // secret_numberという束縛変数(不変)
    // ローカルな乱数生成器のコピー取得。乱数の範囲は1~100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // ループ
    loop {
        println!("Please input your guess.");

        // let: 束縛変数
        // 変数guessに対してString::new()を束縛している
        let mut guess = String::new();
        // mut: 可変である
        // String::new(): 文字列型の新たな値を作っている

        //read_line(): 可変なguessに対して入力を要求している
        //expect(): 例外処理。メッセージをカスタマイズできる
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // rustでは既に定義している名前を新しいもので隠すことができる
        // Stringであるguessをu32(符号なし32ビット整数)に変換している
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number.");

        // {}: プレースホルダ。guessを渡している
        println!("You guessed: {}", guess);

        // match: 値を取り、それを評価できる
        match guess.cmp(&secret_number) {
            Ordering::Less  => println!("Too small!"),
            Ordering::Greater  => println!("Too big!"),
            Ordering::Equal  => {
                println!("You win!");
                break; // ループ終了
            }
        }
    }
}
