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
$ cat example.md                                           
- Page A localhost:3000
  - /api/users // ユーザー一覧のデータ取得
    - localhost:3001/v1/users // 外部APIへの依存
    - localhost:3001/v1/other-data // 外部APIへの依存
  - /api/notice // お知らせデータ取得
    - localhost:3002/v1/notices // 外部APIへの依存

$ cat example.md | ./target/debug/markdown-to-mermaid -d LR
graph LR
    node_0["Page A localhost:3000"]
    node_1["/api/users"]
    node_2["localhost:3001/v1/users"]
    node_3["localhost:3001/v1/other-data"]
    node_4["/api/notice"]
    node_5["localhost:3002/v1/notices"]
    node_0 -->|"ユーザー一覧のデータ取得"| node_1
    node_1 -->|"外部APIへの依存"| node_2
    node_1 -->|"外部APIへの依存"| node_3
    node_0 -->|"お知らせデータ取得"| node_4
    node_4 -->|"外部APIへの依存"| node_5
```

![image](https://github.com/user-attachments/assets/423f2544-916c-4990-9472-825401145b89)
