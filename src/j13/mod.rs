use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Value(u8),
    List(Vec<Element>),
}

peg::parser! {
            grammar element_parser() for str {
                rule number() -> Element
                    = n:$(['0'..='9']+) { Element::Value(n.parse().unwrap()) }

                pub rule list() -> Element
                    = "[" l:((number() / list()) ** ",") "]" { Element::List(l) }
            }
        }


impl Element {
    pub fn parse_str(s: &str) -> Element {
        element_parser::list(s).unwrap()
    }
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::List(list1), Element::List(list2)) if
            list1.is_empty() && list2.is_empty() => {
                Ordering::Equal
            }
            (Element::List(list1), _) if
            list1.is_empty() => {
                Ordering::Less
            }
            (_, Element::List(list2)) if
            list2.is_empty() => {
                Ordering::Greater
            }
            (Element::Value(v1), Element::Value(v2)) => {
                v1.cmp(v2)
            }
            (Element::Value(v1), Element::List(_)) => {
                Element::List(vec![Element::Value(*v1)]).cmp(other)
            }
            (Element::List(_), Element::Value(v2)) => {
                self.cmp(&Element::List(vec![Element::Value(*v2)]))
            }
            (Element::List(list1), Element::List(list2)) => {
                if let Some(ord) = list1.iter().zip(list2)
                    .map(|(elt1, elt2)| elt1.cmp(elt2))
                    .find(|ord| *ord != Ordering::Equal) {
                    ord
                } else {
                    list1.len().cmp(&list2.len())
                }
            }
        }
    }
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();
    let mut total = 0;
    let mut pair_index = 0;

    loop {
        pair_index += 1;
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();

        let first = Element::parse_str(first);
        let second = Element::parse_str(second);

        match first.cmp(&second) {
            Ordering::Less => { total += pair_index }
            Ordering::Equal => { panic!() }
            Ordering::Greater => {}
        }

        match lines.next() {
            None => { break; }
            Some(_) => {}
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j13.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    let mut packets = vec!();

    loop {
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();

        let first = Element::parse_str(first);
        let second = Element::parse_str(second);

        packets.push(first);
        packets.push(second);

        match lines.next() {
            None => { break; }
            Some(_) => {}
        }
    }

    packets.sort();

    let first_packet = Element::List(vec!(Element::List(vec!(Element::Value(2)))));
    let second_packet = Element::List(vec!(Element::List(vec!(Element::Value(6)))));

    let pos1 = packets.iter().position(|packet| first_packet.cmp(packet) == Ordering::Less).unwrap();
    let pos2 = packets.iter().position(|packet| second_packet.cmp(packet) == Ordering::Less).unwrap();

    (pos1 + 1) * (pos2 + 2)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j13.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j13_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(13, _p1(include_str!("j13_test.txt")));
        assert_eq!(6369, _p1(include_str!("j13.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(140, _p2(include_str!("j13_test.txt")));
        assert_eq!(25800, _p2(include_str!("j13.txt")));
    }
}