# Markdown List to Mermaid Converter
このツールは、Markdownのリスト形式（箇条書き）をMermaid形式のグラフに変換するものです。リスト内の項目をノードとして表示し、親子関係を矢印で表現します。コメントもサポートしており、リスト内の各項目に対して注釈を付けることができます。

## 使い方
### インストール
1. Rustがインストールされていない場合、Rust公式サイトからインストールしてください。
2. このリポジトリをクローンまたはダウンロードし、以下のコマンドでビルドします。
```bash
git clone https://github.com/yourusername/md-to-mermaid.git
cd md-to-mermaid
cargo build --release
```

### コマンドラインオプション
- -i, --input <FILE>
  入力ファイルを指定します。指定しない場合は標準入力から読み取ります。
- -o, --output <FILE>
  出力ファイルを指定します。指定しない場合は標準出力に結果を表示します。
- -d, --direction <DIRECTION>
  グラフの方向を指定します。TD（Top Down）またはLR（Left to Right）を選択できます。デフォルトはTDです。

### 使用例
```bash
$ cat example.md | ./target/debug/markdown-to-mermaid -d LR
graph LR
    node_0["Start"]
    node_1["Step 1"]
    node_2["Step 1.1"]
    node_3["Step 2"]
    node_4["End"]
    node_0 -->|"Proceed to Step 1"| node_1
    node_1 -->|"Detailed step"| node_2
    node_0 -->|"Another option"| node_3
    node_0 --> node_4
```