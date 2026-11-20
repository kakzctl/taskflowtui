/// A named collection used to organize tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: u32,
    pub name: String,
}

impl Project {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::Project;

    #[test]
    fn project_stores_its_id_and_name() {
        let project = Project::new(7, "Learning");
        assert_eq!(project.id, 7);
        assert_eq!(project.name, "Learning");
    }
}
