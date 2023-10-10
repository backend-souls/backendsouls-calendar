use derive_builder::Builder;
use getset::Getters;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Type {
    Event,
    Task,
    Group,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match &self {
            Type::Event => "Event",
            Type::Group => "Group",
            Type::Task => "Task",
        }
    }
}

#[derive(Debug, Clone, Copy, Builder, Getters)]
#[getset(get)]
pub struct MetadataProperties {
    _type: Type,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() -> anyhow::Result<()> {
        let metadata = MetadataPropertiesBuilder::default()._type(Type::Group).build()?;

        assert_eq!(Type::Group, *metadata._type());

        Ok(())
    }
}
