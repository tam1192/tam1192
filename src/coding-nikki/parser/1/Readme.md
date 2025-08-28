# 構文解析 part1

公開日：

構文解析について、お話しします。

# ひとくちメモ

彩澄姉妹が最近好きでして。  
初めて声を聞いたのは、ほんの偶然というか。ボイロ劇場みてたらナレーターとして使われてた。

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45241088" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45241088">マイライフロガー / いるかアイス feat. 彩澄しゅお</a></iframe>

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45271857" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45271857">【歌うVOICEPEAKカバー】メランコリック【彩澄しゅお】</a></iframe>

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm45244373" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm45244373">稲葉曇『春難色』Vo. 彩澄しゅお & 彩澄りりせ</a></iframe>

<iframe width="312" height="176" src="https://ext.nicovideo.jp/thumb/sm44383609" scrolling="no" style="border:solid 1px #ccc;" frameborder="0"><a href="https://www.nicovideo.jp/watch/sm44383609">しゅおと宮舞のお昼寝！</a></iframe>

りりねぇが少なめですまねぇ。~~決してアレではない~~

# ポインタ

C を学ぶ時、ポインタを知ります。 ポインタってどこで使うんだとなるわけです。

## 指定した容器に情報を詰め込んで

今マイバック持参とかあるじゃないですか。あとマイカゴ。  
マイカゴって、スーパーのレジ店員に渡すと**そのまま品物を中に詰めてくれる**ので、こちらとしてそのまま車に詰め込んで、持ち帰って終わるだけなので楽なんですよね。

```rust
#[derive(Debug)]
struct Goods<'str> {
  _name: &'str str
}

fn registrar<'str>(mut unpurchased: Vec<Goods<'str>>, mybag: &mut [Option<Goods<'str>>]) -> (usize, Vec<Goods<'str>>) {
    let mut counter = 0;
    for i in mybag.iter_mut() {
        if i.is_none() {
            *i = unpurchased.pop();
            counter += 1;
        }
    }
    (counter, unpurchased)
}

fn main() {
    let unpurchased_list = vec![Goods{_name: "pan"}, Goods{_name: "kome"}, Goods{_name: "tofu"}];
    let mut my_bag: [Option<Goods>; 2] = [None, None];

    let (size, pra_bag) = registrar(unpurchased_list, &mut my_bag);

    println!("レジ袋: {:?}", pra_bag);
    println!("マイバッグに入れられた品物: {:?}", my_bag);
    println!("マイバッグに入れられた品物の数: {}", size);
}
```

というのを想像して書いてみたらこういうプログラムができました。  
`Clone`を実装してないので商品(`Goods`)は move のみできます。現実の商品コピーできないからね。

レジ(`registrar`)にはカゴ(`unpurchased_list`)とマイバッグ(`my_bag`)を渡します。  
なお、**マイバックは一時的に貸しているものであり、帰ってきます。** 参照渡しとはそういうことです。

会計が済めば、入り切らなかった商品がレジ袋(`pra_bag`)に詰められて帰ってきます。  
丁寧な店員なので、マイバッグに入った品物の数も答えてくれました。

<span style="color: red;">「あんちゃんが渡してくれた袋に二品つっこめたで」</span>

ポインタのほんの一つの使い方はこんな感じです。  
rust では一時的に貸すという表現ができますねぇ。

### そのまま渡して返して貰えばいいのでは？

~~レジ袋って無駄だと騒がれてるじゃん？~~  
C 言語の関数の戻り値ってアドレスが異なるのです。  
正しくは、関数の結果を変数にコピーするという動作を行ったはずです。

rust ではあんまり関係ないですが。少なからず戻り値での受け渡しは無駄が生じる可能性があるということです。

また、現実の my_bag 同様、**変数の再利用ができます。**
変数って作るときに**コスト**が生じます。 特に Box で作るとなると、ヒープ確保にコストがかかります。  
java なら GC があるので、GC が回収するオブジェクトが無駄に増えることになります。

変数って単純なようで奥深いです。 rust ではトレイトがあります。  
トレイトオブジェクトでリストを管理すれば、変数の扱いを自分なりにカスタマイズできるようになるのです。

とにかく、**自分が用意したものを相手に使ってもらいたい時に、ポインタは有用だということなのです。**

> [!NOTE]  
> rust では参照です。

## 関数をやり取りする時

関数って変数ではないです。  
変数って数値とかそういう物だと思います。 関数は一連の処理を纏めた物です。 変数とはいえませぬ。

関数に対して「アレ」とはいえますが、「コレ」といって渡すことができません。 関数って移動できないから。

難しい概念です。

ミキサーで考えると、関数がミキサーになり、入れるバナナと牛乳が変数になり、バナナ牛乳ができるわけです。
関数の引数にバナナと牛乳、出力がバナナ牛乳となります。

ミキサーは一家に一台ぐらいしかないです。複数いらないから。  
バナナと牛乳は一家にたくさんあります。たくさん食べたいから。

また、ミキサーは加工できません。一般的に  
関数も加工できません。基本的に

なので、関数（ミキサー）を引数の一部に使いたいなら、「あれ」というしかないのです。

### コールバック関数

関数を「あれ」と言いたいときはいつでしょう。 コールバック関数が挙げられます。

すぐに欲しくないけれど、必要になったら作り出すという時に使えるでしょうか。

```rust
use std::time::Duration;
use std::thread::sleep;
fn callback(func: impl FnOnce()) {
    sleep(Duration::from_secs(2));
    func();
}

fn func() {
    println!("hello world");
}

fn main() {
    let f = func;
    callback(f);
}
```

rust では`()`を抜くだけで関数がオブジェクトになります。
C では関数ポインタで扱います。

