use std::error::Error;

const USING_TEST_INPUT: bool = true;

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
    let input = {
        if USING_TEST_INPUT {
            todo!("No has afegit la input per testing");
        } else {
            gai::load_input(2, todo!())?
        }
    };

    println!("{:?}", input);
    Ok(())
}
