
type ParseResult<Output> = Result<(usize, Output), usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>
}

/*

alt -- DONE sort of
char -- Use TAG instead
delimited -- DONE
many0 -- DONE
many1 -- DONE
map -- DONE
none_of -- DONE
opt -- DONE
tag -- DONE
take_until -- DONE
take_while_m_n
tuple -- DONE sort of
value -- DONE
verify

 */


struct Parser {
    document: String
}

impl Parser {

    fn tag<'a>(&self, expected: &'static str)
       -> impl Fn(usize) -> ParseResult<()> + '_
    {
        move |index| match self.document.get(index..index+expected.len()) {
            Some(next) if next == expected => {
                Ok((index+expected.len(), ()))
            }
            _ => Err(index),
        }
    }

    fn pair<P1, P2, R1, R2>(&self, parser1: P1, parser2: P2)
        -> impl Fn(usize) -> ParseResult<(R1, R2)>
        where
            P1: Fn(usize) -> ParseResult<R1>,
            P2: Fn(usize) -> ParseResult<R2>
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
        -> impl Fn(usize) -> ParseResult<(R1, R2, R3)>
    where
        P1: Fn(usize) -> ParseResult<R1>,
        P2: Fn(usize) -> ParseResult<R2>,
        P3: Fn(usize) -> ParseResult<R3>,
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

    fn many0<P, R>(&self, parser: P)
        -> impl Fn(usize) -> ParseResult<Vec<R>>
        where
            P: Fn(usize) -> ParseResult<R>
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

    fn many1<P, R>(&self, parser: P)
       -> impl Fn(usize) -> ParseResult<Vec<R>>
        where
            P: Fn(usize) -> ParseResult<R>
    {
        move |mut index| {
            let mut result = Vec::new();

            match parser(index) {
                Err(err) => Err(err),
                Ok((next_index, next_item)) => {
                    index = next_index;
                    result.push(next_item);
                    while let Ok((next_index, next_item)) = parser(index) {
                        index = next_index;
                        result.push(next_item);
                    }
                    Ok((index, result))
                }
            }
        }
    }


    fn map<P, F, A, B>(&self, parser: P, map_fn: F)
        -> impl Fn(usize) -> ParseResult<B>
        where
            P: Fn(usize) -> ParseResult<A>,
            F: Fn(A) -> B,
    {
        move |index| match parser(index) {
            Ok((next_index, result)) => Ok((next_index, map_fn(result))),
            Err(err) => Err(err),
        }
    }


    fn alt2<P1, P2, A>(&self, parser1: P1, parser2: P2)
        -> impl Fn(usize) -> ParseResult<A>
        where
            P1: Fn(usize) -> ParseResult<A>,
            P2: Fn(usize) -> ParseResult<A>,
    {
        move |index| match parser1(index) {
            Ok((index1, result1)) => Ok((index1, result1)),
            Err(err) => match parser2(index){
                Ok((index2, result2)) => Ok((index2, result2)),
                Err(err) => Err(err)
            }
        }
    }

    fn opt<P1, R1>(&self, parser1: P1)
        -> impl Fn(usize) -> ParseResult<Option<R1>>
        where
            P1: Fn(usize) -> ParseResult<R1>
    {
        move |index| match parser1(index){
            Ok((index1, result1)) => Ok((index1, Some(result1))),
            Err(err) => Ok((index, None))
        }
    }

    fn delimited<P1, P2, P3, R1, R2, R3>(&self, parser1: P1, parser2: P2, parser3: P3)
        -> impl Fn(usize) -> ParseResult<R2>
        where
            P1: Fn(usize) -> ParseResult<R1>,
            P2: Fn(usize) -> ParseResult<R2>,
            P3: Fn(usize) -> ParseResult<R3>,
    {
        move |index| match parser1(index) {
            Ok((index1, result1))
            => match parser2(index1) {
                Ok((index2, result2))
                => match parser3(index2){
                    Ok((index3, result3))=> Ok((index3, (result2))),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn value<P1, R1, V: Clone>(&self, parser1: P1, val: V)
       -> impl Fn(usize) -> ParseResult<V>
        where
            P1: Fn(usize) -> ParseResult<R1>,
    {
        move |index| match parser1(index) {
            Ok((next_index, result)) => Ok((next_index, val.clone())),
            Err(err) => Err(err),
        }
    }

    pub fn none_of<'a>(&self, charlist: String)
        -> impl Fn(usize) -> ParseResult<String> + '_
    {
        move |index| match self.document.get(index..index+1) {
            Some(next) => {
                if charlist.contains(next){
                    Err(index)
                } else {
                    Ok((index+next.len(), next.to_owned()))
                }
            },
            _ => Err(index)
        }
    }

    pub fn take_until<'a>(&self, substr: String)
        -> impl Fn(usize) -> ParseResult<String> + '_
    {
        move |index| match self.document[index..].find(&substr){
            None => Err(index),
            Some(foundindex) => Ok((foundindex, self.document[index..foundindex].to_string()))
        }
    }

    pub fn take_while_m_n<'a, F>(&self, mn: usize, mx: usize, pred: F)
        -> impl Fn(usize) -> ParseResult<String> + 'a
        where
            F: Fn(&str) -> bool,
    {
        move |mut index| {
            let mut result = Vec::new();

            let mut resultindex = index;
            while pred(self.document.get(resultindex..resultindex + 1).unwrap()) {
                resultindex = resultindex + 1;
                result.push(self.document.get(resultindex..resultindex+1).unwrap());
            }

            if result.len() >= mn && result.len() <= mx {
                Ok((resultindex, result.into_iter().collect()))
            } else {
                Err(index)
            }
        }
    }

    /*
        fn tag<'a>(&self, expected: &'static str)
       -> impl Fn(usize) -> ParseResult<()> + '_
    {
        move |index| match self.document.get(index..index+expected.len()) {
            Some(next) if next == expected => {
                Ok((index+expected.len(), ()))
            }
            _ => Err(index),
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
    fn parser_tag_test1() {
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
    fn parser_tag_test2() {
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
    fn parser_tuple3_test2() {
        let testdoc = Parser{
            document: "<doc>".to_string()
        };
        let parse_doc = testdoc.tuple3(
            testdoc.tag("<"),
            testdoc.tag("doc"),
            testdoc.tag("<")
        );
        assert_eq!(
            Err(4),
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

    #[test]
    fn parser_many1_test1() {
        let testdoc = Parser{
            document: "<<<<>".to_string()
        };
        let parse_doc = testdoc.many1(
            testdoc.tag("<")
        );
        assert_eq!(
            Ok((4,vec![(),(),(),()])),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_many1_test2() {
        let testdoc = Parser{
            document: ">>>>".to_string()
        };
        let parse_doc = testdoc.many1(
            testdoc.tag("<")
        );
        assert_eq!(
            Err(0),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_alt2_test1() {
        let testdoc = Parser{
            document: "<<>".to_string()
        };
        let parse_doc = testdoc.alt2(
            testdoc.tag(">"),
            testdoc.tag("<<")
        );
        assert_eq!(
            Ok((2,())),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_opt_test1() {
        let testdoc = Parser{
            document: "AC".to_string()
        };
        let parse_doc = testdoc.tuple3(
            testdoc.tag("A"),
            testdoc.opt(testdoc.tag("B")),
            testdoc.tag("C")
        );
        assert_eq!(
            Ok((2,((),None, ()))),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_map_test1() {
        let testdoc = Parser{
            document: "AAAAB".to_string()
        };
        let parse_doc =
            testdoc.map(testdoc.many0(
            testdoc.tag("A")
                ),
                |result| {
                    result.len()
                }
            );
        assert_eq!(
            Ok((4,4)),
            parse_doc(0)
        );
    }


    #[test]
    fn parser_delimited_test1() {
        let testdoc = Parser{
            document: "AC".to_string()
        };
        let parse_doc = testdoc.delimited(
            testdoc.tag("A"),
            testdoc.opt(testdoc.tag("B")),
            testdoc.tag("C")
        );
        assert_eq!(
            Ok((2,(None))),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_value_test1() {
        let testdoc = Parser{
            document: "AAA".to_string()
        };
        let parse_doc = testdoc.value(testdoc.tag("A"), "B");
        assert_eq!(
            Ok((1, "B")),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_none_of_test1() {
        let testdoc = Parser{
            document: "AAAA".to_string()
        };
        let parse_doc = testdoc.none_of("BCD".to_string());
        assert_eq!(
            Ok((1, "A".to_string())),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_none_of_test2() {
        let testdoc = Parser{
            document: "AAAA".to_string()
        };
        let parse_doc = testdoc.none_of("ABCD".to_string());
        assert_eq!(
            Err(0),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_take_until_test1() {
        let testdoc = Parser{
            document: "ABCD".to_string()
        };
        let parse_doc = testdoc.take_until("C".to_string());
        assert_eq!(
            Ok((2, "AB".to_string())),
            parse_doc(0)
        );
    }

    #[test]
    fn parser_take_while_m_n_test1() {
        let testdoc = Parser{
            document: "ABCD1234".to_string()
        };
        let parse_doc = testdoc.take_while_m_n(1,6,|c: char| c.is_alphabetic());
        assert_eq!(
            Ok((3, "ABCD".to_string())),
            parse_doc(0)
        );
    }


}

