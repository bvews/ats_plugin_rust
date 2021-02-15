# BVE Trainsim ATS Plugin made with Rust

## 概要

BVE の ATS プラグインの Rust による試験的な実装です。これは初めての Rust プログラミングの結果であるため、実装や実装過程はそんなによろしくないかもしれません。

## ライセンス

[Unlicense](http://unlicense.org)

## ビルド方法

以下の説明は、お使いの PC の OS が Windows であることを前提としています。

### Rust のインストール

下のリンクから Rust をダウンロードしてインストールします。

[Rust をインストール - Rust プログラミング言語](https://www.rust-lang.org/ja/tools/install)

### ツールチェーンの追加

コマンドラインで以下のコマンドを入力してください。

```plaintext
rustup target add i686-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
```

### ビルド

32bit 版の DLL は、以下のコマンドでコンパイルできます。

```plaintext
cargo build --release --target="i686-pc-windows-msvc"
```

64bit 版の DLL は、以下のコマンドでコンパイルできます。

```plaintext
cargo build --release --target="x86_64-pc-windows-msvc"
```
