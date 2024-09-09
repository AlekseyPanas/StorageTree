// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod repository;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    println!("Hello, world!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
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

*/
