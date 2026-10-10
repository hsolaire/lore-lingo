use crate::desktop::{
    CapturePermission, CaptureSourceCatalog, CaptureSourceSummary, DesktopCapabilities,
    DesktopError, NativeCaptureSource, PlatformBackend,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct CatalogState {
    revision: u64,
    native_sources: HashMap<String, NativeCaptureSource>,
}

pub struct DesktopService {
    backend: Arc<dyn PlatformBackend>,
    catalog: Mutex<CatalogState>,
}

impl DesktopService {
    pub fn new(backend: Arc<dyn PlatformBackend>) -> Self {
        Self {
            backend,
            catalog: Mutex::new(CatalogState::default()),
        }
    }

    pub fn capabilities(&self) -> DesktopCapabilities {
        self.backend.capabilities()
    }

    pub fn capture_permission(&self) -> Result<CapturePermission, DesktopError> {
        self.backend
            .capture_permission()
            .map_err(|error| error.to_public())
    }

    pub fn request_capture_permission(&self) -> Result<CapturePermission, DesktopError> {
        self.backend
            .request_capture_permission()
            .map_err(|error| error.to_public())
    }

    pub fn list_capture_sources(&self) -> Result<CaptureSourceCatalog, DesktopError> {
        let discovered = self
            .backend
            .discover_capture_sources()
            .map_err(|error| error.to_public())?;
        let mut catalog = self
            .catalog
            .lock()
            .map_err(|_| DesktopError::backend_failure())?;
        let revision = catalog
            .revision
            .checked_add(1)
            .ok_or_else(DesktopError::backend_failure)?;
        let mut native_sources = HashMap::with_capacity(discovered.len());
        let mut sources = Vec::with_capacity(discovered.len());

        for (index, source) in discovered.into_iter().enumerate() {
            let source_id = format!("capture-source-{revision}-{}", index + 1);
            native_sources.insert(source_id.clone(), source.native_source);
            sources.push(CaptureSourceSummary {
                source_id,
                catalog_revision: revision,
                kind: source.kind,
                title: source.title,
                application_name: source.application_name,
                icon_data_url: source.icon_data_url,
                monitor: source.monitor,
                capturable: source.capturable,
            });
        }

        catalog.revision = revision;
        catalog.native_sources = native_sources;
        Ok(CaptureSourceCatalog {
            catalog_revision: revision,
            sources,
        })
    }

    pub fn resolve_source(
        &self,
        catalog_revision: u64,
        source_id: &str,
    ) -> Result<NativeCaptureSource, DesktopError> {
        let catalog = self
            .catalog
            .lock()
            .map_err(|_| DesktopError::backend_failure())?;
        if catalog.revision != catalog_revision {
            return Err(DesktopError::source_stale());
        }
        catalog
            .native_sources
            .get(source_id)
            .cloned()
            .ok_or_else(DesktopError::source_not_found)
    }

    pub fn validate_source(
        &self,
        catalog_revision: u64,
        source_id: &str,
    ) -> Result<(), DesktopError> {
        let catalog = self
            .catalog
            .lock()
            .map_err(|_| DesktopError::backend_failure())?;
        if catalog_revision != catalog.revision {
            return Err(DesktopError::source_stale());
        }
        if !catalog.native_sources.contains_key(source_id) {
            return Err(DesktopError::source_not_found());
        }
        Ok(())
    }
}
