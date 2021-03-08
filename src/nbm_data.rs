use optional::Optioned;

/// A parsed NBM 1D viewer CSV file.
pub struct NBMData {
    init_time: chrono::NaiveDateTime,
    rows: Vec<chrono::NaiveDateTime>,
    cols: Vec<String>,
    num_rows: usize,
    num_cols: usize,
    vals: Vec<Optioned<f32>>,
}

impl NBMData {
    /// Get the initialization time for this file.
    pub fn initialization_time(&self) -> chrono::NaiveDateTime {
        self.init_time
    }

    /// Get an iterator over rows for a given column.
    ///
    /// If no such column exists, it will return an `nbm_tools::Error::NoSuchColumn` error value.
    pub fn column_iter<'a, 'b>(&'a self, col_name: &'b str) -> Result<NBMColumnIter, crate::Error> {
        let col_idx = self
            .cols
            .iter()
            .position(|x| x == col_name)
            .ok_or(crate::Error::NoSuchColumn)?;

        Ok(NBMColumnIter {
            col: col_idx,
            curr_row: 0,
            src: self,
        })
    }

    pub(crate) fn create(text: &str) -> Result<Self, crate::Error> {
        let raw = crate::nbm_raw::NBMRaw::new(text);
        let (num_rows, num_cols) = raw.count_rows_and_cols();
        let (cols, rows, vals) = raw.parse_full(num_rows, num_cols);

        assert_eq!(cols.len(), num_cols);
        assert_eq!(rows.len(), num_rows);
        assert_eq!(vals.len(), num_rows * num_cols);

        Ok(NBMData {
            init_time: rows[0],
            rows,
            cols,
            num_rows,
            num_cols,
            vals,
        })
    }

    fn get_vals(&self, row: usize, col: usize) -> Option<(chrono::NaiveDateTime, f32)> {
        debug_assert!(self.num_cols > col);
        debug_assert!(self.num_rows > row);

        let valid_time = unsafe { *self.rows.get_unchecked(row) };
        let val = unsafe { *self.vals.get_unchecked(row * self.num_cols + col) };

        if val.is_some() {
            Some((valid_time, val.unpack()))
        } else {
            None
        }
    }
}

impl std::str::FromStr for NBMData {
    type Err = crate::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::create(text)
    }
}

/// An iterator over the values of a column and the valid time of that value. 
///
/// This struct is returned from the `NBMData::column_iter()` method. 
pub struct NBMColumnIter<'a> {
    col: usize,
    curr_row: usize,
    src: &'a NBMData,
}

impl<'a> Iterator for NBMColumnIter<'a> {
    type Item = (chrono::NaiveDateTime, f32);

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_row < self.src.num_rows {
            let vals = self.src.get_vals(self.curr_row, self.col);

            self.curr_row += 1;

            if vals.is_some() {
                return vals;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
        for text in crate::test::load_test_files()? {
            let nbm = NBMData::from_str(&text)?;
            println!("{}", nbm.initialization_time());
        }

        Ok(())
    }

    #[test]
    fn test_iterator() -> Result<(), Box<dyn std::error::Error>> {
        for text in crate::test::load_test_files()? {
            let nbm = NBMData::from_str(&text)?;

            for (vt, val) in nbm.column_iter("APCP24hr_surface_90% level")? {
                println!("{}  {}mm", vt, val);
            }
        }

        Ok(())
    }
}
