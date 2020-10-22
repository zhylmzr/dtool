use wdf::Wdf;

mod text;
mod wdf;

fn main() {
    let mut wdf = Wdf::new("character.wdf");
    wdf.extra_all_with_hash("output", "known.lst").unwrap();
}
