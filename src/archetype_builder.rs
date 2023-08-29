use std::env;
use std::fs::File;
use std::path::Path;

pub struct ArchetypeBuilder {

}
impl ArchetypeBuilder {

    pub fn init_projects() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_dir = Path::new(&out_dir).join(Path::new("included_archetypes"));
        let mut output = File::create(&dest_dir).unwrap();

        let path = Path::new("archetypes");

    }
}