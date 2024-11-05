/*
 * Workflow Execution Service
 *
 * ## Executive Summary The Workflow Execution Service (WES) API provides a standard way for users to submit workflow requests to workflow execution systems, and to monitor their execution. This API lets users run a single workflow (currently [**CWL**](https://www.commonwl.org/) or [**WDL**](http://www.openwdl.org/) formatted workflows, other types may be supported in the future) on multiple different platforms, clouds, and environments. Key features of the API: - can request that a workflow be run - can pass parameters to that workflow (e.g. input files, cmdline arguments) - can get information about running workflows (e.g. status, errors, output file locations) - can cancel a running workflow ## Introduction This document describes the WES API and provides details on the specific endpoints, request formats, and response.  It is intended to provide key information for developers of WES-compatible services as well as clients that will call these WES services. Use cases include: - \"Bring your code to the data\": a researcher who has built their own custom analysis can submit it to run on a dataset owned by an external organization, instead of having to make a copy of the data - Best-practices pipelines: a researcher who maintains their own controlled data environment can find useful workflows in a shared directory (e.g. Dockstore.org), and run them over their data ## Standards The WES API specification is written in OpenAPI and embodies a RESTful service philosophy.  It uses JSON in requests and responses and standard HTTP/HTTPS for information transport. ## Authorization and Authentication Users must supply credentials that establish their identity and authorization in order to use a WES endpoint. We recommend that WES implementations use an OAuth2 [**bearer token**](https://oauth.net/2/bearer-tokens/), although they can choose other mechanisms if appropriate. WES callers can use the `auth_instructions_url` from the [**`service-info` endpoint**](https://ga4gh.github.io/workflow-execution-service-schemas/#/WorkflowExecutionService/GetServiceInfo) to learn how to obtain and use a bearer token for a particular implementation. <br><br> The WES implementation is responsible for checking that a user is authorized to submit workflow run requests. The particular authorization policy is up to the WES implementer. <br><br> Systems like WES need to also address the ability to pass credentials with jobs for input and output access.  In the current version of WES, the passing of credentials to authenticate and authorize access to inputs and outputs, as well as mandates about necessary file transfer protocols to support, are out of scope.  However, parallel work on the Data Object Service is addressing ways to pass around access credentials with data object references, opening up the possibility that a future version of WES will provide concrete mechanisms for workflow runs to access data using credentials different than those used for WES.  This is a work in progress and support of DOS in WES will be added in a future release of WES. 
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

#![allow(unused_imports)]
#![allow(clippy::empty_docs)]
use crate::clients::wes::models;
use serde::{Deserialize, Serialize};

/// RunRequest : To execute a workflow, send a run request including all the details needed to begin downloading and executing a given workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunRequest {
    /// REQUIRED The workflow run parameterizations (JSON encoded), including input and output file locations
    #[serde(rename = "workflow_params", skip_serializing_if = "Option::is_none")]
    pub workflow_params: Option<serde_json::Value>,
    /// REQUIRED The workflow descriptor type, must be \"CWL\" or \"WDL\" currently (or another alternative supported by this WES instance)
    #[serde(rename = "workflow_type", skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<String>,
    /// REQUIRED The workflow descriptor type version, must be one supported by this WES instance
    #[serde(rename = "workflow_type_version", skip_serializing_if = "Option::is_none")]
    pub workflow_type_version: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workflow_engine_parameters", skip_serializing_if = "Option::is_none")]
    pub workflow_engine_parameters: Option<std::collections::HashMap<String, String>>,
    /// REQUIRED The workflow CWL or WDL document. When `workflow_attachments` is used to attach files, the `workflow_url` may be a relative path to one of the attachments.
    #[serde(rename = "workflow_url", skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}

impl RunRequest {
    /// To execute a workflow, send a run request including all the details needed to begin downloading and executing a given workflow.
    pub fn new() -> RunRequest {
        RunRequest {
            workflow_params: None,
            workflow_type: None,
            workflow_type_version: None,
            tags: None,
            workflow_engine_parameters: None,
            workflow_url: None,
        }
    }
}

