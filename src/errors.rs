use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyErrors {
    #[error("[thiserror] File Not Found.")]
    MyFileNotFoundError(#[from] std::io::Error),

    #[error("[thiserror] Permission Error.")]
    MyPermissionError,
}

fn f1_0() -> Result<(), MyErrors> {
    Err(MyErrors::MyPermissionError)
}

fn f1_1() -> Result<()> {
    // Err(anyhow!(MyErrors::MyPermissionError))
    bail!(MyErrors::MyPermissionError)
}

fn f2() -> Result<()> {
    if true {
        f1_0()?;
    } else {
        f1_1()?;
    }
    Ok(())
}

pub fn error_stuff() -> Result<()> {
    if let Err(err) = f2() {
        match err.downcast_ref() {
            Some(MyErrors::MyPermissionError) => {
                println!("MyErrors::MyPermissionError");
            }
            _ => {
                println!("No check");
            }
        }
    }

    Ok(())
}
