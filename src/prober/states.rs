// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::{Duration, SystemTime};

use indexmap::IndexMap;
use serde_derive::Serialize;

use super::mode::Mode;
use super::replica::ReplicaURL;
use super::status::Status;
use crate::config::regex::Regex;

#[derive(Serialize)]
pub struct ServiceStates {
    pub status: Status,
    pub date: Option<String>,
    pub probes: IndexMap<String, ServiceStatesProbe>,
}

#[derive(Serialize)]
pub struct ServiceStatesProbe {
    pub id: String,
    pub label: String,
    pub status: Status,
    pub nodes: IndexMap<String, ServiceStatesProbeNode>,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNode {
    pub status: Status,
    pub label: String,
    pub mode: Mode,
    pub replicas: IndexMap<String, ServiceStatesProbeNodeReplica>,
    pub http_body_healthy_match: Option<Regex>,
    #[serde(default)]
    #[serde(with = "http_serde::header_map")]
    pub http_headers: http::HeaderMap,
    pub http_method: Option<String>,
    pub http_body: Option<String>,
    pub rabbitmq_queue: Option<String>,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplica {
    pub status: Status,
    pub url: Option<ReplicaURL>,
    pub script: Option<String>,
    pub metrics: ServiceStatesProbeNodeReplicaMetrics,
    pub load: Option<ServiceStatesProbeNodeReplicaLoad>,
    pub report: Option<ServiceStatesProbeNodeReplicaReport>,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaMetrics {
    pub latency: Option<u64>,
    pub system: Option<ServiceStatesProbeNodeReplicaMetricsSystem>,
    pub rabbitmq: Option<ServiceStatesProbeNodeReplicaMetricsRabbitMQ>,
}

#[derive(Serialize, Clone)]
pub struct ServiceStatesProbeNodeReplicaMetricsSystem {
    pub cpu: u16,
    pub ram: u16,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaMetricsRabbitMQ {
    pub queue_ready: u32,
    pub queue_nack: u32,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplicaLoad {
    pub cpu: f32,
    pub ram: f32,
    pub queue: ServiceStatesProbeNodeReplicaLoadQueue,
}

#[derive(Serialize, Clone, Default)]
pub struct ServiceStatesProbeNodeReplicaLoadQueue {
    pub loaded: bool,
    pub stalled: bool,
}

#[derive(Serialize)]
pub struct ServiceStatesProbeNodeReplicaReport {
    pub time: SystemTime,
    pub interval: Duration,
}
