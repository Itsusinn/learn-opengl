use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi::CString;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum Error {
   #[error("I/O 错误")]
   IO(#[from] io::Error),
   #[error("Failed to read CString from file that contains 0")]
   FileContainsNil,
   #[error("Failed get executable path")]
   FailedToGetExePath
}

pub struct Resources {
   root_path: PathBuf
}

impl Resources {
   pub fn get_root_path(&self) -> &PathBuf{
      &self.root_path
   }

   pub fn from_relative_exe_path(
      rel_path:&Path
   ) -> Result<Resources, Error>{
      let exe_file_name = ::std::env::current_exe()
         .map_err(|_| Error::FailedToGetExePath)?;
      let exe_path = exe_file_name.parent()
         .ok_or(Error::FailedToGetExePath)?;

      Ok(Resources {
         root_path: exe_path.join(rel_path)
      })
   }
   pub fn load_cstring(
      &self,
      resource_name: &str
   ) -> Result<CString, Error> {
      let mut file = fs::File::open(
         resource_name_to_path(&self.root_path,resource_name)
      )?;

      let mut buffer = Vec::with_capacity(
         file.metadata()?.len() as usize + 1
      );
      file.read_to_end(&mut buffer)?;

      //  检查空字符'\0'
      if buffer.iter().any(|i| *i == 0) {
         return Err(Error::FileContainsNil);
      }

      Ok( unsafe{ CString::from_vec_unchecked(buffer) } )
   }
}

fn resource_name_to_path(
   root_dir: &Path,
   location: &str
) -> PathBuf {
   let mut path:PathBuf = root_dir.into();
   for part in location.split('/') {
      // path.join(part)
      path = path.join(part)
   }
   path
}