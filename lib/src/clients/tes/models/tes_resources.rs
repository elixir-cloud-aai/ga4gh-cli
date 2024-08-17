/*
 * Task Execution Service
 *
 * ## Executive Summary The Task Execution Service (TES) API is a standardized schema and API for describing and executing batch execution tasks. A task defines a set of input files, a set of containers and commands to run, a set of output files and some other logging and metadata.  TES servers accept task documents and execute them asynchronously on available compute resources. A TES server could be built on top of a traditional HPC queuing system, such as Grid Engine, Slurm or cloud style compute systems such as AWS Batch or Kubernetes. ## Introduction This document describes the TES API and provides details on the specific endpoints, request formats, and responses. It is intended to provide key information for developers of TES-compatible services as well as clients that will call these TES services. Use cases include:    - Deploying existing workflow engines on new infrastructure. Workflow engines   such as CWL-Tes and Cromwell have extentions for using TES. This will allow   a system engineer to deploy them onto a new infrastructure using a job scheduling   system not previously supported by the engine.    - Developing a custom workflow management system. This API provides a common   interface to asynchronous batch processing capabilities. A developer can write   new tools against this interface and expect them to work using a variety of   backend solutions that all support the same specification.   ## Standards The TES API specification is written in OpenAPI and embodies a RESTful service philosophy. It uses JSON in requests and responses and standard HTTP/HTTPS for information transport. HTTPS should be used rather than plain HTTP except for testing or internal-only purposes. ### Authentication and Authorization Is is envisaged that most TES API instances will require users to authenticate to use the endpoints. However, the decision if authentication is required should be taken by TES API implementers.  If authentication is required, we recommend that TES implementations use an OAuth2  bearer token, although they can choose other mechanisms if appropriate.  Checking that a user is authorized to submit TES requests is a responsibility of TES implementations. ### CORS If TES API implementation is to be used by another website or domain it must implement Cross Origin Resource Sharing (CORS). Please refer to https://w3id.org/ga4gh/product-approval-support/cors for more information about GA4GH’s recommendations and how to implement CORS. 
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::clients::tes::models;
use serde::{Deserialize, Serialize};

/// TesResources : Resources describes the resources requested by a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TesResources {
    /// Requested number of CPUs
    #[serde(rename = "cpu_cores", skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<i32>,
    /// Define if the task is allowed to run on preemptible compute instances, for example, AWS Spot. This option may have no effect when utilized on some backends that don't have the concept of preemptible jobs.
    #[serde(rename = "preemptible", skip_serializing_if = "Option::is_none")]
    pub preemptible: Option<bool>,
    /// Requested RAM required in gigabytes (GB)
    #[serde(rename = "ram_gb", skip_serializing_if = "Option::is_none")]
    pub ram_gb: Option<f64>,
    /// Requested disk size in gigabytes (GB)
    #[serde(rename = "disk_gb", skip_serializing_if = "Option::is_none")]
    pub disk_gb: Option<f64>,
    /// Request that the task be run in these compute zones. How this string is utilized will be dependent on the backend system. For example, a system based on a cluster queueing system may use this string to define priorty queue to which the job is assigned.
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    /// Key/value pairs for backend configuration. ServiceInfo shall return a list of keys that a backend supports. Keys are case insensitive. It is expected that clients pass all runtime or hardware requirement key/values that are not mapped to existing tesResources properties to backend_parameters. Backends shall log system warnings if a key is passed that is unsupported. Backends shall not store or return unsupported keys if included in a task. If backend_parameters_strict equals true, backends should fail the task if any key/values are unsupported, otherwise, backends should attempt to run the task Intended uses include VM size selection, coprocessor configuration, etc. Example: ``` {   \"backend_parameters\" : {     \"VmSize\" : \"Standard_D64_v3\"   } } ```
    #[serde(rename = "backend_parameters", skip_serializing_if = "Option::is_none")]
    pub backend_parameters: Option<std::collections::HashMap<String, String>>,
    /// If set to true, backends should fail the task if any backend_parameters key/values are unsupported, otherwise, backends should attempt to run the task
    #[serde(rename = "backend_parameters_strict", skip_serializing_if = "Option::is_none")]
    pub backend_parameters_strict: Option<bool>,
}

impl TesResources {
    /// Resources describes the resources requested by a task.
    pub fn new() -> TesResources {
        TesResources {
            cpu_cores: None,
            preemptible: None,
            ram_gb: None,
            disk_gb: None,
            zones: None,
            backend_parameters: None,
            backend_parameters_strict: None,
        }
    }
}

