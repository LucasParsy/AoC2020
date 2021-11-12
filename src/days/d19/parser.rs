pub mod parser_methods {
    use std::str::FromStr;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, digit1, space1},
        combinator::{map_res, opt},
        multi::many1,
        sequence::tuple,
        IResult,
    };

    use crate::days::d19::{Expression, Rule};

    fn num_parse(input: &str) -> IResult<&str, u16> {
        map_res(digit1, u16::from_str)(input)
    }

    fn literal_parse(input: &str) -> IResult<&str, Expression> {
        let (st, tup) = tuple((tag(" \""), anychar, tag("\"")))(input)?;
        Ok((st, Expression::Chr(tup.1)))
    }

    fn space_num_parse(input: &str) -> IResult<&str, u16> {
        let (st, num) = tuple((space1, num_parse))(input)?;
        Ok((st, num.1))
    }
    
    fn multiple_num_parse(input: &str) -> IResult<&str, Vec<u16>> {
        let res = (many1(space_num_parse))(input)?;
        Ok((res.0, res.1))
    }

    fn optional_num_parse(input: &str) -> IResult<&str, Vec<u16>> {
        let (st, res) = tuple((tag(" |"), multiple_num_parse))(input)?;
        Ok((st, res.1))
    }

    fn rule_parse(input: &str) -> IResult<&str, Expression> {
        let res = tuple((
            multiple_num_parse,
            opt(optional_num_parse),
        ))(input)?;
        let rule = Expression::Rule(Rule { pattern: res.1.0, sec_patt: res.1.1 });
        Ok((input, rule))
    }

    pub fn expression_parse(input: &str) -> IResult<&str, (u16, Expression)> {
        let (input, tup) = tuple((num_parse, tag(":"), alt((literal_parse, rule_parse))))(input)?;
        Ok((input, (tup.0, tup.2)))
    }
}
