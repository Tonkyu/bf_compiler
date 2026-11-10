#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add(u8),
    Move(isize),
    Output,
    Input,
    Loop(Vec<Op>),
}

/// Brainfuck のソースを `Op` の木に変換する。
/// 括弧の対応が取れていなければエラーを返す。
pub fn parse(code: &str) -> Result<Vec<Op>, String> {
    let bytes = code.as_bytes();
    let mut i = 0;
    let ops = parse_block(bytes, &mut i, false)?;
    Ok(ops)
}

fn parse_block(bytes: &[u8], i: &mut usize, in_loop: bool) -> Result<Vec<Op>, String> {
    let mut ops = Vec::new();

    while *i < bytes.len() {
        match bytes[*i] {
            b'+' | b'-' => {
                let mut count = 0u8;
                while *i < bytes.len() && matches!(bytes[*i], b'+' | b'-') {
                    count = if bytes[*i] == b'+' {
                        count.wrapping_add(1)
                    } else {
                        count.wrapping_sub(1)
                    };
                    *i += 1;
                }
                ops.push(Op::Add(count));
                continue;
            }
            b'>' | b'<' => {
                let mut offset = 0isize;
                while *i < bytes.len() && matches!(bytes[*i], b'>' | b'<') {
                    offset += if bytes[*i] == b'>' { 1 } else { -1 };
                    *i += 1;
                }
                ops.push(Op::Move(offset));
                continue;
            }
            b'.' => ops.push(Op::Output),
            b',' => ops.push(Op::Input),
            b'[' => {
                let open = *i;
                *i += 1;
                let body = parse_block(bytes, i, true)
                    .map_err(|e| if e.is_empty() { format!("unmatched '[' at byte {open}") } else { e })?;
                ops.push(Op::Loop(body));
                continue; // parse_block は ']' の次まで進めている
            }
            b']' => {
                if in_loop {
                    *i += 1;
                    return Ok(ops);
                }
                return Err(format!("unmatched ']' at byte {}", *i));
            }
            _ => {}
        }
        *i += 1;
    }

    if in_loop {
        // 呼び出し元で位置付きのメッセージに置き換える
        Err(String::new())
    } else {
        Ok(ops)
    }
}

/// `Op` の列をループごとにインデントした読みやすい形に整形する。
/// `Add` は符号付きで表示する（`Add(255)` は `Add(-1)`）。
pub fn pretty(ops: &[Op]) -> String {
    let mut out = String::new();
    write_ops(ops, 0, &mut out);
    out
}

fn write_ops(ops: &[Op], depth: usize, out: &mut String) {
    let indent = "    ".repeat(depth);
    for op in ops {
        match op {
            Op::Add(n) => out.push_str(&format!("{indent}Add({:+})\n", *n as i8)),
            Op::Move(n) => out.push_str(&format!("{indent}Move({n:+})\n")),
            Op::Output => out.push_str(&format!("{indent}Output\n")),
            Op::Input => out.push_str(&format!("{indent}Input\n")),
            Op::Loop(body) => {
                out.push_str(&format!("{indent}Loop {{\n"));
                write_ops(body, depth + 1, out);
                out.push_str(&format!("{indent}}}\n"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_loops_and_folds_runs() {
        let ops = parse("++-[>>[-]<]日本語コメント").unwrap();
        assert_eq!(
            ops,
            vec![
                Op::Add(1),
                Op::Loop(vec![
                    Op::Move(2),
                    Op::Loop(vec![Op::Add(255)]),
                    Op::Move(-1),
                ]),
            ]
        );
    }

    #[test]
    fn pretty_prints_with_indentation() {
        let ops = parse("+>[-<.]").unwrap();
        assert_eq!(
            pretty(&ops),
            "Add(+1)\nMove(+1)\nLoop {\n    Add(-1)\n    Move(-1)\n    Output\n}\n"
        );
    }

    #[test]
    fn rejects_unbalanced_brackets() {
        assert!(parse("[[]").is_err());
        assert!(parse("[]]").is_err());
    }
}
