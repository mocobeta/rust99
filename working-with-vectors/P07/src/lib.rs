#[derive(Debug, PartialEq)]
pub struct List<T: Copy>(Vec<ListItem<T>>);

#[derive(Debug, PartialEq)]
pub enum ListItem<T: Copy> {
    Item(T),
    NestedItem(Box<List<T>>),
}

pub mod utils {
    use super::{List, ListItem};
    pub fn list<T: Copy>(items: Vec<ListItem<T>>) -> List<T> {
        List(items)
    }
    pub fn item<T: Copy>(v: T) -> ListItem<T> {
        ListItem::Item(v)
    }
    pub fn nested<T: Copy>(li: List<T>) -> ListItem<T> {
        ListItem::NestedItem(Box::new(li))
    }
}

pub fn flatten<T: Copy>(li: &List<T>) -> List<T> {
    utils::list(_flatten_list(li))
}

fn _flatten_list<T: Copy>(li: &List<T>) -> Vec<ListItem<T>> {
    let mut items = vec![];
    for item in li.0.iter() {
        items.extend(_flatten_item(item));
    }
    items
}

fn _flatten_item<T: Copy>(item: &ListItem<T>) -> Vec<ListItem<T>> {
    match item {
        ListItem::Item(v) => vec![ListItem::Item(*v)],
        ListItem::NestedItem(boxed_li) => _flatten_list(boxed_li.as_ref()),
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::*;

    #[test]
    fn test_flatten() {
        let li = list(vec![
            nested(list(vec![item(1), item(1)])),
            item(2),
            nested(list(vec![item(3), nested(list(vec![item(5), item(8)]))])),
        ]);
        let expected = list(vec![item(1), item(1), item(2), item(3), item(5), item(8)]);
        assert_eq!(flatten(&li), expected);
    }
}
