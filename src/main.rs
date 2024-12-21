use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Read};
use log::{debug};

#[derive(Parser, Debug)]
#[command(name = "md_to_mermaid")]
#[command(about = "Convert Markdown to Mermaid format", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, default_value = "TD")]
    direction: String,
}

fn parse_markdown_to_mermaid(markdown: &str, direction: &str) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut last_nodes: Vec<(String, usize)> = Vec::new();

    let re = Regex::new(r"^(\s*)[-*+] (.+?)(?:\s*//\s*(.+))?$").unwrap();

    for (i, line) in markdown.lines().enumerate() {
        debug!("Processing line {}: {}", i + 1, line);
        if let Some(captures) = re.captures(line) {
            let indent = captures[1].len();
            let content = captures[2].trim();
            let label = captures.get(3).map(|m| m.as_str().trim());
            let node_id = format!("node_{}", i);
            nodes.push((node_id.clone(), content.to_string()));

            if !last_nodes.is_empty() {
                // last_nodesが空でない場合、親ノードを探す処理
                let parent = last_nodes.iter().rev().find(|(_, level)| *level < indent);
                // last_nodesの中でインデントが現在のインデントより浅い（親ノード）ものを探す
                if let Some((parent_id, _)) = parent {
                    // 親ノードが見つかった場合、その親ノードIDと現在のノードIDを辺として追加
                    edges.push((parent_id.clone(), node_id.clone(), label.map(|c| c.to_string())));
                }
            }

            // 現在のノードIDとインデントをlast_nodesに追加（次回の親ノードとして使用）
            last_nodes.push((node_id.clone(), indent));
            // last_nodesをインデントが現在のインデント以下のノードに絞り込む
            last_nodes.retain(|(_, level)| *level <= indent);
        }
    }

    let mut mermaid = format!("graph {direction}\n", direction = direction);
    for (node_id, content) in nodes {
        mermaid.push_str(&format!("    {node_id}[\"{content}\"]\n"));
    }
    for (from, to, label) in edges {
        if let Some(label_text) = label {
            mermaid.push_str(&format!("    {from} -->|\"{label_text}\"| {to}\n"));
        } else {
            mermaid.push_str(&format!("    {from} --> {to}\n"));
        }
    }

    mermaid
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("Parsed arguments: {:?}", args);
    let markdown = if let Some(input_path) = args.input {
        debug!("Reading input file from: {}", input_path);
        fs::read_to_string(&input_path).expect("Failed to read input file")
    } else {
        debug!("No input file specified, reading from standard input");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from standard input");
        buffer
    };

    debug!("Input markdown txt: \n{}", markdown);

    let mermaid = parse_markdown_to_mermaid(&markdown, &args.direction);

    if let Some(output_path) = args.output {
        fs::write(&output_path, mermaid).expect("Failed to write output file");
    } else {
        println!("{}", mermaid);
    }
}
