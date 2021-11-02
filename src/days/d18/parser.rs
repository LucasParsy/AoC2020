pub mod parser_methods {
    use std::str::FromStr;

    use nom::{
        branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
        multi::many0, sequence::tuple, IResult,
    };

    use crate::days::d18::{Expression, Operator, Term};

    fn num_parse(input: &str) -> IResult<&str, Term> {
        map_res(digit1, Term::from_str)(input)
    }

    fn operator_parse(input: &str) -> IResult<&str, Operator> {
        map_res(nom::character::complete::anychar, Operator::from_char)(input)
    }

    fn parenthesis_parse(input: &str) -> IResult<&str, Term> {
        let (input, tup) = tuple((tag("("), expression_parse, tag(")")))(input)?;
        Ok((input, Term::Expr(tup.1)))
    }

    fn term_parse(input: &str) -> IResult<&str, Term> {
        let (input, term) = alt((parenthesis_parse, num_parse))(input)?;
        Ok((input, term))

        //let (input, tup) = num_parse(input)?;
    }

    fn optional_operation_parse(input: &str) -> IResult<&str, (Operator, Term)> {
        let (input, tup) = tuple((tag(" "), operator_parse, tag(" "), term_parse))(input)?;
        Ok((input, (tup.1, tup.3)))
    }

    pub fn expression_parse(input: &str) -> IResult<&str, Expression> {
        let (input, tup) = tuple((term_parse, many0(optional_operation_parse)))(input)?;
        let mut operators = vec![Operator::Add];
        let mut terms = vec![tup.0];
        for (op, term) in tup.1.into_iter() {
            terms.push(term);
            operators.push(op);
        }
        let expr = Expression { operators, terms };
        Ok((input, expr))
    }
}
