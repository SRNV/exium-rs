mod exium;
use crate::exium::Exium;
fn main() {
    let exium: Exium = Exium::new();
    exium.read("
import component { Bio } from './b.bio';
    ");
}
