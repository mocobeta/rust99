use P27::group;

pub fn main() {
    let li = vec![
        "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
    ];
    let groups = group(&[2, 2, 5], &li);
    groups.into_iter().for_each(|x| println!("{:?}", x));
}
