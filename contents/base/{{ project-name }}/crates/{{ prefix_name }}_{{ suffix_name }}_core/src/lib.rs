mod r#impl;
pub mod settings;

use anyhow::Result;
use settings::CoreSettings;
{% if persistence ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_persistence::PersistencePool;
{% endif %}{% if cache ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_cache::CachePool;
{% endif %}{% if messaging ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_messaging::MessagingClient;
{% endif %}
pub mod proto {
    tonic::include_proto!("{{ prefix_name }}_{{ suffix_name }}");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("{{ prefix_name }}_{{ suffix_name }}_descriptor");
}

#[derive(Clone)]
pub struct {{ PrefixName }}{{ SuffixName }}Core {
{% if persistence ~= 'None' %}    #[allow(dead_code)]
    pub(crate) db: PersistencePool,
{% endif %}{% if cache ~= 'None' %}    #[allow(dead_code)]
    pub(crate) cache: CachePool,
{% endif %}{% if messaging ~= 'None' %}    #[allow(dead_code)]
    pub(crate) messaging: MessagingClient,
{% endif %}    #[allow(dead_code)]
    settings: CoreSettings,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
    pub fn builder({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Builder {
        Builder::new({% if persistence ~= 'None' %}db{% endif %})
    }
}

pub struct Builder {
{% if persistence ~= 'None' %}    db: PersistencePool,
{% endif %}{% if cache ~= 'None' %}    cache: Option<CachePool>,
{% endif %}{% if messaging ~= 'None' %}    messaging: Option<MessagingClient>,
{% endif %}    settings: CoreSettings,
}

impl Builder {
    pub fn new({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Self {
        Self {
{% if persistence ~= 'None' %}            db,
{% endif %}{% if cache ~= 'None' %}            cache: None,
{% endif %}{% if messaging ~= 'None' %}            messaging: None,
{% endif %}            settings: CoreSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

{% if cache ~= 'None' %}    pub fn with_cache(mut self, cache: CachePool) -> Self {
        self.cache = Some(cache);
        self
    }

{% endif %}{% if messaging ~= 'None' %}    pub fn with_messaging(mut self, messaging: MessagingClient) -> Self {
        self.messaging = Some(messaging);
        self
    }

{% endif %}    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        Ok({{ PrefixName }}{{ SuffixName }}Core {
{% if persistence ~= 'None' %}            db: self.db,
{% endif %}{% if cache ~= 'None' %}            cache: self.cache.expect("cache must be initialized with with_cache()"),
{% endif %}{% if messaging ~= 'None' %}            messaging: self.messaging.expect("messaging must be initialized with with_messaging()"),
{% endif %}            settings: self.settings,
        })
    }
}
