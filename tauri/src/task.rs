use maa_framework::instance::TaskParam;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::start_up::StartUpConfig, MaaZError};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TaskRunningState {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskStatus {
    pub id: Option<i64>,
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
    pub state: TaskRunningState,
}

impl From<TaskType> for TaskStatus {
    fn from(task_type: TaskType) -> Self {
        Self {
            id: None,
            task_type,
            state: TaskRunningState::Pending,
        }
    }
}

macro_rules! task_type {
    ($($variant:ident),+) => {
        #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
        pub enum TaskType {
            $($variant),+
        }

        impl TaskType {
            pub fn get_string(self) -> String {
                match self {
                    $(TaskType::$variant => stringify!($variant).to_owned()),+
                }
            }
        }

        impl TryFrom<String> for TaskType {
            type Error = MaaZError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    $(stringify!($variant) => Ok(TaskType::$variant),)+
                    _ => Err(MaaZError::UnknowTaskError(value)),
                }
            }
        }
    };
}

task_type!(StartUp);

#[derive(Serialize)]
pub struct StartUpParam {
    pub package: String,
}

impl TaskParam for StartUpParam {
    fn get_param(&self) -> String {
        let inner = json!({
            "package": self.package
        });
        json!({
            "sub_start_app": inner
        })
        .to_string()
    }
}

impl From<StartUpConfig> for StartUpParam {
    fn from(config: StartUpConfig) -> Self {
        Self {
            package: config.client_type.get_package_name(),
        }
    }
}

#[derive(Serialize)]
pub struct AwardParam;

impl TaskParam for AwardParam {
    fn get_param(&self) -> String {
        json!({}).to_string()
    }
}
