use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};
use clap::error::ErrorKind;
use clap::error::ErrorKind::ArgumentConflict;
use clap::ValueEnum;
use strum_macros::{Display, EnumIter};


#[derive(Display, EnumIter, ValueEnum, Clone, Copy)]
pub enum MavenArchetypes {
    DeepLearning4J,
    HelloWorld,
    JavaFx,
    Jdbc,
    Lwjgl,
    OpenCv,
    LibGdx,

}
impl MavenArchetypes {

}


pub struct MavenArchetypeInfo(pub MavenArchetypes);

impl Display for MavenArchetypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self.0 {
            MavenArchetypes::HelloWorld => {
                write!(f, "hello-world-archetype")
            }
            _ => {
                write!(f, "some archetype")
            }
        }

    }
}
impl MavenArchetypeInfo {
    pub(crate) fn group_id(&self) -> String {
        match self.0 {
            MavenArchetypes::HelloWorld => {
                "org.example".to_string()
            }
            _ => {
                "org.example".to_string()
            }
        }
    }
    pub(crate) fn path(&self) -> std::io::Result<PathBuf> {

        match self.0 {
            MavenArchetypes::HelloWorld => {
                let dir = format!("{}/archetypes/hello-world-archetype", std::env::current_dir().unwrap().to_str().unwrap());
                println!("{dir}");
                Ok(PathBuf::from(dir))
            }
            _ => {
                Err(Error::new(std::io::ErrorKind::Unsupported, "archetype not supported yet."))
            }
        }
    }
}