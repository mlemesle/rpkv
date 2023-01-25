use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufWriter;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use thiserror::Error;

type RPKVResult<T> = Result<T, Error>;

type Store = HashMap<String, String>;

const DB_PATH: &str = "./rpkv.db";

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    StdIO(#[from] std::io::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
}

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        match value {
            Error::StdIO(err) => PyIOError::new_err(err),
            Error::Bincode(err) => PyBufferError::new_err(err.to_string()),
        }
    }
}

fn get_store() -> RPKVResult<Store> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(DB_PATH)?;
    let store = bincode::deserialize_from::<_, Store>(&mut file).unwrap_or_default();

    Ok(store)
}

fn write_store(store: Store) -> RPKVResult<()> {
    let mut file = BufWriter::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(DB_PATH)?,
    );
    bincode::serialize_into(&mut file, &store)?;

    Ok(())
}

#[pyfunction]
fn add_to_keystore(key: String, value: String) -> RPKVResult<()> {
    let mut store = get_store()?;
    store.insert(key, value);
    write_store(store)?;
    Ok(())
}

#[pyfunction]
fn get_from_keystore(key: String) -> RPKVResult<Option<String>> {
    let store = get_store()?;

    Ok(store.get(&key).cloned())
}

#[pyfunction]
fn get_path() -> RPKVResult<&'static str> {
    Ok(DB_PATH)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rpkv(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_to_keystore, m)?)?;
    m.add_function(wrap_pyfunction!(get_from_keystore, m)?)?;
    m.add_function(wrap_pyfunction!(get_path, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::add_to_keystore;

    #[test]
    fn toto() {
        add_to_keystore("toto".into(), "rue des pets".into()).unwrap();
    }
}
