use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, tuple, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Service {
        name: String,
        namespace: Option<String>,
    },
    Rule {
        service_name: String,
        rule_name: String,
        rule_type: String,
        params: Option<String>,
    },
    // Add more enum variants for other DSL keywords (e.g., Trait, Deploy, etc.)
}

pub fn parse_dsl(input: &str) -> IResult<&str, Vec<Statement>> {
    separated_list0(multispace1, parse_statement)(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_service, parse_rule))(input) // Add other DSL keyword parsers here
}

fn parse_service(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("SERVICE:")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, name) = map(alpha1, |s: &str| s.to_string())(input)?;
    let (input, _) = multispace0(input)?;

    let (input, namespace) = opt(preceded(
        tag("NAMESPACE:"),
        preceded(multispace1, map(alpha1, |s: &str| s.to_string())),
    ))(input)?;

    Ok((input, Statement::Service { name, namespace }))
}

fn parse_rule(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("RULE:")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, service_name) = map(alpha1, |s: &str| s.to_string())(input)?;
    let (input, _) = multispace1(input)?;

    let (input, rule_name) = map(alpha1, |s: &str| s.to_string())(input)?;
    let (input, _) = multispace1(input)?;

    let (input, rule_type) = map(alpha1, |s: &str| s.to_string())(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = opt(preceded(
        tag("PARAMS"),
        preceded(multispace1, take_while(|c: char| !c.is_whitespace())),
    ))(input)?;

    Ok((
        input,
        Statement::Rule {
            service_name,
            rule_name,
            rule_type,
            params: params.map(|s: &str| s.to_string()),
        },
    ))
}

// Implement other parsing functions for different DSL keywords (e.g., Trait, Deploy, etc.)
