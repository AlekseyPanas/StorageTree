// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;
// use crate::repository::{GoalCompletionStatus, IRepo, TaskbasedGoal, TimebasedGoal, TaskbasedRecurrence, TimebasedRecurrence, InMemoryRepo, TimebasedCriteria, Goal, TaskbasedCriteriaItem};
//
// mod repository;
//
// pub struct RepoWrap(pub Mutex<InMemoryRepo>);
//
// // https://github.com/tauri-apps/tauri/discussions/1336
//
// #[tauri::command]
// fn get_all_timebased_goals(state: State<RepoWrap>) -> Vec<TimebasedGoal> {
//     let mut state_guard = state.0.lock().unwrap();
//     state_guard.get_timebased_goals(0, 0, &[])
// }
//
// #[tauri::command]
// fn get_all_taskbased_goals(state: State<RepoWrap>) -> Vec<TaskbasedGoal> {
//     let mut state_guard = state.0.lock().unwrap();
//     state_guard.get_taskbased_goals(0, 0, &[])
// }
//
// #[tauri::command]
// fn get_all_timebased_recurrences(state: State<RepoWrap>) -> Vec<TimebasedRecurrence> {
//     let mut state_guard = state.0.lock().unwrap();
//     state_guard.get_timebased_recurrences(0, 0)
// }
//
// #[tauri::command]
// fn get_all_taskbased_recurrences(state: State<RepoWrap>) -> Vec<TaskbasedRecurrence> {
//     let mut state_guard = state.0.lock().unwrap();
//     state_guard.get_taskbased_recurrences(0, 0)
// }
//
fn main() {
//     let mut repo = InMemoryRepo::new();
//
//     repo.create_edit_timebased_goal(TimebasedGoal {
//         goal: Goal {
//             parent_id: 0,
//             recurrence_id: 0,
//             goal_id: 0,
//             goal_name: "Do Homework".to_string(),
//             start_unix_timestamp: 200,
//             end_unix_timestamp: 400,
//             failure_callback: vec!["Stab yourself".to_string(), "10 pullups".to_string()],
//             success_callback: vec!["Fly".to_string(), "Laugh".to_string()],
//             finally_callback: vec!["Exist".to_string(), "Exist some more".to_string()],
//             completion_status: GoalCompletionStatus::Incomplete
//
//         },
//         criteria: TimebasedCriteria {
//             time_ms: 10000,
//             link_id: 0,
//             task: "Cooking that sauce".to_string(),
//             feed: false,
//             dedicated_time_ms: 0
//         }
//     });
//
//     repo.create_edit_taskbased_goal(TaskbasedGoal {
//         goal: Goal {
//             parent_id: 0,
//             recurrence_id: 0,
//             goal_id: 0,
//             goal_name: "Finish Task".to_string(),
//             start_unix_timestamp: 700,
//             end_unix_timestamp: 900,
//             failure_callback: vec!["Killllll".to_string(), "2 squats".to_string()],
//             success_callback: vec!["LIVE".to_string()],
//             finally_callback: vec!["WALK".to_string()],
//             completion_status: GoalCompletionStatus::Incomplete
//
//         },
//         criteria: vec![
//             TaskbasedCriteriaItem {
//                 description: "Start task".to_string(),
//                 link_id: 0,
//                 is_checked: false
//             },
//             TaskbasedCriteriaItem {
//                 description: "Work on task".to_string(),
//                 link_id: 0,
//                 is_checked: false
//             },
//             TaskbasedCriteriaItem {
//                 description: "Finish task".to_string(),
//                 link_id: 0,
//                 is_checked: false
//             }
//         ]
//     });

    tauri::Builder::default()
//         .manage(RepoWrap(Mutex::new(repo)))
//         .invoke_handler(tauri::generate_handler![get_all_timebased_goals, get_all_taskbased_goals, get_all_timebased_recurrences, get_all_taskbased_recurrences])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


