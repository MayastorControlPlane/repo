use crate::apis::{
    client::{Error, ResponseContent, ResponseContentUnexpected},
    configuration,
};
use actix_web_opentelemetry::ClientExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct ReplicasClient {
    configuration: Rc<configuration::Configuration>,
}

impl ReplicasClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait::async_trait(?Send)]
#[dyn_clonable::clonable]
pub trait Replicas: Clone {
    async fn del_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>>;
    async fn del_node_pool_replica_share(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>>;
    async fn del_pool_replica(
        &self,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>>;
    async fn del_pool_replica_share(
        &self,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>>;
    async fn get_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>>;
    async fn get_node_pool_replicas(
        &self,
        node_id: &str,
        pool_id: &str,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    async fn get_node_replicas(
        &self,
        id: &str,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    async fn get_replica(
        &self,
        id: &str,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>>;
    async fn get_replicas(
        &self,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    async fn put_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
        create_replica_body: crate::models::CreateReplicaBody,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>>;
    async fn put_node_pool_replica_share(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
        protocol: crate::models::ReplicaShareProtocol,
    ) -> Result<String, Error<crate::models::RestJsonError>>;
    async fn put_pool_replica(
        &self,
        pool_id: &str,
        replica_id: &str,
        create_replica_body: crate::models::CreateReplicaBody,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>>;
    async fn put_pool_replica_share(
        &self,
        pool_id: &str,
        replica_id: &str,
        protocol: crate::models::ReplicaShareProtocol,
    ) -> Result<String, Error<crate::models::RestJsonError>>;
}

#[async_trait::async_trait(?Send)]
impl Replicas for ReplicasClient {
    async fn del_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            Ok(())
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn del_node_pool_replica_share(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            Ok(())
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn del_pool_replica(
        &self,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/pools/{pool_id}/replicas/{replica_id}",
            configuration.base_path,
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            Ok(())
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn del_pool_replica_share(
        &self,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<(), Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/pools/{pool_id}/replicas/{replica_id}/share",
            configuration.base_path,
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            Ok(())
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn get_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<crate::models::Replica>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn get_node_pool_replicas(
        &self,
        node_id: &str,
        pool_id: &str,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<Vec<crate::models::Replica>>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn get_node_replicas(
        &self,
        id: &str,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{id}/replicas",
            configuration.base_path,
            id = crate::apis::client::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<Vec<crate::models::Replica>>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn get_replica(
        &self,
        id: &str,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/replicas/{id}",
            configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<crate::models::Replica>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn get_replicas(
        &self,
    ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/replicas", configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<Vec<crate::models::Replica>>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn put_node_pool_replica(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
        create_replica_body: crate::models::CreateReplicaBody,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.send_json(&create_replica_body).await
        } else {
            local_var_req_builder
                .trace_request()
                .send_json(&create_replica_body)
                .await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<crate::models::Replica>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn put_node_pool_replica_share(
        &self,
        node_id: &str,
        pool_id: &str,
        replica_id: &str,
        protocol: crate::models::ReplicaShareProtocol,
    ) -> Result<String, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share/{protocol}",
            configuration.base_path,
            node_id = crate::apis::client::urlencode(node_id),
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string(),
            protocol = protocol.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<String>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn put_pool_replica(
        &self,
        pool_id: &str,
        replica_id: &str,
        create_replica_body: crate::models::CreateReplicaBody,
    ) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/pools/{pool_id}/replicas/{replica_id}",
            configuration.base_path,
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.send_json(&create_replica_body).await
        } else {
            local_var_req_builder
                .trace_request()
                .send_json(&create_replica_body)
                .await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<crate::models::Replica>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    async fn put_pool_replica_share(
        &self,
        pool_id: &str,
        replica_id: &str,
        protocol: crate::models::ReplicaShareProtocol,
    ) -> Result<String, Error<crate::models::RestJsonError>> {
        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!(
            "{}/pools/{pool_id}/replicas/{replica_id}/share/{protocol}",
            configuration.base_path,
            pool_id = crate::apis::client::urlencode(pool_id),
            replica_id = replica_id.to_string(),
            protocol = protocol.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        if let Some(ref local_var_token) = configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;

        let local_var_status = local_var_resp.status();

        if local_var_status.is_success() {
            let local_var_content = local_var_resp.json::<String>().await?;
            Ok(local_var_content)
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
}
