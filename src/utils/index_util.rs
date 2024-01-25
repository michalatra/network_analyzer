use crate::utils::input_util::read_input;

pub fn select_index(prompt: &str, limit: usize) -> Option<usize> {
    let index: Option<usize> = read_input(prompt);

    match index {
        Some(index) => {
            if index < limit {
                Some(index)
            } else {
                None
            }
        },
        None => None
    }
}