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
    node_0["Page A localhost:3000"]
    node_1["/api/users"]
    node_2["localhost:3001/v1/users"]
    node_3["/api/notice"]
    node_4["localhost:3002/v1/notices"]
    node_0 -->|"ユーザー一覧のデータ取得"| node_1
    node_1 -->|"外部APIへの依存"| node_2
    node_0 -->|"お知らせデータ取得"| node_3
    node_3 -->|"外部APIへの依存"| node_4
```

![image](https://github.com/user-attachments/assets/5b1e561c-5f9b-434c-9099-3b9ed51a5348)
