use crate::ast::{AssignmentOp, Type, AST};

/// Formats an AST node into a readable string with appropriate indentation.
/// This function handles various types of AST nodes, applying formatting rules based on node type.
/// It also manages operator precedence to ensure correct placement of parentheses.
///
/// # Arguments
/// * `ast` - The AST node to format.
/// * `indent_level` - The level of indentation for the formatted output.
///
/// # Returns
/// A formatted string representation of the AST node.
pub fn format_ast(ast: &AST, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);

    // Determines the precedence level for an AST node to handle operator precedence.
    let precedence = |node: &AST| match node {
        AST::LogicalOr(_, _, _) => 10,
        AST::LogicalAnd(_, _, _) => 20,
        AST::Equal(_, _, _) | AST::NotEqual(_, _, _) => 30,
        AST::LessThan(_, _, _)
        | AST::LessThanOrEqual(_, _, _)
        | AST::GreaterThan(_, _, _)
        | AST::GreaterThanOrEqual(_, _, _) => 40,
        AST::Add(_, _, _) | AST::Sub(_, _, _) => 50,
        AST::Mul(_, _, _) | AST::Div(_, _, _) | AST::Mod(_, _, _) => 60,
        AST::PowArcana(_, _, _) | AST::PowAether(_, _, _) => 70,
        AST::LogicalNot(_, _) => 80,
        _ => 100,
    };

    let current_precedence = precedence(ast);

    // Formats a sub-expression, adding parentheses if necessary based on precedence.
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
                format!("{:.1}", value)
            } else {
                format!("{}", value)
            }
        }
        AST::Rune(value, _) => format!("\"{}\"", value),
        AST::Omen(value, _) => match value {
            true => "boon".to_string(),
            false => "hex".to_string(),
        },
        AST::Abyss(_) => "abyss".to_string(),
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
            let val = format_ast(value, indent_level);
            let trimmed_val = val.trim();
            match trimmed_val {
                "abyss" => "reveal".to_string(),
                _ => format!("reveal {}", trimmed_val),
            }
        }
        AST::Block(statements, _) => {
            let mut result = format!("{}{{\n", indent);
            for statement in statements {
                result.push_str(&format!("{}\n", format_ast(statement, indent_level + 1)));
            }
            result.push_str(&format!("{}}}", indent));
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
                result.push_str(&format!(" ({})", conditions));
            }
            result.push_str(" {\n");
            for branch in branches {
                if let AST::Comment(text, _) = branch {
                    result.push_str(&format!("{}{}\n", "    ".repeat(indent_level + 1), text));
                    continue;
                }

                if let AST::OracleBranch { pattern, body, .. } = branch {
                    let pattern = pattern
                        .iter()
                        .map(|pat| format_ast(pat, indent_level + 1))
                        .collect::<Vec<_>>()
                        .join(", ");
                    result.push_str(&format!(
                        "{}{} => {}\n",
                        "    ".repeat(indent_level + 1),
                        if pattern == "" {
                            "_".to_string()
                        } else {
                            format!("({})", pattern)
                        },
                        format_ast(&body, indent_level + 1).trim()
                    ));
                }
            }
            result.push_str(&format!("{}}}", indent));
            result
        }
        AST::OracleDontCareItem(_) => format!("_"),
        AST::Orbit { params, body, .. } => {
            let mut result = "orbit".to_string();
            if !params.is_empty() {
                let params_str = params
                    .iter()
                    .map(|param| format_ast(param, indent_level))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&format!(" ({})", params_str));
            }
            result.push_str(&format_ast(body, indent_level).trim());
            result
        }
        AST::OrbitParam {
            name,
            start,
            end,
            op,
            ..
        } => {
            let start_expr = format_ast(start, 0);
            let end_expr = format_ast(end, 0);
            format!("{} = {}{}{}", name, start_expr, op, end_expr)
        }
        AST::Resume(value, _) => match value {
            Some(idendifier) => format!("resume {}", idendifier),
            None => "resume".to_string(),
        },
        AST::Eject(value, _) => match value {
            Some(idendifier) => format!("eject {}", idendifier),
            None => "eject".to_string(),
        },
        AST::Engrave {
            name,
            params,
            return_type,
            body,
            ..
        } => {
            let return_type_str = match return_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                Type::Omen => "omen",
                Type::Abyss => "",
            };
            let params_str = params
                .iter()
                .map(|param| format_ast(param, indent_level))
                .collect::<Vec<_>>()
                .join(", ");
            match return_type_str {
                "" => format!(
                    "engrave {}({}) {}",
                    name,
                    params_str,
                    format_ast(body, indent_level)
                ),
                _ => format!(
                    "engrave {}({}) -> {} {}",
                    name,
                    params_str,
                    return_type_str,
                    format_ast(body, indent_level)
                ),
            }
        }
        AST::EngraveParam {
            name, param_type, ..
        } => {
            let type_str = match param_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                _ => "",
            };
            format!("{}: {}", name, type_str)
        }
        AST::FuncCall { name, args, .. } => {
            let args_str = args
                .iter()
                .map(|arg| format_ast(arg, indent_level))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", name, args_str)
        }
        AST::Summon(prompt, var_type, _) => {
            let type_str = match var_type {
                Type::Arcana => "arcana",
                Type::Aether => "aether",
                Type::Rune => "rune",
                _ => "",
            };
            format!("summon({}, {})", prompt, type_str)
        }
        AST::Comment(text, _) => text.clone(),
        _ => format!("Not implemented: {:?}", ast),
    }
}
