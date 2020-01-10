/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct AdminApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl AdminApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> AdminApiClient {
        AdminApiClient { configuration }
    }
}

pub trait AdminApi {
    fn accept_consent_request(
        &self,
        consent_challenge: &str,
        body: Option<crate::models::AcceptConsentRequest>,
    ) -> Result<crate::models::CompletedRequest, Error>;
    fn accept_login_request(
        &self,
        login_challenge: &str,
        body: Option<crate::models::AcceptLoginRequest>,
    ) -> Result<crate::models::CompletedRequest, Error>;
    fn accept_logout_request(
        &self,
        logout_challenge: &str,
    ) -> Result<crate::models::CompletedRequest, Error>;
    fn create_json_web_key_set(
        &self,
        set: &str,
        body: Option<crate::models::JsonWebKeySetGeneratorRequest>,
    ) -> Result<crate::models::JsonWebKeySet, Error>;
    fn create_o_auth2_client(
        &self,
        body: crate::models::OAuth2Client,
    ) -> Result<crate::models::OAuth2Client, Error>;
    fn delete_json_web_key(&self, kid: &str, set: &str) -> Result<(), Error>;
    fn delete_json_web_key_set(&self, set: &str) -> Result<(), Error>;
    fn delete_o_auth2_client(&self, id: &str) -> Result<(), Error>;
    fn flush_inactive_o_auth2_tokens(
        &self,
        body: Option<crate::models::FlushInactiveOAuth2TokensRequest>,
    ) -> Result<(), Error>;
    fn get_consent_request(
        &self,
        consent_challenge: &str,
    ) -> Result<crate::models::ConsentRequest, Error>;
    fn get_json_web_key(&self, kid: &str, set: &str)
        -> Result<crate::models::JsonWebKeySet, Error>;
    fn get_json_web_key_set(&self, set: &str) -> Result<crate::models::JsonWebKeySet, Error>;
    fn get_login_request(
        &self,
        login_challenge: &str,
    ) -> Result<crate::models::LoginRequest, Error>;
    fn get_logout_request(
        &self,
        logout_challenge: &str,
    ) -> Result<crate::models::LogoutRequest, Error>;
    fn get_o_auth2_client(&self, id: &str) -> Result<crate::models::OAuth2Client, Error>;
    fn get_version(&self) -> Result<crate::models::Version, Error>;
    fn introspect_o_auth2_token(
        &self,
        token: &str,
        scope: Option<&str>,
    ) -> Result<crate::models::OAuth2TokenIntrospection, Error>;
    fn is_instance_alive(&self) -> Result<crate::models::HealthStatus, Error>;
    fn list_o_auth2_clients(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::models::OAuth2Client>, Error>;
    fn list_subject_consent_sessions(
        &self,
        subject: &str,
    ) -> Result<Vec<crate::models::PreviousConsentSession>, Error>;
    fn prometheus(&self) -> Result<(), Error>;
    fn reject_consent_request(
        &self,
        consent_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<crate::models::CompletedRequest, Error>;
    fn reject_login_request(
        &self,
        login_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<crate::models::CompletedRequest, Error>;
    fn reject_logout_request(
        &self,
        logout_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<(), Error>;
    fn revoke_authentication_session(&self, subject: &str) -> Result<(), Error>;
    fn revoke_consent_sessions(&self, subject: &str, client: Option<&str>) -> Result<(), Error>;
    fn update_json_web_key(
        &self,
        kid: &str,
        set: &str,
        body: Option<crate::models::JsonWebKey>,
    ) -> Result<crate::models::JsonWebKey, Error>;
    fn update_json_web_key_set(
        &self,
        set: &str,
        body: Option<crate::models::JsonWebKeySet>,
    ) -> Result<crate::models::JsonWebKeySet, Error>;
    fn update_o_auth2_client(
        &self,
        id: &str,
        body: crate::models::OAuth2Client,
    ) -> Result<crate::models::OAuth2Client, Error>;
}

impl AdminApi for AdminApiClient {
    fn accept_consent_request(
        &self,
        consent_challenge: &str,
        body: Option<crate::models::AcceptConsentRequest>,
    ) -> Result<crate::models::CompletedRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/consent/accept",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("consent_challenge", &consent_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn accept_login_request(
        &self,
        login_challenge: &str,
        body: Option<crate::models::AcceptLoginRequest>,
    ) -> Result<crate::models::CompletedRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/login/accept",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("login_challenge", &login_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn accept_logout_request(
        &self,
        logout_challenge: &str,
    ) -> Result<crate::models::CompletedRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/logout/accept",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("logout_challenge", &logout_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn create_json_web_key_set(
        &self,
        set: &str,
        body: Option<crate::models::JsonWebKeySetGeneratorRequest>,
    ) -> Result<crate::models::JsonWebKeySet, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}",
            configuration.base_path,
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn create_o_auth2_client(
        &self,
        body: crate::models::OAuth2Client,
    ) -> Result<crate::models::OAuth2Client, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/clients", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_json_web_key(&self, kid: &str, set: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}/{kid}",
            configuration.base_path,
            kid = crate::apis::urlencode(kid),
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_json_web_key_set(&self, set: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}",
            configuration.base_path,
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_o_auth2_client(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/clients/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn flush_inactive_o_auth2_tokens(
        &self,
        body: Option<crate::models::FlushInactiveOAuth2TokensRequest>,
    ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/flush", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_consent_request(
        &self,
        consent_challenge: &str,
    ) -> Result<crate::models::ConsentRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/requests/consent", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("consent_challenge", &consent_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_json_web_key(
        &self,
        kid: &str,
        set: &str,
    ) -> Result<crate::models::JsonWebKeySet, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}/{kid}",
            configuration.base_path,
            kid = crate::apis::urlencode(kid),
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_json_web_key_set(&self, set: &str) -> Result<crate::models::JsonWebKeySet, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}",
            configuration.base_path,
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_login_request(
        &self,
        login_challenge: &str,
    ) -> Result<crate::models::LoginRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/requests/login", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("login_challenge", &login_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_logout_request(
        &self,
        logout_challenge: &str,
    ) -> Result<crate::models::LogoutRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/requests/logout", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("logout_challenge", &logout_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_o_auth2_client(&self, id: &str) -> Result<crate::models::OAuth2Client, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/clients/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_version(&self) -> Result<crate::models::Version, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/version", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn introspect_o_auth2_token(
        &self,
        token: &str,
        scope: Option<&str>,
    ) -> Result<crate::models::OAuth2TokenIntrospection, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/introspect", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("token", token.to_string());
        if let Some(param_value) = scope {
            form_params.insert("scope", param_value.to_string());
        }
        req_builder = req_builder.form(&form_params);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn is_instance_alive(&self) -> Result<crate::models::HealthStatus, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/health/alive", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn list_o_auth2_clients(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::models::OAuth2Client>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/clients", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        if let Some(ref s) = offset {
            req_builder = req_builder.query(&[("offset", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn list_subject_consent_sessions(
        &self,
        subject: &str,
    ) -> Result<Vec<crate::models::PreviousConsentSession>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/sessions/consent", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("subject", &subject.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn prometheus(&self) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/metrics/prometheus", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn reject_consent_request(
        &self,
        consent_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<crate::models::CompletedRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/consent/reject",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("consent_challenge", &consent_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn reject_login_request(
        &self,
        login_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<crate::models::CompletedRequest, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/login/reject",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("login_challenge", &login_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn reject_logout_request(
        &self,
        logout_challenge: &str,
        body: Option<crate::models::RejectRequest>,
    ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/oauth2/auth/requests/logout/reject",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        req_builder = req_builder.query(&[("logout_challenge", &logout_challenge.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn revoke_authentication_session(&self, subject: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/sessions/login", configuration.base_path);
        let mut req_builder = client.delete(uri_str.as_str());

        req_builder = req_builder.query(&[("subject", &subject.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn revoke_consent_sessions(&self, subject: &str, client_id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/oauth2/auth/sessions/consent", configuration.base_path);
        let mut req_builder = client.delete(uri_str.as_str());

        req_builder = req_builder.query(&[("subject", &subject.to_string())]);
        if let Some(ref s) = client_id {
            req_builder = req_builder.query(&[("client", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn update_json_web_key(
        &self,
        kid: &str,
        set: &str,
        body: Option<crate::models::JsonWebKey>,
    ) -> Result<crate::models::JsonWebKey, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}/{kid}",
            configuration.base_path,
            kid = crate::apis::urlencode(kid),
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_json_web_key_set(
        &self,
        set: &str,
        body: Option<crate::models::JsonWebKeySet>,
    ) -> Result<crate::models::JsonWebKeySet, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/keys/{set}",
            configuration.base_path,
            set = crate::apis::urlencode(set)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_o_auth2_client(
        &self,
        id: &str,
        body: crate::models::OAuth2Client,
    ) -> Result<crate::models::OAuth2Client, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/clients/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
