use ga4gh_sdk::clients::tes::models::ListTasksParams;
use ga4gh_sdk::clients::tes::models::TesTask;
use ga4gh_sdk::clients::tes::{Task, TES};
use ga4gh_sdk::utils::configuration::Configuration;
use ga4gh_sdk::utils::test_utils::ensure_funnel_running;
use ga4gh_sdk::utils::transport::Transport;
use ga4gh_sdk::utils::configuration::BasicAuth;
use clap::{arg, Command};
use std::fs;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use std::path::Path;
use url::Url;
use std::error::Error;
use log::{debug, error};

/// # Examples
///
/// To run the `create` command:
///
/// ```sh
/// ga4gh-cli tes create '{
///     "name": "Hello world",
///     "inputs": [{
///         "url": "s3://funnel-bucket/hello.txt",
///         "path": "/inputs/hello.txt"
///     }],
///     "outputs": [{
///         "url": "s3://funnel-bucket/output.txt",
///         "path": "/outputs/stdout"
///     }],
///     "executors": [{
///         "image": "alpine",
///         "command": ["cat", "/inputs/hello.txt"],
///         "stdout": "/outputs/stdout"
///     }]
/// }'
/// ```
///
/// Or:
///
/// ```sh
/// ga4gh-cli tes create ./tests/sample.tes
/// ```
///
/// To run the `list` command:
///
/// ```sh
/// ga4gh-cli tes list --name_prefix None --state None --tag_key None --tag_value None --page_size None --page_token None --view FULL'
/// ```
/// OR
/// Parameters with None values can be avoided, like:
/// ```sh
/// ga4gh-cli tes list --view FULL
/// ```
///
/// ASSUME, cqgk5lj93m0311u6p530 is the id of a task created before
/// To run the `get` command:
///
/// ```sh
/// ga4gh-cli tes get cqgk5lj93m0311u6p530 BASIC
/// ```
///
/// To run the `status` command:
///
/// ```sh
/// ga4gh-cli tes status cqgk5lj93m0311u6p530      
/// ```
///
///
/// To run the `cancel` command:
///
/// ```sh
/// ga4gh-cli tes cancel cqgk5lj93m0311u6p530      
/// ```

use ga4gh_sdk::clients::tes::models::TesListTasksResponse;
use ga4gh_sdk::clients::tes::models::TesState;

fn tes_state_to_str(state: &Option<TesState>) -> &str {
    match state {
        Some(TesState::Unknown) => "Unknown",
        Some(TesState::Queued) => "Queued",
        Some(TesState::Initializing) => "Initializing",
        Some(TesState::Running) => "Running",
        Some(TesState::Paused) => "Paused",
        Some(TesState::Complete) => "Complete",
        Some(TesState::ExecutorError) => "Executor Error",
        Some(TesState::SystemError) => "System Error",
        Some(TesState::Canceled) => "Canceled",
        Some(TesState::Canceling) => "Canceling",
        Some(TesState::Preempted) => "Preempted",
        None => "None",
    }
}

fn format_task(task: &TesTask) -> String {
    format!(
        "{:<25} {:<15}\n",
        task.id.as_deref().unwrap_or("None"),
        tes_state_to_str(&task.state)
    )
}

