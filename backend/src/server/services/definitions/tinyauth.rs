use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TinyAuth;

impl ServiceDefinition for TinyAuth {
    fn name(&self) -> &'static str {
        "TinyAuth"
    }
    fn description(&self) -> &'static str {
        "The simplest way to protect your apps with a login screen"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http3000, "/site.webmanifest", "Tinyauth", None)
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/tinyauth.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TinyAuth>));
