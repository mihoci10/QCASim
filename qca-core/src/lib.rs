use semver::{BuildMetadata, Prerelease, Version};

pub mod design;
pub mod objects;
pub mod simulation;

pub mod analysis;

pub const QCA_CORE_VERSION: Version = Version {
    major: 0,
    minor: 1,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};
pub fn get_qca_core_version() -> String {
    QCA_CORE_VERSION.to_string()
}
