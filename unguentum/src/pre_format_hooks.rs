use parser::ast::Expression;

pub(crate) fn remove_trailing_whitespace_from_function_defs(expression: &mut Expression) {
    match expression {
        Expression::Symbol(_)
        | Expression::Literal(_)
        | Expression::Comment(_)
        | Expression::Continue(_)
        | Expression::Formula(_, _)
        | Expression::Newline(_)
        | Expression::Whitespace(_)
        | Expression::EOF(_)
        | Expression::Break(_) => {}
        Expression::Term(term) => {
            term.term
                .iter_mut()
                .for_each(remove_trailing_whitespace_from_function_defs);
        }
        Expression::Unary(_, expression) => {
            remove_trailing_whitespace_from_function_defs(expression)
        }
        Expression::Bop(_, expression1, expression2) => {
            remove_trailing_whitespace_from_function_defs(expression1);
            remove_trailing_whitespace_from_function_defs(expression2);
        }
        Expression::MultiBop(lhs, other) => {
            remove_trailing_whitespace_from_function_defs(lhs);
            other
                .iter_mut()
                .map(|(_, rhs)| rhs)
                .for_each(|arg0| remove_trailing_whitespace_from_function_defs(arg0));
        }
        Expression::FunctionDef(function_def) => {
            let body = &mut function_def.body;
            if let Expression::Term(ref mut terms) = **body {
                while terms
                    .term
                    .last()
                    .is_some_and(|last_expr| matches!(last_expr, Expression::Whitespace(_)))
                {
                    terms.term.pop();
                }
            }
        }
        Expression::LambdaFunction(lambda) => {
            let body = &mut lambda.body;
            if let Expression::Term(ref mut terms) = **body {
                while terms
                    .term
                    .last()
                    .is_some_and(|last_expr| matches!(last_expr, Expression::Whitespace(_)))
                {
                    terms.term.pop();
                }
            }
        }
        Expression::IfExpression(if_expr) => {
            remove_trailing_whitespace_from_function_defs(&mut if_expr.if_conditional.body);
            if_expr
                .trailing_else
                .as_mut()
                .iter_mut()
                .for_each(|trailing_else| {
                    remove_trailing_whitespace_from_function_defs(&mut trailing_else.body)
                });
            if_expr.else_ifs.iter_mut().for_each(|else_if| {
                remove_trailing_whitespace_from_function_defs(&mut else_if.if_conditional.body)
            });
        }
        Expression::WhileExpression(while_loop) => {
            remove_trailing_whitespace_from_function_defs(&mut while_loop.condition);
            remove_trailing_whitespace_from_function_defs(&mut while_loop.body);
        }
        Expression::RepeatExpression(repeat_loop) => {
            remove_trailing_whitespace_from_function_defs(&mut repeat_loop.body);
        }
        Expression::FunctionCall(call) => {
            call.args.args.iter_mut().for_each(|arg| {
                arg.0
                    .iter_mut()
                    .for_each(remove_trailing_whitespace_from_function_defs)
            });
        }
        Expression::SubsetExpression(subset) => subset.args.args.iter_mut().for_each(|arg| {
            arg.0
                .iter_mut()
                .for_each(remove_trailing_whitespace_from_function_defs)
        }),
        Expression::ForLoopExpression(for_loop) => {
            remove_trailing_whitespace_from_function_defs(&mut for_loop.collection);
            remove_trailing_whitespace_from_function_defs(&mut for_loop.body);
        }
    }
}
