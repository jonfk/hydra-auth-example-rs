use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    admin_api: Box<dyn crate::apis::AdminApi>,
    public_api: Box<dyn crate::apis::PublicApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            admin_api: Box::new(crate::apis::AdminApiClient::new(rc.clone())),
            public_api: Box::new(crate::apis::PublicApiClient::new(rc.clone())),
        }
    }

    pub fn admin_api(&self) -> &dyn crate::apis::AdminApi{
        self.admin_api.as_ref()
    }

    pub fn public_api(&self) -> &dyn crate::apis::PublicApi{
        self.public_api.as_ref()
    }

}
