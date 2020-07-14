mod wdf;
use wdf::Wdf;

fn main() {
    let mut wdf = Wdf::new("character.wdf");
    wdf.extra_all("output/character").unwrap();
}
