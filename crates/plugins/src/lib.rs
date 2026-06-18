use std::sync::{Arc, Mutex};

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Clone)]
pub struct PluginRegistry {
    inner: Arc<Mutex<Vec<Arc<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register(&self, p: Arc<dyn Plugin>) {
        match self.inner.lock() {
            Ok(mut g) => g.push(p),
            Err(poisoned) => poisoned.into_inner().push(p),
        }
    }

    pub fn list_names(&self) -> Vec<String> {
        match self.inner.lock() {
            Ok(g) => g.iter().map(|p| p.name().to_string()).collect(),
            Err(poisoned) => poisoned
                .into_inner()
                .iter()
                .map(|p| p.name().to_string())
                .collect(),
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct Dummy;
    impl Plugin for Dummy {
        fn name(&self) -> &str {
            "dummy"
        }
        fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[test]
    fn register_and_list() {
        let reg = PluginRegistry::new();
        reg.register(Arc::new(Dummy));
        let names = reg.list_names();
        assert_eq!(names, vec!["dummy".to_string()]);
    }
}
