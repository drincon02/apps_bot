pub mod filereading {
    use toml::Table;
    use std::fs;
    use std::io::Error;
    use std::process::Command;
    #[derive(Debug)]
    pub enum FileErr {
        IoError(Error),
        TomlError(toml::de::Error)
    }
    #[derive(Debug)]
    pub enum BootFile<'a> {
        Read(&'a String),
        Run(&'a String),
        Delete(&'a String),
        Save(&'a String, Vec<String>, Vec<(String, String)>)
    }

    impl BootFile<'_> {
        pub fn call(&self) -> Result<&BootFile, FileErr>{
            match self {
                BootFile::Delete(filename) => {
                    match fs::remove_file(filename) {
                        Ok(_) => Ok(self),
                        Err(error) => Err(FileErr::IoError(error))
                    }
                },
                BootFile::Run(filename) => {
                    match fs::read_to_string(filename) {
                        Ok(file_string) => {
                            let toml_file: Result<&BootFile, FileErr> = match file_string.parse::<Table>() {
                                Ok(file) => {
                                    let mut key_name = String::from("general");
                                    let general_details = file.get(&key_name).unwrap();
                                    key_name = String::from("apps");
                                    let apps_to_open = general_details.get(&key_name).unwrap().as_array().unwrap();
                                    
                                    for app in apps_to_open {
                                        let mut app_process = Command::new(app.as_str().unwrap());
                                        app_process.spawn().expect("Something went wrong");
                                    }

                                    Ok(self)
                                },
                                Err(error) => Err(FileErr::TomlError(error))
                            };
                            toml_file

                        }
                        Err(error) => Err(FileErr::IoError(error))
                    }
                },
                BootFile::Read(_) => Ok(self),
                BootFile::Save(_,_,_) => Ok(self),
            }
        }
    }
}