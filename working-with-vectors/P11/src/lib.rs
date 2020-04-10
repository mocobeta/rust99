use P09::pack;

#[derive(PartialEq, Debug)]
pub enum Item<T> {
    Element(T),
    Pair(usize, T),
}

pub mod utils {
    use super::Item;
    pub fn elem<T>(e: T) -> Item<T> {
        Item::Element(e)
    }
    pub fn pair<T>(n: usize, e: T) -> Item<T> {
        Item::Pair(n, e)
    }
}

pub fn encode_modified<T: PartialEq + Copy>(li: &Vec<T>) -> Vec<Item<T>> {
    use utils::*;
    pack(li)
        .into_iter()
        .map(|x| {
            if x.len() == 1 {
                elem(x[0])
            } else {
                pair(x.len(), x[0])
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;
    #[test]
    fn test_encode_modified() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            encode_modified(&li),
            vec![
                pair(4, 'a'),
                elem('b'),
                pair(2, 'c'),
                pair(2, 'a'),
                elem('d'),
                pair(4, 'e')
            ]
        );
    }
}
