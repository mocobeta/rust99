use P27::group3;

pub fn main() {
    let li = vec![
        "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
    ];
    let groups = group3(&li);
    groups.into_iter().for_each(|x| println!("{:?}", x));
}
