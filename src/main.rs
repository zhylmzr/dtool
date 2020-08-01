mod wdf;
mod text;
use wdf::Wdf;

fn main() {
    let mut wdf = Wdf::new("character.wdf");
    wdf.extra_all("output").unwrap();
}
