use wasm_bindgen::prelude::*;
use rand::prelude::*;

#[wasm_bindgen]
pub fn keep_working() -> String {
    if random() {
        String::from("Keep working, keep learning.")
    }
    else {
        String::from("Let work go to hell! Happy hour begins!")
    }
}


/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
