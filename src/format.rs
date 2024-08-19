use crate::ast::{AssignmentOp, Type, AST};

pub fn format_ast(ast: &AST, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level); // インデントを生成

    // 優先順位テーブル
    let precedence = |node: &AST| match node {
        AST::LogicalOr(_, _, _) => 10,  // 最も低い優先順位
        AST::LogicalAnd(_, _, _) => 20, // 次に低い
        AST::Equal(_, _, _) | AST::NotEqual(_, _, _) => 30,
        AST::LessThan(_, _, _)
        | AST::LessThanOrEqual(_, _, _)
        | AST::GreaterThan(_, _, _)
        | AST::GreaterThanOrEqual(_, _, _) => 40,
        AST::Add(_, _, _) | AST::Sub(_, _, _) => 50,
        AST::Mul(_, _, _) | AST::Div(_, _, _) | AST::Mod(_, _, _) => 60,
        AST::PowArcana(_, _, _) | AST::PowAether(_, _, _) => 70, // 累乗演算の優先順位が最も高い
        AST::LogicalNot(_, _) => 80, // 単項演算子（論理否定）も高い優先順位
        _ => 100,                    // その他のノード
    };

    let current_precedence = precedence(ast);

    let format_with_parentheses = |expr: &AST, parent_precedence: u8| -> String {
        let sub_precedence = precedence(expr);
        let code = format_ast(expr, indent_level);

        if sub_precedence < parent_precedence {
            format!("({})", code)
        } else {
            code
        }
    };

    match ast {
        AST::Statement(statement, _) => {
            format!("{}{};", indent, format_ast(statement, indent_level))
        }
        AST::Add(left, right, _)
        | AST::Sub(left, right, _)
        | AST::Mul(left, right, _)
        | AST::Div(left, right, _)
        | AST::Mod(left, right, _)
        | AST::PowArcana(left, right, _)
        | AST::PowAether(left, right, _)
        | AST::LogicalAnd(left, right, _)
        | AST::LogicalOr(left, right, _)
        | AST::Equal(left, right, _)
        | AST::NotEqual(left, right, _)
        | AST::LessThan(left, right, _)
        | AST::LessThanOrEqual(left, right, _)
        | AST::GreaterThan(left, right, _)
        | AST::GreaterThanOrEqual(left, right, _) => {
            let operator = match ast {
                AST::Add(_, _, _) => "+",
                AST::Sub(_, _, _) => "-",
                AST::Mul(_, _, _) => "*",
                AST::Div(_, _, _) => "/",
                AST::Mod(_, _, _) => "%",
                AST::PowArcana(_, _, _) => "^",
                AST::PowAether(_, _, _) => "**",
                AST::LogicalAnd(_, _, _) => "&&",
                AST::LogicalOr(_, _, _) => "||",
                AST::Equal(_, _, _) => "==",
                AST::NotEqual(_, _, _) => "!=",
                AST::LessThan(_, _, _) => "<",
                AST::LessThanOrEqual(_, _, _) => "<=",
                AST::GreaterThan(_, _, _) => ">",
                AST::GreaterThanOrEqual(_, _, _) => ">=",
                _ => unreachable!(),
            };
            format!(
                "{} {} {}",
                format_with_parentheses(left, current_precedence),
                operator,
                format_with_parentheses(right, current_precedence)
            )
        }
        AST::LogicalNot(expr, _) => {
            format!("!{}", format_with_parentheses(expr, current_precedence))
        }
        AST::VarAssign {
            name,
            value,
            var_type,
            is_morph,
            ..
        } => {
            let type_str = match var_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                _ => "",
            };
            format!(
                "forge {}{}: {} = {}",
                if *is_morph { "morph " } else { "" },
                name,
                type_str,
                format_ast(value, indent_level)
            )
        }
        AST::Assignment {
            name, value, op, ..
        } => match op {
            AssignmentOp::Assign => format!("{} = {}", name, format_ast(value, indent_level)),
            AssignmentOp::AddAssign => {
                format!("{} += {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::SubAssign => {
                format!("{} -= {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::MulAssign => {
                format!("{} *= {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::DivAssign => {
                format!("{} /= {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::ModAssign => {
                format!("{} %= {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::PowArcanaAssign => {
                format!("{} ^= {}", name, format_ast(value, indent_level))
            }
            AssignmentOp::PowAetherAssign => {
                format!("{} **= {}", name, format_ast(value, indent_level))
            }
        },
        AST::Var(name, _) => name.clone(),
        AST::Arcana(value, _) => format!("{}", value),
        AST::Aether(value, _) => {
            if value.fract() == 0.0 {
                format!("{:.1}", value) // 小数点以下が0の場合は1桁で表示
            } else {
                format!("{}", value) // それ以外の場合はそのまま表示
            }
        }
        AST::Rune(value, _) => format!("\"{}\"", value),
        AST::Omen(value, _) => match value {
            true => "boon".to_string(),
            false => "hex".to_string(),
        },
        AST::Unveil(args, _) => format!(
            "unveil({})",
            args.iter()
                .map(|arg| format_ast(arg, indent_level))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        AST::Trans(value, var_type, _) => {
            let type_str = match var_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                _ => "",
            };
            format!("trans({} as {})", format_ast(value, indent_level), type_str)
        }
        AST::Reveal(value, _) => {
            format!("reveal {}", format_ast(value, indent_level))
        }
        AST::Block(statements, _) => {
            let mut result = format!("{}{{\n", indent);
            for statement in statements {
                result.push_str(&format!("{}\n", format_ast(statement, indent_level + 1)));
            }
            result.push_str(&format!("{}}};", indent));
            result
        }
        AST::Oracle {
            is_match,
            conditionals,
            branches,
            ..
        } => {
            let mut result = "oracle".to_string();
            if !conditionals.is_empty() {
                let conditions = conditionals
                    .iter()
                    .map(|cond| {
                        if *is_match {
                            format!("{}", format_ast(&cond.expression, indent_level))
                        } else {
                            format!(
                                "{} = {}",
                                cond.variable,
                                format_ast(&cond.expression, indent_level)
                            )
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&format!("({})", conditions));
            }
            result.push_str(" {\n");
            for branch in branches {
                let pattern = branch
                    .pattern
                    .iter()
                    .map(|pat| format_ast(pat, indent_level + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&format!(
                    "{}{} => {}\n",
                    "  ".repeat(indent_level + 1),
                    if pattern == "" {
                        "_".to_string()
                    } else {
                        format!("({})", pattern)
                    },
                    format_ast(&branch.body, indent_level + 1).trim()
                ));
            }
            result.push_str(&format!("{}}}", indent));
            result
        }
        AST::OracleDontCareItem(_) => format!("_"),
        AST::Comment(text, _) => text.clone(),
    }
}
