# Rustに入門したい
ゲームを作りながらRustを勉強してみたいんです。

海外の方がYoutubeでRust向けゲームエンジンである`piston`を用いて、スネークゲームを作ってる動画を見つけたので、
それを翻訳しながら、自分が勉強したことをまとめていこうと思います。
[Making a Snake Game in Rust](https://www.youtube.com/watch?v=HCwMb0KslX8)

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
authors = ["Hiroya_W <e8239@g.maizuru-ct.ac.jp>"]
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

