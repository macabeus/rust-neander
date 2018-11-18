pub struct UIState {
    pub current_line: usize,
    pub memory_list_first_line: usize,
    pub memory_list_last_line: usize,
    pub is_typing: bool,
    pub typing_char: Option<char>,
    pub quit: bool,
}

