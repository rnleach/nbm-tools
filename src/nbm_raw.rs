use optional::Optioned;

pub(crate) struct NBMRaw<'text> {
    text: &'text str,
}

impl<'text> NBMRaw<'text> {
    pub fn new(text: &'text str) -> Self {
        NBMRaw { text }
    }

    /// Count the numbe of non-header rows and columns in this file.
    ///
    /// This doesn't include the header row or the column with valid times.
    pub fn count_rows_and_cols(&self) -> (usize, usize) {
        let cols = self.text.lines().take(1).fold(0usize, |cnt, line| {
            // -1 to remove the valid time column
            cnt + line.split(',').count() - 1
        });

        // -1 to remove the header row
        let rows = self.text.lines().count() - 1;

        (rows, cols)
    }

    pub fn parse_full(
        &self,
        num_rows: usize,
        num_cols: usize,
    ) -> (Vec<String>, Vec<chrono::NaiveDateTime>, Vec<Optioned<f32>>) {
        let mut cols = Vec::with_capacity(num_cols);
        let mut rows = Vec::with_capacity(num_rows);
        let mut vals = Vec::with_capacity(num_cols * num_rows);

        // Parse the column headers
        self.text.lines().take(1).for_each(|txt| {
            // Skip 1st column because that is the valid time.
            txt.split(',').skip(1).for_each(|header| {
                cols.push(header.trim().to_owned());
            })
        });

        // Parse the rows and the values
        self.text.lines().skip(1).for_each(|line| {
            let mut col_iter = line.split(',');

            if let Some(vt) = col_iter.next() {
                if let Some(vt) = parse_date_time(vt) {
                    rows.push(vt);

                    col_iter.for_each(|val_str| {
                        let val_str = val_str.trim();
                        let val = match val_str.parse::<f32>() {
                            Ok(val) => optional::some(val),
                            Err(_) => optional::none(),
                        };

                        vals.push(val);
                    });
                } else {
                    #[cfg(test)]
                    panic!("Unable to parse {}", vt);
                }
            }
        });

        (cols, rows, vals)
    }
}

fn parse_date_time(text: &str) -> Option<chrono::NaiveDateTime> {
    let vt_str = text.trim();

    if vt_str.len() == 10 {
        let year = vt_str[0..4].parse::<i32>().ok()?;
        let month = vt_str[4..6].parse::<u32>().ok()?;
        let day = vt_str[6..8].parse::<u32>().ok()?;
        let hour = vt_str[8..].parse::<u32>().ok()?;

        Some(chrono::NaiveDate::from_ymd(year, month, day).and_hms(hour, 0, 0))
    } else {
        None
    }
}
