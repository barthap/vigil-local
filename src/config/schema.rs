// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::defaults;
use crate::probe::mode::Mode;
use crate::probe::replica::ReplicaURL;

#[derive(Deserialize)]
pub struct Config {
  pub server: ConfigServer,
  pub report: ConfigReport,
  pub metrics: ConfigMetrics,
  pub probe: ConfigProbe,
}

#[derive(Deserialize)]
pub struct ConfigServer {
  #[serde(default = "defaults::server_log_level")]
  pub log_level: String,
}

#[derive(Deserialize)]
pub struct ConfigReport {
  pub endpoint: String,
  pub token: String,
}

#[derive(Deserialize)]
pub struct ConfigMetrics {
  #[serde(default = "defaults::metrics_interval")]
  pub interval: u64,

  #[serde(default = "defaults::metrics_poll_retry")]
  pub poll_retry: u8,

  #[serde(default = "defaults::metrics_poll_delay_dead")]
  pub poll_delay_dead: u64,

  #[serde(default = "defaults::metrics_poll_delay_sick")]
  pub poll_delay_sick: u64,
}

#[derive(Deserialize)]
pub struct ConfigProbe {
  pub service: Vec<ConfigProbeService>,
}

#[derive(Deserialize)]
pub struct ConfigProbeService {
  pub id: String,
  pub node: Vec<ConfigProbeServiceNode>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
  #[default]
  Get,
  Head,
  Post,
  Put,
  Patch,
}

#[derive(Deserialize)]
pub struct ConfigProbeServiceNode {
  pub id: String,
  pub mode: Mode,
  pub replicas: Option<Vec<ConfigProbeServiceReplicaNode>>,
  pub scripts: Option<Vec<ConfigProbeServiceScriptNode>>,
  pub http_method: Option<HttpMethod>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigProbeServiceReplicaNode {
  Extended {
    url: ReplicaURL,
    label: String,
    id: Option<String>,
  },
  Simple(ReplicaURL),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigProbeServiceScriptNode {
  Extended {
    script: String,
    label: String,
    id: Option<String>,
  },
  Simple(String),
}

impl ConfigProbeServiceReplicaNode {
  pub fn id(&self) -> &str {
    if let Self::Extended { id: Some(id), .. } = self {
      return id;
    }
    self.url().get_raw()
  }

  pub fn url(&self) -> &ReplicaURL {
    match self {
      Self::Extended { url, .. } => url,
      Self::Simple(replica_url) => replica_url,
    }
  }

  pub fn label(&self) -> Option<&str> {
    match self {
      Self::Extended { label, .. } => Some(label),
      _ => None,
    }
  }
}

impl ConfigProbeServiceScriptNode {
  pub fn id(&self) -> Option<&str> {
    if let Self::Extended { id: Some(id), .. } = self {
      return Some(id);
    }
    None
  }

  pub fn script_content(&self) -> &str {
    match self {
      Self::Extended { script, .. } => script,
      Self::Simple(script) => script,
    }
  }

  pub fn label(&self) -> Option<&str> {
    match self {
      Self::Extended { label, .. } => Some(label),
      _ => None,
    }
  }
}
