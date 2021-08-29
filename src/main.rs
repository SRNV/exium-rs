mod exium;
use crate::exium::Exium;
fn main() {
    let exium: Exium = Exium::new(|reason, cursor, context| {
        println!("error");
    });
    exium.read("
import component { Bio } from './b.bio';
    ");
}
