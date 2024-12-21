use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "md_to_mermaid")]
#[command(about = "Convert Markdown to Mermaid format with comments", long_about = None)]
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
        if let Some(captures) = re.captures(line) {
            let indent = captures[1].len();
            let content = captures[2].trim();
            let comment = captures.get(3).map(|m| m.as_str().trim());
            let node_id = format!("node_{}", i);

            nodes.push((node_id.clone(), content.to_string()));

            if !last_nodes.is_empty() {
                let parent = last_nodes.iter().rev().find(|(_, level)| *level < indent);
                if let Some((parent_id, _)) = parent {
                    edges.push((parent_id.clone(), node_id.clone(), comment.map(|c| c.to_string())));
                }
            }

            last_nodes.push((node_id.clone(), indent));
            last_nodes.retain(|(_, level)| *level <= indent);
        }
    }

    let mut mermaid = format!("graph {direction}\n", direction = direction);
    for (node_id, content) in nodes {
        mermaid.push_str(&format!("    {node_id}[\"{content}\"]\n"));
    }
    for (from, to, comment) in edges {
        if let Some(comment_text) = comment {
            mermaid.push_str(&format!("    {from} -->|\"{comment_text}\"| {to}\n"));
        } else {
            mermaid.push_str(&format!("    {from} --> {to}\n"));
        }
    }

    mermaid
}

fn main() {
    let args = Args::parse();
    let markdown = if let Some(input_path) = args.input {
        fs::read_to_string(&input_path).expect("Failed to read input file")
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from standard input");
        buffer
    };

    let mermaid = parse_markdown_to_mermaid(&markdown, &args.direction);

    if let Some(output_path) = args.output {
        fs::write(&output_path, mermaid).expect("Failed to write output file");
    } else {
        println!("{}", mermaid);
    }
}
