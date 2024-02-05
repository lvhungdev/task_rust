pub struct Table {
    col_size: usize,
    header: Vec<String>,
    content: Vec<Vec<String>>,
}

impl Table {
    pub fn new(col_size: usize) -> Self {
        return Self {
            col_size,
            header: Vec::new(),
            content: Vec::new(),
        };
    }

    pub fn with_header(mut self, header: Vec<String>) -> Self {
        assert_eq!(self.col_size, header.len());
        self.header = header;

        return self;
    }

    pub fn with_content(mut self, content: Vec<Vec<String>>) -> Self {
        for c in content {
            assert_eq!(self.col_size, c.len());
            self.content.push(c);
        }

        return self;
    }

    pub fn display(&self) {
        let max_len: Vec<usize> = self.find_max_len_each_col();

        let header_with_space: Vec<String> = self
            .header
            .iter()
            .enumerate()
            .map(|(i, m)| Self::get_text_with_space(m, max_len[i]))
            .collect();
        println!("{}", header_with_space.join("  "));

        let divider_with_space: Vec<String> = self
            .header
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let divider: String = (0..m.len()).map(|_| "-").collect();
                return Self::get_text_with_space(&divider, max_len[i]);
            })
            .collect();
        println!("{}", divider_with_space.join("  "));

        for row in &self.content {
            let row_with_space: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, m)| Self::get_text_with_space(m, max_len[i]))
                .collect();

            println!("{}", row_with_space.join("  "));
        }
    }

    fn find_max_len_each_col(&self) -> Vec<usize> {
        return (0..self.col_size)
            .map(|i| {
                let cols: Vec<String> = self.content.iter().map(|c| c[i].to_string()).collect();

                let max_content_len = match cols.iter().max_by(|x, y| x.len().cmp(&y.len())) {
                    Some(c) => c.len(),
                    None => 0,
                };

                return if self.header[i].len() > max_content_len {
                    self.header[i].len()
                } else {
                    max_content_len
                };
            })
            .collect();
    }

    fn get_text_with_space(text: &str, max_len: usize) -> String {
        if text.len() >= max_len {
            return text.to_string();
        }

        let space_to_fill: usize = max_len - text.len();
        let space: String = (0..space_to_fill).map(|_| " ").collect();

        return format!("{}{}", text, space);
    }
}
