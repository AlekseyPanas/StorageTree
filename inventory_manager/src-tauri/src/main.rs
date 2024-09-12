// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;
use crate::repository::{GoalCompletionStatus, IRepo, TaskbasedGoal, TimebasedGoal, TaskbasedRecurrence, TimebasedRecurrence, InMemoryRepo, TimebasedCriteria, Goal, TaskbasedCriteriaItem};

mod repository;

pub struct RepoWrap(pub Mutex<InMemoryRepo>);

// https://github.com/tauri-apps/tauri/discussions/1336

#[tauri::command]
fn get_all_timebased_goals(state: State<RepoWrap>) -> Vec<TimebasedGoal> {
    let mut state_guard = state.0.lock().unwrap();
    state_guard.get_timebased_goals(0, 0, &[])
}

#[tauri::command]
fn get_all_taskbased_goals(state: State<RepoWrap>) -> Vec<TaskbasedGoal> {
    let mut state_guard = state.0.lock().unwrap();
    state_guard.get_taskbased_goals(0, 0, &[])
}

#[tauri::command]
fn get_all_timebased_recurrences(state: State<RepoWrap>) -> Vec<TimebasedRecurrence> {
    let mut state_guard = state.0.lock().unwrap();
    state_guard.get_timebased_recurrences(0, 0)
}

#[tauri::command]
fn get_all_taskbased_recurrences(state: State<RepoWrap>) -> Vec<TaskbasedRecurrence> {
    let mut state_guard = state.0.lock().unwrap();
    state_guard.get_taskbased_recurrences(0, 0)
}

fn main() {
    let mut repo = InMemoryRepo::new();

    repo.create_edit_timebased_goal(TimebasedGoal {
        goal: Goal {
            parent_id: 0,
            recurrence_id: 0,
            goal_id: 0,
            goal_name: "Do Homework".to_string(),
            start_unix_timestamp: 200,
            end_unix_timestamp: 400,
            failure_callback: vec!["Stab yourself".to_string(), "10 pullups".to_string()],
            success_callback: vec!["Fly".to_string(), "Laugh".to_string()],
            finally_callback: vec!["Exist".to_string(), "Exist some more".to_string()],
            completion_status: GoalCompletionStatus::Incomplete

        },
        criteria: TimebasedCriteria {
            time_ms: 10000,
            link_id: 0,
            task: "Cooking that sauce".to_string(),
            feed: false,
            dedicated_time_ms: 0
        }
    });

    repo.create_edit_taskbased_goal(TaskbasedGoal {
        goal: Goal {
            parent_id: 0,
            recurrence_id: 0,
            goal_id: 0,
            goal_name: "Finish Task".to_string(),
            start_unix_timestamp: 700,
            end_unix_timestamp: 900,
            failure_callback: vec!["Killllll".to_string(), "2 squats".to_string()],
            success_callback: vec!["LIVE".to_string()],
            finally_callback: vec!["WALK".to_string()],
            completion_status: GoalCompletionStatus::Incomplete

        },
        criteria: vec![
            TaskbasedCriteriaItem {
                description: "Start task".to_string(),
                link_id: 0,
                is_checked: false
            },
            TaskbasedCriteriaItem {
                description: "Work on task".to_string(),
                link_id: 0,
                is_checked: false
            },
            TaskbasedCriteriaItem {
                description: "Finish task".to_string(),
                link_id: 0,
                is_checked: false
            }
        ]
    });

    tauri::Builder::default()
        .manage(RepoWrap(Mutex::new(repo)))
        .invoke_handler(tauri::generate_handler![get_all_timebased_goals, get_all_taskbased_goals, get_all_timebased_recurrences, get_all_taskbased_recurrences])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
Frontend Plan:

fn fetch_goal_data() -> Goal[]
    - Fetch all goals from DB

fn fetch_recurrences() -> Recurrence[]
    - Fetch all recurrences from DB

fn build_timeline_structure(Goal[], Recurrence[]) -> GoalBlock[]
    - Converts goals and recurrences into the timetable structure
    - More precisely, determines the row each goal and recurrence lives on
    - Saves this data to timeline_structure

react state: timeline_structure
react state: edit_recurrence_dat
react state: edit_goal_dat
react state: success_list
react state: failure_list
react state: recurrence_context
react state: goal_context

App:
    Timeline View:  // Interactive timeline component containing all goal blocks, and functionality
        // Dragging or scrolling edits some state which controls timeline "camera" window
        // On receive click from block, make it selected
        // On receive appropriate drag event, modify timeline_structure accordingly and push goal change to DB
        foreach in timeline_structure:
            Block:
                // A single goal block. Customizable to display blocks for everything that will appear on the timeline
                // Clicking it calls parent function from Timeline view
                // Dragging an edge or on body of block will trigger appropriate callbacks in parent
    Edit Recurrence:
        // Modal to edit or make a new recurrence. Invisible by default. Filled from edit_recurrence_dat
        // Upon clicking save, calls a function passed from App. App knows what to do based on the context in which the modal was opened
    Edit Goal:  // Same as Edit Recurrence but with edit_goal_dat
    Success,Failure,Edit Modal
    Skip these goals, edit recurrence, create goals Modal
    Failure List Modal:  // Reads from failure_list
    Success List Modal:  // Reads from success_list
    Recurrence context menu:  // Reads position and is_enabled from recurrence_context
    Goal context menu:  // same but with goal_context


Calming Notion of Goal Resolution:
    - Goals exist as some piece of data with some timebound and some completion status
    - Notice how this is completely irrespective of what time it is now
    - At any time you may resolve a goal, execute its callbacks, and this may add, move, or change things in the timeline
    - Again, notice how the current time doesn't matter
    - Now on the app level, resolving simply means searching for unresolved expired goals, executing their callbacks, changing
    their state to resolved, checking again for expired goals, and repeating this until no expired goals remain

    - Recurrences are functions that spawn new goals and track goals that have already been spawned to know which next ones to spawn
    - Note how this is irrespective of current time
    - You can tell a recurrence to spawn goals up to a certain time
    - This is irrespective of current time and can be called anytime with any value
*/
