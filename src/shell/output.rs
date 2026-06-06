#[derive(PartialEq, Debug)]
pub enum Output {
    Ok(Vec<u8>),
    Err(Vec<u8>, Option<i32>),
}

#[cfg(test)]
impl Output {
    pub fn ok_str(str: &str) -> Self {
        Output::Ok(str.as_bytes().to_vec())
    }

    pub fn err_str(str: &str) -> Self {
        Output::Err(str.as_bytes().to_vec(), Some(1))
    }
}
