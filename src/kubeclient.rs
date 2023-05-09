use futures::TryStreamExt;
use k8s_openapi::api::core::v1::ConfigMap;

use kube::error::Error;
use kube::{Api, Client};
use kube_runtime::reflector::store::Writer;
use kube_runtime::reflector::Store;
use kube_runtime::watcher::Config;
use kube_runtime::{reflector, watcher, WatchStreamExt};
use log::info;
use tokio::task;

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

        let watcher_config = Config::default().labels(&label_selector);

        task::spawn(async move {
            reflector(writer, watcher(config_maps_api, watcher_config))
                .applied_objects()
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
