

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>
}

/*

alt
char
delimited
many0
many1
map
none_of
opt
tag -- DONE
take_while_m_n
tuple
value
verify

 */


struct Parser {
    document: String
}

impl Parser {

    fn tag<'a>(&self, expected: &'static str)
               -> impl Fn(usize) -> Result<(usize, ()), usize> + '_
    {
        move |index| match self.document.get(index..index+expected.len()) {
            Some(next) if next == expected => {
                Ok((index+expected.len(), ()))
            }
            _ => Err(index),
        }
    }

    fn pair<P1, P2, R1, R2>(&self, parser1: P1, parser2: P2)
            -> impl Fn(usize) -> Result<(usize, (R1, R2)), usize>
        where
            P1: Fn(usize) -> Result<(usize, R1), usize>,
            P2: Fn(usize) -> Result<(usize, R2), usize>,
    {
        move |index| match parser1(index) {
            Ok((next_index, result1))
            => match parser2(next_index) {
                Ok((final_index, result2)) => Ok((final_index, (result1, result2))),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn map<P, F, A, B>(parser: P, map_fn: F)
            -> impl Fn(&str) -> Result<(usize, B), usize>
        where
            P: Fn(&str) -> Result<(usize, A), usize>,
            F: Fn(A) -> B,
    {
        move |input|
            parser(input)
                .map(|(next_input, result)| (next_input, map_fn(result)))
    }

    /*
    impl<Input: Clone, A, B, Error: ParseError<Input>,FnA: Parser<Input, A, Error>, FnB: Parser<Input, B, Error>> Tuple<Input, (A, B), Error> for (FnA, FnB) {
    fn parse(&mut self, input: Input) -> IResult<Input, (A, B), Error> {
        {
            let (i, o) = self.0.parse(input.clone())?;

            {
                let (i, o) = self.1.parse(i.clone())?;

                Ok((i, (o, o)))
            }
        }
    }
}
     */

    /*
impl<
    Input: Clone, Output, Error: ParseError<Input>,
    A: Parser<Input, Output, Error>, B: Parser<Input, Output, Error>
> Alt<Input, Output, Error> for (A, B) {
    fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
        match self.0.parse(input.clone()) {
            Err(Err::Error(e)) => match self.1.parse(input.clone()) {
                Err(Err::Error(e)) => {
                    let err = e.or(e);
                    Err(Err::Error(Error::append(input, ErrorKind::Alt, err)))
                }
                res => res,
            },
            res => res,
        }
    }
}
     */



}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn parser_tag1() {
        let testdoc = Parser{
            document: "<doc>".to_string()
        };
        let parse_doc = testdoc.tag("<");
        assert_eq!(
            Ok((1, ())),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_tag2() {
        let testdoc = Parser{
            document: "<doc>".to_string()
        };
        let parse_doc = testdoc.tag(">");
        assert_eq!(
            Err(0),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_pair1() {
        let testdoc = Parser{
            document: "<doc>".to_string()
        };
        let parse_doc = testdoc.pair(
            testdoc.tag("<"),
            testdoc.tag("doc")
        );
        assert_eq!(
            Ok((4, ((),()))),
            parse_doc(0)
        );
    }
}