callback 関数は、**ライブラリ作者がユーザーの関数を適切なタイミングで呼び出す時に便利に使われます。**  
例えば時間のかかるファイルが読み込み終わった後とか。

> [!NOTE]  
> イベント駆動する javascript では、イベントとして関数を登録し、ファイルが読み込み終わった時などにイベント発火として関数が呼び出されます。

# コンビネーター

callback 関数は他に、関数を加工する時、関数を組み合わせる時に便利に使われます。

```rust
fn can_i_help_you() {
    println!("いらっしゃいませこんにちは");
}

fn booko0f(func: impl Fn()) {
    for _ in 0..3 {
        func();
    }
}

fn main() {
    booko0f(can_i_help_you);
}
```

関数を組み合わせて機能を強くする例です。~~ブック ⚪︎ フか。~~

# パーサー

こういうやつです。

```rust
fn digit_parser(i: &str) -> (&str, Option<i32>) {
    let n = i.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    if let Ok(res) = i[0..n].parse::<i32>() {
        (&i[n..], Some(res))
    } else {
        (&i, None)
    }
}

fn main() {
    let x = "123hello";
    let (x, y) = digit_parser(x);
    println!("{:?}, {:?}", x, y);
}
```

文字列から数値だけ抽出します。 その後、数値と文字列を分割して出力します。  
rust ではシャドーイングする設計にすると綺麗です。

# パーサーコンビネーター

```rust
fn digit_parser(i: &str) -> (&str, Option<i32>) {
    let n = i.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    if let Ok(res) = i[0..n].parse::<i32>() {
        (&i[n..], Some(res))
    } else {
        (&i, None)
    }
}

fn looparser<P, T>(parser: P) -> impl Fn(&str) -> (&str, Option<(&str, T)>)
where P: Fn(&str) -> (&str, Option<T>)
{
    move |i| {
        for n in 0..i.len() {
            let (i2, res) = parser(&i[n..]);
            if let Some(res) = res {
                return (i2, Some((&i[..n], res)))
            }
        }
        (i, None)
    }
}

fn main() {
    let x = "hello123world";
    let (x, y) = looparser(digit_parser)(x);
    println!("{:?}, {:?}", x, y);
}
```

コードが急にカオスになりました。  
詳しいことは次回に回して、**やりたいことだけ言います**。

### looparser

~~名付け適当~~ digit_parser は「**数文字が先頭から続く範囲で**数値を取り出します。」  
つまり、途中の数値は取り出されません。

```rust
fn digit_parser(i: &str) -> (&str, Option<i32>) {
    let n = i.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    if let Ok(res) = i[0..n].parse::<i32>() {
        (&i[n..], Some(res))
    } else {
        (&i, None)
    }
}

fn main() {
    let x = "hello123";
    let (x, y) = digit_parser(x);
    println!("{:?}, {:?}", x, y);
}
```

数値が後ろになるだけでパースできなくなりました。
looparser では、**パーサーが成功するまで文字列の先頭をずらしながら試行錯誤する**という物です。

よって、"hello123world"では、6 文字目から digit_parser が成功し、数値に変換して返すということができるようになりましたとさ。

## パーサーという関数を受け入れる関数

looparser では、関数をあたかも**Parser というオブジェクトのように扱っています**。  
こういった時にも関数ポインタは必要になるのですね。

## コード解説

```rust, ignore
fn digit_parser(i: &str) -> (&str, Option<i32>) {
    let n = i.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    if let Ok(res) = i[0..n].parse::<i32>() {
        (&i[n..], Some(res))
    } else {
        (&i, None)
    }
}
```

i は文字列。 `str.find()`は、引数にコールバック関数を入れます。  
ここでもコールバック関数がっ...

コールバック関数の引数は char にしておきます。 そうすることで、find メソッドがコールバック関数に先頭から文字を一文字づつ入れて、試してくれます。  
コールバック関数の戻り値は bool です。 true を返すと、成功(=条件に合う文字)と判定されます。

つまり、find メソッドは先頭から一文字ずつ試していき、条件に会う文字を見つけたらその文字があるインデックスを返すというメソッドになります。

### 戻り値

戻り値は(&str, Option)という形をとってます。

```rust, ignore
let base = "123hello";
let (base, res1) = parser1(base);
let (base, res2) = if res1.is_none() {
    parser2(base)
} else {
    (base, None)
}
```

まぁこんな感じの書き方ができるよねってことで。

## コード解説 2

```rust, ignore
fn looparser<P, T>(parser: P) -> impl Fn(&str) -> (&str, Option<(&str, T)>)
where P: Fn(&str) -> (&str, Option<T>)
{
    move |i| {
        for n in 0..i.len() {
            let (i2, res) = parser(&i[n..]);
            if let Some(res) = res {
                return (i2, Some((&i[..n], res)))
            }
        }
        (i, None)
    }
}
```

`move |i| {}` これも rust では関数の一つとなります。 クロージャという物です。  
クロージャはある意味変数に近いです。 実行時に生成することができます。

### クロージャのキャプチャ

`move`というのは所有権関係の物です。  
looparser のスコープには`parser`というこれまた関数オブジェクトが存在します。  
この`parser`の所有権を、クロージャに移動することを認める一言になってます。

クロージャ内部では、キャプチャした parser オブジェクトがそのまま使えます。
つまり、**looparser でうけっとったコールバック関数を、クロージャ内部で使うことができる**ということになります。

looparser は戻り値で関数ポインタを渡してきますが、**そのクロージャでも looparser が受け取ったコールバック関数が引き続き利用可能です**。

# 今回のまとめ

ポインタからコールバック関数、そしてパーサーコンビネーターまでとりあえず纏めてみました。  
本題まで行けてないのが悲しい。

#
