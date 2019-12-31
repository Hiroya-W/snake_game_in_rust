# Rustに入門したい
ゲームを作りながらRustを勉強してみたいんです。

海外の方がYoutubeでRust向けゲームエンジンである`piston`を用いて、スネークゲームを作ってる動画を見つけたので、
翻訳しながら、自分が勉強したことをまとめていこうと思います。
[Making a Snake Game in Rust](https://www.youtube.com/watch?v=HCwMb0KslX8)

この記事での完成形は、こんな感じ。この記事では、スネークが動くところまで実装します。

<img src="/home/hiroya/Pictures/ScreenShots/Snake Game_065.png" alt="Snake Game_065" style="zoom:50%;" />



**Rust初心者が書く記事ですので、間違ったことを書いてしまっていた場合はご指摘いただけると幸いです。よろしくおねがいします。**

[200行のRustでスネークゲーム](https://qiita.com/elpnt/items/fb948105eeb41cb3629b)を作られた方も。
記事を書き終えてから、ぜひソースコードを眺めて理解したいです。

Rustの開発環境の準備はとても簡単なので省略します。
記事も書かれているので参考に。Rustのアップデート方法もここにお世話になりました。
[rustup で Rust コンパイラーを簡単インストール](https://qiita.com/chikoski/items/b6461367e8c3875bb235)

# 開発環境
- Manjaro Linux 18.1.5
- Visual Studio Code 1.14.1
- rustup 1.12.1
- rustc 1.40.0
- cargo 1.40.0

Rustのアップデートは以下のコマンドを叩く。

```
$ rustup update
```

# プロジェクトの構築

[PistonDevelopers/Piston-Tutorials](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)にもプロジェクトの構築手順が記載されている。
こちらも適宜参考にする。

プロジェクトを作成します。
`--bin`オプションの他にも、`--lib`オプションがある。

- `--bin` : ビルドした際に、実行可能ファイルを作成する場合
- `--lib` : ビルドした際に、他のRustパッケージから利用できるライブラリファイルを作成する場合

というように使い分け、どちらも指定しなければ、`--bin`が使用される。



```
$ cargo new snake_game --bin
```



次に依存関係のインストールを行う。

[Setting Up The Project](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started#setting-up-the-project)に書かれている`dependencies`をコピペする。



```toml
:Cargo.toml
[package]
name = "snake_game"
version = "0.1.0"
authors = ["Hiroya_W <mail@address>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
piston = "0.49.0"
piston2d-graphics = "0.35.0"
pistoncore-glutin_window = "0.63.0"
piston2d-opengl_graphics = "0.69.0"
```

コマンドを実行する。
これにより、すべての依存関係が取得され、コンパイルされます。

```
$ cd snake_game
$ cargo build
```

ドキュメントを作成する。`--open`オプションをつければ、ブラウザでドキュメントが開く。
あとで使うらしい。

```
cargo doc --open
```



# いざ

# 外部クレートの読み込み(OLD)

クレートとは、他の言語における「ライブラリ」やパッケージと同じ意味です。
先程の依存関係のインストールでは、クレートがインストールされます。

このクレートを使用することを**Rust 2015**では記述する必要がありました。
ので、こう書きます。

```rust
main.rs
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

fn main() {
    println!("Hello, world!");
}
```

現在は**Rust 2018**がリリースされ、それを利用しています。
Rust 2018からは書く必要が無くなったようです。

> `extern crate`が不要になり、`use`文で他クレートのシンボルを直接インポートできるようになります。
>
> [Rust 2018のリリース前情報](https://qiita.com/garkimasera/items/1bc973eae60fe0c10210)

消しちゃいましょう。

以下のように`use`文を用いて他の名前空間にあるプログラムの要素をインポートします。

```rust
main.rs
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
```

この時、アスタリスクには注意する必要があります。

アスタリスクを用いることで、私達の名前空間にその名前空間の全てを読み込むことになります。

使用している関数や変数がどこから読み込んだものなのか見失ってしまったり、意図しない関数や変数を読み込んでしまう原因になります。

当時は、サンプルプログラムがアスタリスクを使用して記述されていたようです。現在は使わない記述がされているので、そのように修正します。

しなければならない、ではなく、した方がいい、という部分ですね。

# 外部クレートの読み込み(NEW)

結局のところ、外部クレートを読み込む部分の記述は以下のようになります。サンプルプログラムのように、`as`を使えば、自分で分かりやすい名前を付けて読み込めるようですが、あえてここでは使わずに進めます。以降、本記事では、記載するコードブロックが長くなってしまうため、`use`文の記述を省略します。

```rust
main.rs
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
```

# アプリの設計のお話

画像は参考元動画から使用しています。

アプリの一般的なデザインは次の通りです。

ゲームを表すBlobデータまたは構造体があります。

この構造は、ウィンドウに描画する「レンダリング」メソッドを持っています。

<img src="/home/hiroya/Documents/Git-Repos/snake_game/figures/Making a Snake Game in Rust 2-4 screenshot.png" alt="Making a Snake Game in Rust 2-4 screenshot" style="zoom:50%;" />

キー入力やスネークの当たり判定など、ゲーム内の全てのものはレンダリングとは別に発生します。

それらは、このBlobデータを更新するだけです。

<img src="/home/hiroya/Documents/Git-Repos/snake_game/figures/Making a Snake Game in Rust 2-7 screenshot.png" alt="Making a Snake Game in Rust 2-7 screenshot" style="zoom:50%;" />

<img src="/home/hiroya/Documents/Git-Repos/snake_game/figures/Making a Snake Game in Rust 2-11 screenshot.png" alt="Making a Snake Game in Rust 2-11 screenshot" style="zoom:50%;" />

もちろん、その時Blobデータはレンダリングするように伝えられるので、スネークが歩き回るアニメーションを見ることができます。

<img src="/home/hiroya/Documents/Git-Repos/snake_game/figures/Making a Snake Game in Rust 2-18 screenshot.png" alt="Making a Snake Game in Rust 2-18 screenshot" style="zoom:50%;" />

# ウィンドウの作成

最初のステップはウィンドウを作成することです。

main関数では、`opengl`変数と`GlutinWindow`を作成します。

```rust
main.rs
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window : GlutinWindow = WindowSettings::new(?,?); // この使い方は...？
}
```

サンプルコードを見れば、この静的メソッドの使い方がわかります。

しかし、そうしない場合はどうすればいいのでしょうか？



## ドキュメントを見てみる

プロジェクトの作成時に作成したドキュメントを使用し、

`WindowSetting`を検索してみます。

出てきました！

![範囲を選択_054](/home/hiroya/Pictures/ScreenShots/範囲を選択_054.png)

見てみると、まず、`title`はジェネリック型の`T`を持ち、`String`型に変換できる必要があることが分かります。

また、ジェネリック型の`S`を持つ`size`もあり、`Size`型に変換できる必要があります。

![](/home/hiroya/Pictures/ScreenShots/範囲を選択_053.png)

**わかった？**

もちろん、わかりません。`Size`型とは何でしょうか？

`Size`をクリックしてみれば、分かります。

![](/home/hiroya/Pictures/ScreenShots/範囲を選択_055.png)

つまり、`Trait Implementations`に書かれている4つのうち、どれかの形で記述すれば良いのです。

## コードを書く

ウィンドウのタイトル、幅と高さを記述し、続くメソッドはサンプルを参考に記述していきます。

```rust
main.rs
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
}
```

## 実行してみる

以下のコマンドでビルドしてから実行することができます。

```
$ cargo run
```



今の状態では、ウィンドウが開くと、すぐに閉じるという動作をします。なぜなら、プログラムのmain関数の最後まで実行されたため、プログラムが終了するからです。

# イベントループ

## コードを書く

もう少しサンプルからコピペします。動画じゃコピーパスタって言ってて面白かった。

もちろん、常に責任を持ってコピペをするようにしてください。

```rust
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) { // ここは何をしているのでしょうか？
        if let Some(args) = e.render_args() {
            // app.render(&args);
        }
    }
}
```

さて、`event.next`は何をするのでしょうか？

## ドキュメントを見てみる

`event_loop::Events`を検索します。

![範囲を選択_056](/home/hiroya/Pictures/ScreenShots/範囲を選択_056.png)

見つかりました！探すのが上手になりましたね！

![範囲を選択_057](/home/hiroya/Pictures/ScreenShots/範囲を選択_057.png)

`Option<Event>`の`Event`をクリックして見てみると、

`Event`は`列挙型(Enum)`であり、`Input`,`Loop`,`Custom`の異なる表現があることが分かります。

![範囲を選択_058](/home/hiroya/Pictures/ScreenShots/範囲を選択_058.png)

`Trait Implementations`には、全て異なるイベントのトレイトが書かれています。

これらは全て、`Event`列挙体にメソッドを追加します。

ここでは、基本的に、受け取ったイベントの型を確認することができます。

![範囲を選択_059](/home/hiroya/Pictures/ScreenShots/範囲を選択_059.png)

知りたい`RenderEvent`トレイトの`render_args`メソッドはオプション型`Option<RenderArgs>`の型を返す、ということが分かります。

オプション型は値があれば、`Some`で値を包み、無いときは`None`が使われます。

## if let文を眺めてみる

次に`if let`文によるパターンマッチを眺めてみます。

```rust
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() { // ここのif let文を眺める
            // app.render(&args);
        }
    }
```



この`if let`文が意味するのは、「`Event`が`RenderEvent`である時、何か処理をする」ということである。

`Event`が`RenderEvent`でない時、`render_args`メソッドは`None`返します。
つまり、`if let`文でのパターンマッチに失敗するので、処理は行われません。



## App構造体を作る

`App`という構造体を作り、`GlGraphics`を持たせます。

`GlGraphics`は、ウィンドウ内に物を描画する役割を持ちます。

続いて、構造体に`render`メソッドを追加します。

```rust
struct App{
    gl : GlGraphics,
}

impl App {
    fn render(&mut self, arg: &RenderArgs) {}
}

fn main() {
    //...
}
```

RustではPythonのように動き、`render`メソッドの最初の引数`self`はメソッドを呼び出すインスタンス(レシーバともいう)が渡されます。

[rubyのレシーバとは](https://qiita.com/you8/items/e5f5c27cfed60a23fa75)

また、Rustなので、メソッドがインスタンスを取得する方法を指定できます。

- `self` : メソッドを呼び出すインスタンスの所有権がメソッドに移動する
- `&self` : メソッドを呼び出すインスタンスをイミュータブルな参照として使用する
- `&mut self` : メソッドを呼び出すインスタンスをミュータブルな参照として使用する

今回の`render`メソッドでは、`&mut self`として使用します。

なぜなら、画面への描画はミュータブルである必要があるからです。

また、`render`は`RenderArgs`イベントへの参照も行います。

### 色を作る

緑色の背景がほしいので、色を作ります。

色は`Red`,`Green`,`Blue`,`Opacity`の4つの値を持ち、
値の範囲は0.0~1.0でなければなりません。

```rust

impl App {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    }
}
```

### 描画する部分

グラフィックスレンダラーを用いて描画させます。

そのためには、レンダラーイベントから取得できる`viewport`とクロージャが必要です。

クロージャは`draw`メソッドにより呼び出される無名関数でとして働き、

コンテキストと自分自身を2つの引数として渡します。

グラフィックスライブラリの`clear`メソッドを使用して、ウィンドウの色を指定できます。



```rust
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });
    }
}
```

## App構造体を使う

main関数で作成した`App`構造体を初期化し、`render`メソッドを呼び出します。

```rust
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // 初期化
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            // メソッド呼び出し
            app.render(&args);
        }
    }
}
```

## 実行してみる

緑色のウィンドウが描画されれば成功です！

<img src="/home/hiroya/Pictures/ScreenShots/Snake Game_060.png" alt="Snake Game_060" style="zoom:50%;" />

ウィンドウタイトルはスクリーンショットを撮る関係上消えてしまってますが、ちゃんと`Snake Game`と表示されています。

これを、スネークゲームにしていきます。

# スネークゲームを作る

## スネーク構造体を追加する

`App`構造体に、`snake`フィールドを作る

```rust
struct App {
    gl: GlGraphics,
    snake: Snake,
}
```

そして、とても簡単な`Snake`構造体を作ります。

`Snake`構造体は`X座標`と`Y座標`を持ち、`render`メソッドを持たせます。

`render`メソッドには、`GlGraphics`を引数に渡すことで`render`イベントだけでなく、自分自身を描画できるようにします。

```rust
struct Snake {
    pos_x: i32,
    pos_y: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
}
```



正方形を描画するコードは、`App`の `render`メソッドと非常に似ていて、`App`の`render`メソッドでウィンドウを緑1色に`clear`した後、このメソッドを呼び出す必要があります。

```rust
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }
}
```

`App`構造体のインスタンスで、`Snake`を初期化することを忘れないように。

```rust
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 50,
            pos_y: 100,
        },
    };
```

## ひとまず実行してみる

<img src="/home/hiroya/Pictures/ScreenShots/Snake Game_061.png" style="zoom:50%;" />

赤色の正方形が表示されれば成功です！

これで、基本的なスネークゲームを作るための必要なものが揃いました。

## 他のイベント

### キー入力をする

キーボードイベントは別の種類の`Event`ですが、`RenderEvent`と同じ方法で取得できます。

![範囲を選択_062](/home/hiroya/Pictures/ScreenShots/範囲を選択_062.png)

### ゲームを更新する

ゲームを更新するための`UpdateEvent`もあります。

例えば、スネークが移動するとか。

![範囲を選択_063](/home/hiroya/Pictures/ScreenShots/範囲を選択_063.png)

## グリッドで移動するようにする

スネークは、1つの正方形ではなく、単なる正方形が連なったものです。

<img src="/home/hiroya/Documents/Git-Repos/snake_game/figures/Rustでスネークゲームを作成する 5-46 screenshot.png" alt="Rustでスネークゲームを作成する 5-46 screenshot" style="zoom:50%;" />

スネークに方向の情報を持たせることにします。

```rust
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
```

ゲームをグリッドとして表示するようにスネークを修正します。

```rust
struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}
```

```rust
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 0,
            pos_y: 0,
            dir: Direction::Right,
        },
    };
```

`render`メソッドを修正し、グリッドに従って移動するようにします。

```rust
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square =
            graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
}
```

今、ウィンドウサイズは`200x200`としています(単位はpixel)。

1マスのサイズは`20x20`とするなら、`10x10`のグリッドに区切ることができます。

この時、スネークの場所をグリッドの座標を用いて表すと、

- 左上 pos_x : 0, pos_y : 0
- 右下 pos_x : 9, pos_x : 9

と表せます。

では、`UpdateEvent`でスネークが移動するように、`App`構造体に`update`メソッドを追加します。

```rust
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args(){
            app.update();
        }
    }
```

`App`構造体の`update`メソッドで、`Snake`の`update`メソッドを呼び出すようにします。

```rust
impl App {
    fn render(&mut self, args: &RenderArgs) {
    	// ...
    }

    fn update(&mut self){
        self.snake.update();
    }
}
```

`Snake`の`update`メソッドでは、スネークが向いている方向に移動するようにします。パターンマッチを使って座標を変更します。

```rust
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
    	// ...
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x -= 1,
            Direction::Right => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        }
    }
}
```

## とりあえず実行してみる

ものすごい速さで右側に移動するのが見えたでしょうか...。

## イベントの更新回数を調節する

`ups`メソッドを使って1秒あたり8回更新されるようにします。

`EventLoop`の名前空間を読み込んで、

```rust
use piston::event_loop::{EventSettings, Events,EventLoop};
```

`ups`メソッドを呼び出します。

```rust
    let mut events = Events::new(EventSettings::new()).ups(8);
```

これで実行してみれば、いい感じに移動するようになっていると思います。



## キー入力で方向を切り替える

キー入力のイベントを使用するために、名前空間を読み込みます。

```rust
use piston::input::{
    keyboard::Key, Button, ButtonEvent, ButtonState, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};
```

キー入力があった時に、`pressed`メソッドを呼び出すようにします。

```rust
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press{
                app.pressed(&args.button);
            }
        }
```

`pressed`メソッドを実装します。

`last_direction`に所有権を渡すわけではないので、`clone`で値をコピーします。

入力されたキーをパターンマッチで判断し、更にパターンに条件(ガード)をつけます。

こうすることで、スネークの方向転換に制限をかけます。

```rust
impl App {
    fn render(&mut self, args: &RenderArgs) {
        // ...
    }
    fn update(&mut self) {
        // ...
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}
```

でも、これにはバグがあり、コンパイラは`Direction`を比較しようとするとエラーを吐きます。

そもそも、それらが等しいという概念を実装していないからです。

```rust
#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
```

`derive`で自動的に実装できます。とても簡単でした...。

## ついてくる尻尾を追加する

ついてくる尻尾を追加します。

そのために、標準ライブラリの`LinkedList`を使用します。

```rust
use std::collections::LinkedList;
```

各尻尾のX座標,Y座標をペアとして持つ`LinkedList`にスネークを修正していきます。

```rust
struct Snake {
    body: LinkedList<(i32,i32)>,
    dir: Direction,
}
```

そうすると、スネークの移動と、レンダリングの部分のコードを書き変えなければなりません。

移動するには、新しいスネークの頭、もしくは、リストの先頭をクローンします。先程と同様の理由で、`clone`しなければならないことに注意してください。

方向に基づいて頭の位置を更新し、`LinkedList`の先頭に追加し、最後の要素を取り除きます。

移動させるというよりかは、進行方向に新しい頭を置き、尻尾を1個消す、みたいなイメージ。

```rust
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
		// ...
	}

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1-= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}
```

これをレンダリングしていきます。

レンダリングは各尻尾の部分に対して行いますが、本質的には全て同じことを行います。

まず、`Snake.body`を`iter`を使ってイテレータを回します。それぞれの要素において、`map`を使って、座標のペアを`x`と`y`の変数にマッピングして正方形を作成したものをベクトル型に格納していきます。

次に、作成したベクトル型の`squares`に対して、`into_iter`を使って、イテレータを回し、それぞれの正方形を描画していきます。

イテレータには3種類あり、

- `iter(&self)` : 各要素を`&T`型で返すイテレータ
- `iter_mut(&mut self)` : 各要素を`&mut T`型で返すイテレータ
- `into_iter(self)` : 各要素を`T`型で返すイテレータ

`into_iter`メソッドは引数が`self`で、コレクションの所有権が移動します。なので、一度呼ぶとそのコレクションにはアクセスできなくなります。

参照の場合は、`iter`メソッド、ですね。

```rust
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    fn update(&mut self) {
		// ...
    }
}

```

main関数の`snake`インスタンスでの初期化も修正しなければなりません。

`from_iter`メソッドを使って、ベクトル型から`LinkedList`を作成します。

`FromIterator`をインポートします。

```rust
use std::iter::FromIterator;
```

スネークの初期位置は、`(0,0)`と``(0,1)`となります。

```rust
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
    };
```

# 実行してみる

<img src="/home/hiroya/Pictures/ScreenShots/Snake Game_065.png" alt="Snake Game_065" style="zoom:50%;" />

静止画でわかりにくくてごめんなさい。

いつかGIF画像に置き換えます。

こんな感じに、キー入力で移動、尻尾がついてくるところまで実装できました。



# 一旦ここまで

Youtubeでの解説はここまでで終わりでした。

ので、この記事も一旦ここまで、としたいと思います。

まだ移動までしか出来てなくて、ここからゲームらしさが追加されていくことになるので楽しみですね。

完成形のソースコードへのリンクはYoutubeの概要欄に記載されているので、次回はそれを参考に完成させたいと思います。

ここまで書いてみて、Rust特有の仕様、書き方を体験しつつ、ゲーム制作ができるいい題材だったのではないかな、と感じています。