fn format_tasks_response(response: &TesListTasksResponse) -> String {
    let mut table = String::new();
    let headers = format!("{:<25} {:<15}\n", "TASK ID", "State");
    table.push_str(&headers);
    for task in &response.tasks {
        table.push_str(&format_task(task));
    }
    table
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cmd = Command::new("cli")
        .bin_name("cli")
        .version("0.1.0")
        .about("CLI to manage tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("tes")
                .about("TES subcommands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a task")
                        .arg(arg!(<TASK_FILE> "The task file to create"))
                        // .arg(arg!(--url <URL> "The URL for the task"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("list")
                    .about("list all tasks")
                        .arg(arg!(-n --name_prefix [NAME_PREFIX] "The name prefix to filter tasks"))
                        .arg(arg!(-s --state [STATE] "The state to filter tasks"))
                        .arg(arg!(-k --tag_key [TAG_KEY] "The tag key to filter tasks"))
                        .arg(arg!(-v --tag_value [TAG_VALUE] "The tag value to filter tasks"))
                        .arg(arg!(-p --page_size [PAGE_SIZE] "The page size for pagination"))
                        .arg(arg!(-t --page_token [PAGE_TOKEN] "The page token for pagination"))
                        .arg(arg!(-w --view [VIEW] "The view for the tasks"))
                )
                .subcommand(
                    Command::new("get")
                        .about("get task data")
                        .arg(arg!(<id> "The id of the task which should be returned"))
                        .arg(arg!(<view> "The view in which the task should be returned"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("status")
                        .about("get status of the task")
                        .arg(arg!(<id> "The id of the task which should be returned"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("cancel")
                        .about("cancel the task")
                        .arg(arg!(<id> "The id of the task which should be cancel"))
                        .arg_required_else_help(true),
                ),
        );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("tes", sub)) => {
            if let Some(("create", sub)) = sub.subcommand() {
                let task_file = sub.value_of("TASK_FILE")
                    .ok_or_else(|| anyhow::anyhow!("TASK_FILE argument is required"))?;
                let path = Path::new(task_file);
                if !path.exists() {
                    eprintln!("File does not exist: {:?}", path);
                    std::process::exit(1);
                }
                let task_json = match fs::read_to_string(path) {
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                        task_file.to_string()
                    },
                };
                let testask: TesTask = serde_json::from_str(&task_json)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                let config = load_cli_configuration().await;
                match TES::new(&config).await {
                        Ok(tes) => {
                            let task = tes.create(testask).await;
                            println!("{:?}", task);
                        },
                        Err(e) => {
                            error!("Error creating TES instance: {:?}", e);
                            return Err(e);
                        }
                    };
                }
            if let Some(("list", sub)) = sub.subcommand() {
                let name_prefix = sub.value_of("name_prefix").map(|s| s.to_string());
                let state = sub.value_of("state").map(|s| serde_json::from_str(s).expect("Invalid state"));
                let _tag_key = sub.value_of("tag_key").map(|s| s.to_string());
                let _tag_value = sub.value_of("tag_value").map(|s| s.to_string());
                let page_size = sub.value_of("page_size").map(|s| s.parse().expect("Invalid page_size"));
                let page_token = sub.value_of("page_token").map(|s| s.to_string());
                let view = sub.value_of("view").map(|s| s.to_string());

                let parameters = ListTasksParams {
                    name_prefix,
                    state,
                    tag_key: None, // Example does not cover parsing Vec<String>
                    tag_value: None, // Example does not cover parsing Vec<String>
                    page_size,
                    page_token,
                    view,
                };

                debug!("parameters are: {:?}", parameters);
                let config = load_cli_configuration().await;
                match TES::new(&config).await {
                    Ok(tes) => {
                        match tes.list_tasks(Some(parameters)).await {
                            Ok(task_response) => {
                                println!("{}", format_tasks_response(&task_response)); 
                            },
                            Err(e) => {
                                eprintln!("Error listing tasks: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }

            if let Some(("get", sub)) = sub.subcommand() {    
                let id = sub.value_of("id").unwrap();
                let view = sub.value_of("view").unwrap();
                let config = load_cli_configuration().await;
                match TES::new(&config).await {
                    Ok(tes) => {
                        let task = tes.get(view, id).await;
                        println!("{:?}", task);
                    },
                    Err(e) => {
                        error!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }

            if let Some(("status", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                
                let config = load_cli_configuration().await;
                let transport = Transport::new(&config);
                let task = Task::new(id, transport);
                match task.status().await {
                    Ok(status) => {
                        println!("TASKID: {}", id.clone());
                        println!("STATUS: {:?}", status);
                    },
                    Err(e) => {
                        error!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
            if let Some(("cancel", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                let config = load_cli_configuration().await;
                let transport = Transport::new(&config);
                let task = Task::new(id, transport);
                match task.cancel().await {
                    Ok(output) => {
                        println!("STATUS: {:?}", output);
                    }
                    Err(e) => {
                        error!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
        }
        
        _ => {
            eprintln!("Error: Unrecognized command or option");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Loads the configuration from a JSON file.
///
/// # Example `config.json`
///
/// ```json
/// {
///   "base_path": "http://localhost:8000",
///   "user_agent": "Some(User)",
///   "basic_auth": {
///         "username": "your_username",
///         "password": "your_password"
///     },
///   "oauth_access_token": "your_oauth_access_token"
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the configuration file is missing or malformed.

fn read_configuration_from_file(file_path: &str) -> Result<Configuration, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let json_value: Value = serde_json::from_str(&contents)?;

    let base_path = json_value["base_path"].as_str().unwrap_or_default().to_string();
    let user_agent = json_value["user_agent"].as_str().map(|s| s.to_string());
    let basic_auth = json_value["basic_auth"].as_object().map(|auth| BasicAuth {
            username: auth["username"].as_str().unwrap_or_default().to_string(),
            password: Some(auth["password"].as_str().unwrap_or_default().to_string()),
        });
    let oauth_access_token = json_value["oauth_access_token"].as_str().map(|s| s.to_string());

    let config = Configuration::new(base_path, user_agent, basic_auth, oauth_access_token);
    Ok(config)
}

fn load_configuration() -> Configuration {
    let config_file_path = dirs::home_dir()?.join(".config/config.json");
    if config_file_path.exists() {
        read_configuration_from_file(config_file_path.to_str()?).unwrap_or_default()
    } else {
        Configuration::default()
    }
}

async fn load_cli_configuration()-> Configuration{
    let mut config = load_configuration();
    if config.base_path.as_str() == "localhost" {
        let funnel_url = ensure_funnel_running().await;
        let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
        config.set_base_path(funnel_url);
    }
    config
}