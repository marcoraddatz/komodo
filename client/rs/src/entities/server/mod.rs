use derive_builder::Builder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::resource::{Resource, ResourceListItem, ResourceQuery};

pub mod docker_image;
pub mod docker_network;
pub mod stats;

#[typeshare]
pub type Server = Resource<ServerConfig, ()>;

#[typeshare]
pub type ServerListItem = ResourceListItem<ServerListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerListItemInfo {
  pub status: ServerStatus,
  pub region: String,
  pub send_unreachable_alerts: bool,
  pub send_cpu_alerts: bool,
  pub send_mem_alerts: bool,
  pub send_disk_alerts: bool,
  pub send_temp_alerts: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct ServerConfig {
  pub address: String,

  #[serde(default = "default_enabled")]
  #[builder(default = "default_enabled()")]
  #[partial_default(default_enabled())]
  pub enabled: bool,

  #[serde(default = "default_auto_prune")]
  #[builder(default = "default_auto_prune()")]
  #[partial_default(default_auto_prune())]
  pub auto_prune: bool,

  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_unreachable_alerts: bool,

  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_cpu_alerts: bool,

  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_mem_alerts: bool,

  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_disk_alerts: bool,

  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_temp_alerts: bool,

  #[serde(default)]
  #[builder(default)]
  pub region: String,

  #[serde(default = "default_cpu_warning")]
  #[builder(default = "default_cpu_warning()")]
  #[partial_default(default_cpu_warning())]
  pub cpu_warning: f32,

  #[serde(default = "default_cpu_critical")]
  #[builder(default = "default_cpu_critical()")]
  #[partial_default(default_cpu_critical())]
  pub cpu_critical: f32,

  #[serde(default = "default_mem_warning")]
  #[builder(default = "default_mem_warning()")]
  #[partial_default(default_mem_warning())]
  pub mem_warning: f64,

  #[serde(default = "default_mem_critical")]
  #[builder(default = "default_mem_critical()")]
  #[partial_default(default_mem_critical())]
  pub mem_critical: f64,

  #[serde(default = "default_disk_warning")]
  #[builder(default = "default_disk_warning()")]
  #[partial_default(default_disk_warning())]
  pub disk_warning: f64,

  #[serde(default = "default_disk_critical")]
  #[builder(default = "default_disk_critical()")]
  #[partial_default(default_disk_critical())]
  pub disk_critical: f64,
}

fn default_enabled() -> bool {
  true
}

fn default_auto_prune() -> bool {
  true
}

fn default_send_alerts() -> bool {
  true
}

fn default_cpu_warning() -> f32 {
  90.0
}

fn default_cpu_critical() -> f32 {
  99.0
}

fn default_mem_warning() -> f64 {
  75.0
}

fn default_mem_critical() -> f64 {
  95.0
}

fn default_disk_warning() -> f64 {
  75.0
}

fn default_disk_critical() -> f64 {
  95.0
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerActionState {
  pub pruning_networks: bool,
  pub pruning_containers: bool,
  pub pruning_images: bool,
  pub stopping_containers: bool,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
)]
pub enum ServerStatus {
  #[default]
  NotOk,
  Ok,
  Disabled,
}

#[typeshare]
pub type ServerQuery = ResourceQuery<ServerQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerQuerySpecifics {
  
}
