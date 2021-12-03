use std::str::FromStr;

pub fn from_file_data<T: FromStr>(filedata: &str) -> Result<Vec<T>, T::Err> {
    let mut retval = Vec::new();

    for line in filedata.lines() {
        retval.push(T::from_str(line)?);
    }

    Ok(retval)
}
