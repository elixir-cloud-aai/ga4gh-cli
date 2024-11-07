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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WesRunId {
    /// workflow run ID
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

impl WesRunId {
    pub fn new() -> WesRunId {
        WesRunId { run_id: None }
    }
}
