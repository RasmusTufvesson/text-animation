pub struct Font<'a> {
    chars: Vec<&'a str>,
    char_height: u32,
}

impl<'a> Font<'a> {
    const ALL_CHARS: &str = " abcdefghijklmnopqrstuvwxyzåäö";

    pub fn new(chars: Vec<&'a str>, char_height: u32) -> Self {
        // let chars: Vec<&str> = font_string.split("\n--NEXT-LETTER--\n").collect();
        // let height = chars[0].split("\n").collect::<Vec<&str>>().len() as u32;
        Self { chars, char_height }
    }

    pub fn render(&self, to_render: &str) -> String {
        let mut rows: Vec<String> = vec![];
        let mut top_row_index = 0;
        let mut max_width: usize = 0;
        let mut last_char = ' ';
        for render_char in to_render.chars() {
            let render_str: &str = &render_char.to_string();
            if render_str == "\n" || (render_str == "n" && last_char == '\\') {
                top_row_index += self.char_height as usize;
            } else if Self::ALL_CHARS.contains(render_str) {
                let letter = self.chars[Self::ALL_CHARS.chars().position(|c| c == render_char).unwrap()];
                for (i, row) in letter.split('\n').enumerate() {
                    let row_index = top_row_index + i;
                    if rows.len() <= row_index {
                        rows.push(row.to_owned());
                        max_width = max_width.max(rows[row_index].len());
                    } else {
                        rows[row_index] += " ";
                        rows[row_index] += row;
                        max_width = max_width.max(rows[row_index].len());
                    }
                }
            }
            last_char = render_char;
        }
        let rows: Vec<String> = rows.iter().map(|row| row.to_owned() + &" ".repeat(max_width - row.len())).collect();
        rows.join("\n")
    }
}

pub fn frame(to_frame: &str) -> String {
    let mut rows: Vec<String> = to_frame.split('\n').map(|row| "│  ".to_owned()+row+"  │").collect();
    let top_bottom = "─".repeat(rows[0].len() - 6);
    rows.splice(0..0, ["┌".to_owned()+&top_bottom+"┐"]);
    rows.push("└".to_owned()+&top_bottom+"┘");
    rows.join("\n")
}