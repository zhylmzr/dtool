use wdf::Wdf;

mod text;
mod wdf;

fn main() {
    let arr = vec![
        "character.wdf",
        "fx.wdf",
        "helper.wdf",
        "interface.wdf",
        "map.wdf",
        "object.wdf",
        "setting.wdf",
        "tile.wdf",
    ];

    for pkg in arr {
        let mut wdf = Wdf::new(&format!("{}", pkg));
        wdf.extra_all_with_hash("output", "known.lst").unwrap();
    }
}
