use P06::is_palindrome;

pub fn main() {
    let f = |li| {
        println!(
            "{:?} is {} a palindrome.",
            li,
            if is_palindrome(&li) { "" } else { "not" }
        );
    };
    let li = vec![1, 2, 3, 2, 1];
    f(li);

    let li = vec![1, 3, 3, 2, 1];
    f(li);
}
