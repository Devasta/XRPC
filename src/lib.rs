

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

    fn tuple3<P1, P2, P3, R1, R2, R3>(&self, parser1: P1, parser2: P2, parser3: P3)
        -> impl Fn(usize) -> Result<(usize, (R1, R2, R3)), usize>
    where
        P1: Fn(usize) -> Result<(usize, R1), usize>,
        P2: Fn(usize) -> Result<(usize, R2), usize>,
        P3: Fn(usize) -> Result<(usize, R3), usize>
    {
        move |index| match parser1(index) {
            Ok((index1, result1))
            => match parser2(index1) {
                Ok((index2, result2))
                    => match parser3(index2){
                    Ok((index3, result3))=> Ok((index3, (result1, result2, result3))),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn many0<P, RV>(&self, parser: P)
        -> impl Fn(usize) ->Result<(usize, Vec<RV>), usize>
        where
            P: Fn(usize) -> Result<(usize, RV), usize>,
    {
        move |mut index| {
            let mut result = Vec::new();

            while let Ok((next_index, next_item)) = parser(index) {
                index = next_index;
                result.push(next_item);
            }

            Ok((index, result))
        }
    }

    fn map<P, F, A, B>(parser: P, map_fn: F)
        -> impl Fn(usize) -> Result<(usize, B), usize>
        where
            P: Fn(usize) -> Result<(usize, A), usize>,
            F: Fn(A) -> B,
    {
        move |index| match parser(index) {
            Ok((next_index, result)) => Ok((next_index, map_fn(result))),
            Err(err) => Err(err),
        }
    }

    fn take_until(){

    }

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

    #[test]
    fn parser_tuple3_test1() {
        let testdoc = Parser{
            document: "<doc>".to_string()
        };
        let parse_doc = testdoc.tuple3(
            testdoc.tag("<"),
            testdoc.tag("doc"),
            testdoc.tag(">")
        );
        assert_eq!(
            Ok((5,((),(),()))),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_many0_test1() {
        let testdoc = Parser{
            document: "<<<<>".to_string()
        };
        let parse_doc = testdoc.many0(
            testdoc.tag("<")
        );
        assert_eq!(
            Ok((4,vec![(),(),(),()])),
            parse_doc(0)
        );
    }

}

