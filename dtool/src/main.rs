use wdf::Wdf;

mod wdf;
mod text;

fn main() {
    let mut wdf = Wdf::new("character.wdf");
    wdf.extra_all("output").unwrap();
}
