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
use crate::tes::models;
use serde::{Deserialize, Serialize};

/// TesFileType : Define if input/output element is a file or a directory. It is not required that the user provide this value, but it is required that the server fill in the value once the information is avalible at run time.
/// Define if input/output element is a file or a directory. It is not required that the user provide this value, but it is required that the server fill in the value once the information is avalible at run time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TesFileType {
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "DIRECTORY")]
    Directory,

}

impl std::fmt::Display for TesFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "FILE"),
            Self::Directory => write!(f, "DIRECTORY"),
        }
    }
}

impl Default for TesFileType {
    fn default() -> TesFileType {
        Self::File
    }
}

