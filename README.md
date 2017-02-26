# A CLI Tool to check CAMPHOR- schedule
CLIからCAMPHOR-の予定をシュッと確認するツール

# Todo

- エラー処理
- CIの導入

# Install && Usage
Set environment variables `CAMPH_SCHED_URL`, `CAMPH_SCHED_USER`, `CAMPH_SCHED_PASS`.

Clone this repo and

```
$ cargo install
```

Then,

```
$ sculd
```

# Development
Install Rust

```
$ curl https://sh.rustup.rs -sSf | sh
```

See [more](https://www.rust-lang.org/en-US/install.html).

Clone this repo and

```
$ cargo build
```

# おまけ: sculdの由来
> スクルド（古ノルド語: Skuld、またはSculd）は、北欧神話に登場する運命の女神、ノルンたち（ノルニル）の一柱で、三姉妹の三女。その名前は「税」「債務」「義務」または「未来」を意味する。

Wikipediaより引用

詳細は [スクルド](https://ja.wikipedia.org/wiki/%E3%82%B9%E3%82%AF%E3%83%AB%E3%83%89) をどうぞ。
