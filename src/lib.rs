#![warn(missing_docs)]
/*! Tools and types for parsing and analyzing NBM 1D viewer text files.

*/
/* ------------------------------------------------------------------------------------------------
 *                                         Public API
 * --------------------------------------------------------------------------------------------- */
pub use crate::error::Error;
pub use crate::nbm_data::{NBMColumnIter, NBMData};
/* ------------------------------------------------------------------------------------------------
 *                                        Private Modules
 * --------------------------------------------------------------------------------------------- */
mod error;
mod nbm_data;
mod nbm_raw;

#[cfg(test)]
pub(crate) mod test {

    pub fn load_test_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = vec![];

        for entry in std::fs::read_dir("test_data")? {
            let path = entry?.path();

            if path.extension().unwrap_or(std::ffi::OsStr::new("")) != "csv" {
                continue;
            }

            files.push(std::fs::read_to_string(path)?);
        }

        Ok(files)
    }
}
