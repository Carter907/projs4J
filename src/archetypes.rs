use std::fmt::{Display, Formatter};
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
}