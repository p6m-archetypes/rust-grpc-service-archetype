mod r#impl;
pub mod settings;
pub mod store;

use anyhow::Result;
use settings::CoreSettings;
{% if persistence ~= 'None' %}

use {{ project_name }}_persistence::PersistencePool;
{% endif %}
{% if cache ~= 'None' %}

use {{ project_name }}_cache::CachePool;
{% endif %}
{% if messaging ~= 'None' %}

use {{ project_name }}_messaging::MessagingClient;
{% endif %}

pub mod proto {
    tonic::include_proto!("{{ project_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("{{ project_name }}_descriptor");
}

#[derive(Clone)]
pub struct {{ ProjectName }}Core {
    pub(crate) store: store::Store,
{% if cache ~= 'None' %}
    #[allow(dead_code)]
    pub(crate) cache: CachePool,
{% endif %}
{% if messaging ~= 'None' %}
    #[allow(dead_code)]
    pub(crate) messaging: MessagingClient,
{% endif %}
    #[allow(dead_code)]
    settings: CoreSettings,
}

impl {{ ProjectName }}Core {
    pub fn builder({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Builder {
        Builder::new({% if persistence ~= 'None' %}db{% endif %})
    }
}

pub struct Builder {
{% if persistence ~= 'None' %}
    db: PersistencePool,
{% endif %}
{% if cache ~= 'None' %}
    cache: Option<CachePool>,
{% endif %}
{% if messaging ~= 'None' %}
    messaging: Option<MessagingClient>,
{% endif %}
    settings: CoreSettings,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Self {
        Self {
{% if persistence ~= 'None' %}
            db,
{% endif %}
{% if cache ~= 'None' %}
            cache: None,
{% endif %}
{% if messaging ~= 'None' %}
            messaging: None,
{% endif %}
            settings: CoreSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

{% if cache ~= 'None' %}
    pub fn with_cache(mut self, cache: CachePool) -> Self {
        self.cache = Some(cache);
        self
    }

{% endif %}
{% if messaging ~= 'None' %}
    pub fn with_messaging(mut self, messaging: MessagingClient) -> Self {
        self.messaging = Some(messaging);
        self
    }

{% endif %}
    pub async fn build(self) -> Result<{{ ProjectName }}Core> {
        Ok({{ ProjectName }}Core {
{% if persistence ~= 'None' %}
            store: store::Store::new(self.db),
{% else %}
            store: store::Store::default(),
{% endif %}
{% if cache ~= 'None' %}
            cache: self.cache.expect("cache must be initialized with with_cache()"),
{% endif %}
{% if messaging ~= 'None' %}
            messaging: self.messaging.expect("messaging must be initialized with with_messaging()"),
{% endif %}
            settings: self.settings,
        })
    }
}
