use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::fold_many0, number::complete::recognize_float, sequence::delimited, IResult,
};
use parse_hyperlinks::take_until_unbalanced;

#[derive(Debug, Clone, PartialEq)]
enum ExprFragment {
    Value(f32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Precedence(String),
    WS,
}

fn parse_expr(input: &str) -> IResult<&str, ExprFragment> {
    let parser_float = |s| recognize_float(s);
    alt((
        map(tag("*"), |_| ExprFragment::Multiply),
        map(tag("/"), |_| ExprFragment::Divide),
        map(tag("+"), |_| ExprFragment::Plus),
        map(tag("-"), |_| ExprFragment::Minus),
        map(tag("("), |_| ExprFragment::LParen),
        map(tag(")"), |_| ExprFragment::RParen),
        map(tag(" "), |_| ExprFragment::WS),
        map(alpha1, |_| ExprFragment::WS),
        map(parser_float, |value: &str| {
            ExprFragment::Value(value.parse::<f32>().unwrap())
        }),
    ))(input)
}

fn parse_fragment(input: &str) -> IResult<&str, ExprFragment> {
    alt((
        map(
            delimited(tag("("), take_until_unbalanced('(', ')'), tag(")")),
            |expr: &str| {
                let mut treated_input = expr.to_string();
                let mut parens =
                    treated_input.matches("(").count() - treated_input.matches(")").count();
                while parens > 0 {
                    treated_input = format!("{})", treated_input);
                    parens -= 1;
                }
                let (_, fragment) = parse_fragment(&treated_input).unwrap();
                match fragment {
                    ExprFragment::Value(_) => {
                        let interpreted_expr = interpret(&treated_input);
                        ExprFragment::Value(interpreted_expr)
                    }
                    _ => ExprFragment::Precedence(expr.to_string()),
                }
            },
        ),
        map(parse_expr, |expr| expr),
    ))(input)
}

#[derive(Debug)]
enum NodeType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Precedence(String),
    Value(f32),
}

type Ast = Vec<NodeType>;

fn parser(input: &str) -> IResult<&str, Ast> {
    let mut build_expression = fold_many0(parse_fragment, Vec::new, |mut acc, fragment| {
        match fragment {
            ExprFragment::Precedence(expr) => {
                acc.push(NodeType::Precedence(expr));
            }
            ExprFragment::Value(value) => {
                acc.push(NodeType::Value(value));
            }
            ExprFragment::Multiply => {
                acc.push(NodeType::Multiply);
            }
            ExprFragment::Divide => {
                acc.push(NodeType::Divide);
            }
            ExprFragment::Plus => {
                acc.push(NodeType::Plus);
            }
            ExprFragment::Minus => {
                acc.push(NodeType::Minus);
            }
            ExprFragment::WS | ExprFragment::LParen | ExprFragment::RParen => {}
        };
        acc
    });
    build_expression(input)
}

fn calculate(out: &mut f32, value: f32, nodes: &Ast, i: usize) -> f32 {
    if i > 0 {
        let previous_node = &nodes[i - 1];
        match previous_node {
            NodeType::Multiply => {
                *out *= value;
            }
            NodeType::Divide => {
                *out /= value;
            }
            NodeType::Plus => {
                *out += value;
            }
            NodeType::Minus => {
                *out -= value;
            }
            _ => {}
        }
    } else {
        *out = value;
    }
    *out
}

fn render(result: IResult<&str, Ast>) -> f32 {
    let mut out = 0.0;
    match result {
        Ok((_, nodes)) => {
            for (i, _) in nodes.iter().enumerate() {
                let node = &nodes[i];

                match node {
                    NodeType::Precedence(expr) => {
                        let interpreted_expr = interpret(expr);
                        out = calculate(&mut out, interpreted_expr, &nodes, i);
                    }
                    NodeType::Value(value) => {
                        out = calculate(&mut out, *value, &nodes, i);
                    }
                    _ => {}
                }
            }
        }
        Err(_) => out = 0.0,
    };
    out
}

fn interpret(input: &str) -> f32 {
    let result = parser(input);
    render(result)
}

fn main() {
    let expr = &std::env::args().nth(1).unwrap();
    let comparison_label = &std::env::args().nth(2).unwrap_or("=".into());
    let result = interpret(expr);
    println!("{} {} {}", expr, comparison_label, result);
}
