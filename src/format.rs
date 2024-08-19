use crate::ast::{Type, AST};

pub fn format_ast(ast: &AST, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level); // インデントレベルに応じたスペース

    match ast {
        AST::Omen(value, _) => match value {
            true => format!("{}boon", indent),
            false => format!("{}hex", indent),
        },
        AST::Arcana(value, _) => format!("{}{}", indent, value), // Arcana型のフォーマット例
        AST::Aether(value, _) => format!("{}{}", indent, value), // Aether型のフォーマット例
        AST::Rune(value, _) => format!("{}\"{}\"", indent, value), // Rune型のフォーマット例
        AST::Add(left, right, _) => format!(
            "{}{} + {}",
            indent,
            format_ast(left, 0),
            format_ast(right, 0)
        ),

        AST::Subtract(left, right, _) => format!(
            "{}{} - {}",
            indent,
            format_ast(left, 0),
            format_ast(right, 0)
        ),

        AST::Multiply(left, right, _) => format!(
            "{}{} * {}",
            indent,
            format_ast(left, 0),
            format_ast(right, 0)
        ),

        AST::Divide(left, right, _) => format!(
            "{}{} / {}",
            indent,
            format_ast(left, 0),
            format_ast(right, 0)
        ),
        AST::VarAssign {
            name,
            value,
            var_type,
            ..
        } => {
            let var_text = match var_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                Type::Omen => "omen",
            };
            format!(
                "{}forge {}: {} = {}",
                indent,
                name,
                var_text,
                format_ast(value, 0)
            )
        }
        AST::Unveil(args, _) => {
            let formatted_args: Vec<String> = args.iter().map(|arg| format_ast(arg, 0)).collect();
            format!("{}unveil({})", indent, formatted_args.join(", "))
        }
        AST::Oracle {
            conditionals,
            branches,
            ..
        } => {
            let mut result = format!("{}oracle {{\n", indent);
            for branch in branches {
                result.push_str(&format_ast(&branch.body, indent_level + 1));
                result.push('\n');
            }
            result.push_str(&format!("{}}};\n", indent));
            result
        }
        // 他のASTノード型についてもフォーマット処理を実装
        _ => String::new(),
    }
}
