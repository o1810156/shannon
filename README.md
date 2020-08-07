# シャノン・ファノ符号の学習

大学のある科目の自由課題として取り組んだものです。シャノン・ファノ符号を作成することに主眼を置いているので、シャノン・ファノ符号を用いて文章圧縮する機能、などは用意していません。

Rust実行環境は入っているものとし、本リポジトリに移動後次のコマンドで結果を見ることができます。

```bash
$ cargo run data.csv
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/shannon data.csv`
entropy: 2.0545851672678834
B => 0
C => 10
A => 110
D => 1110
E => 1111
avg_len: 2.083333328
```

カンマ区切りで

```csv
ラベル,頻度,
ラベル,頻度,
...
```

と書き込んだCSVファイルならばシャノン・ファノ符号を作成することができます。

# 使用したバージョン

```bash
$ cargo --version
cargo 1.40.0 (bc8e4c8be 2019-11-22)
$ rustc --version
rustc 1.40.0 (73528e339 2019-12-16)
```
