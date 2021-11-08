use async_trait::async_trait;
use futures::TryStreamExt;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::api::ListParams;
use kube::error::Error;
use kube::{Api, Client};
use kube_runtime::reflector::store::Writer;
use kube_runtime::reflector::Store;
use kube_runtime::utils::try_flatten_applied;
use kube_runtime::{reflector, watcher};
use log::info;
use tokio::task;

#[async_trait]
pub(crate) trait KubeClient {
    async fn get_config_map(&self, name: &str) -> Result<ConfigMap, Error>;

    fn watch_config_map<T: Fn(ConfigMap) + Send + Sync + 'static>(
        &self,
        callback: T,
        label_selector: String,
    ) -> Store<ConfigMap>;
}

pub(crate) struct KubeClientImpl {
    config_maps_api: Api<ConfigMap>,
}

impl KubeClientImpl {
    pub(crate) async fn new(namespace: &str) -> Result<KubeClientImpl, Error> {
        let client = Client::try_default().await?;

        let config_maps_api: Api<ConfigMap> = Api::namespaced(client, namespace);

        Ok(KubeClientImpl { config_maps_api })
    }
}

#[async_trait]
impl KubeClient for KubeClientImpl {
    async fn get_config_map(&self, name: &str) -> Result<ConfigMap, Error> {
        self.config_maps_api.get(name).await
    }

    fn watch_config_map<T: Fn(ConfigMap) + Send + Sync + 'static>(
        &self,
        callback: T,
        label_selector: String,
    ) -> Store<ConfigMap> {
        info!(
            "Starting watch on ConfigMap with labels: {}...",
            label_selector
        );

        let writer = Writer::<ConfigMap>::default();

        let reader = writer.as_reader();

        let config_maps_api = self.config_maps_api.clone();

        let label_selector_clone = label_selector.clone();

        task::spawn(async move {
            try_flatten_applied(reflector(
                writer,
                watcher(
                    config_maps_api,
                    ListParams {
                        label_selector: Some(label_selector_clone),
                        ..ListParams::default()
                    },
                ),
            ))
            .try_for_each(|config_map| async {
                callback(config_map);
                Ok(())
            })
            .await
        });

        info!(
            "Started watch on ConfigMap with labels: {}.",
            label_selector
        );

        reader
    }
}
