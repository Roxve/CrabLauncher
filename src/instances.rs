#[derive(Debug)]
pub enum VersionKind {
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
}
#[derive(Debug)]
pub struct Version {
    pub kind: VersionKind,
    pub url: String,
}
