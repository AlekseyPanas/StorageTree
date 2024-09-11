use std::cmp::max;

#[derive(PartialEq, Eq)]
pub enum CreateEditGoalCode {
    Success,
    FailureSubgoalOutsideParentTimebound,
    FailureNewTimeboundSmallerThanSubgoals,
    FailureEditingGoalOfIncorrectType,
    FailureEditingGoalDoesntExist
}

#[derive(PartialEq, Eq)]
pub enum CreateEditRecurrenceCode {
    Success,
    FailureEditingRecurrenceDoesntExist
}

#[derive(PartialEq, Eq)]
pub enum GoalDeathCode {
    Success,
    FailureSubgoalsNotAllDead,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
pub enum GetGoalCode {
    Success,
    FailureGoalIncorrectType,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
pub enum GoalDeleteCode {
    Success,
    FailureGoalHasSubgoals,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
pub enum CheckUncheckTaskbasedCriteriaCode {
    SuccessCriteriaToggled,
    SuccessCriteriaAlreadyInThisState,
    FailureCriteriaIndexOutOfBounds,
    FailureGoalDoesntExist,
    FailureGoalIsTimebased
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum GoalCompletionStatus {
    Incomplete,
    Succeeded,
    Failed,
    Deleted
}

///
#[derive(Clone, Debug)]
pub struct Goal {
    pub parent_id: u128,  // 0 for no parent
    pub recurrence_id: u128,  // Refers to recurrence which spawned this goal; 0 if none
    pub goal_id: u128,  // 0 if not yet defined (i.e goal being created)
    pub goal_name: String,
    pub start_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    pub end_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    pub failure_callback: Vec<String>,
    pub success_callback: Vec<String>,
    pub finally_callback: Vec<String>,
    pub completion_status: GoalCompletionStatus
}

///
#[derive(Clone)]
pub struct TimebasedCriteria {
    pub time_ms: u128,
    pub link_id: u128,  // 0 if custom task is present
    pub task: String,  // Only relevant if link_id is 0
    pub feed: bool,  // Only relevant is link_id is NOT 0
    pub dedicated_time_ms: u128  // time dedicated to this timebased goal so far
}

///
#[derive(Clone)]
pub struct TimebasedGoal {
    pub goal: Goal,
    pub criteria: TimebasedCriteria
}

///
#[derive(Clone, Debug)]
pub struct TaskbasedCriteriaItem {
    pub description: String,
    pub link_id: u128, // 0 if unlinked
    pub is_checked: bool
}

///
#[derive(Clone, Debug)]
pub struct TaskbasedGoal {
    pub goal: Goal,
    pub criteria: Vec<TaskbasedCriteriaItem>
}

///
#[derive(Clone, Debug)]
pub struct Recurrence {
    pub recurrence_id: u128,
    pub start_unix_timestamp: u128,
    pub end_unix_timestamp: u128,  // 0 if indefinite
    pub spawn_interval_ms: u128,
    pub goal_duration_ms: u128,
    pub latest_spawned_start_time_ms: u128  // The start time of the latest goal spawned by this recurrence (tracks next goal to spawn)
}

///
#[derive(Clone)]
pub struct TimebasedRecurrence {
    pub timebased_goal: TimebasedGoal,
    pub recurrence: Recurrence
}

///
#[derive(Clone, Debug)]
pub struct TaskbasedRecurrence {
    pub taskbased_goal: TaskbasedGoal,
    pub recurrence: Recurrence
}

///
pub trait IRepo {
    /// Return the version of the current state. Any mutation to state in this repository should change
    /// the version number to a new unique value
    fn get_state_version(&self) -> u128;

    ///
    fn create_edit_timebased_goal(&mut self, goal_dat: TimebasedGoal) -> CreateEditGoalCode;

    ///
    fn create_edit_taskbased_goal(&mut self, goal_dat: TaskbasedGoal) -> CreateEditGoalCode;

    ///
    fn create_edit_timebased_recurrence(&mut self, recurrence_dat: TimebasedRecurrence) -> CreateEditRecurrenceCode;

    ///
    fn create_edit_taskbased_recurrence(&mut self, recurrence_dat: TaskbasedRecurrence) -> CreateEditRecurrenceCode;

    ///
    fn delete_goal(&mut self, goal_id: u128) -> GoalDeleteCode;

    ///
    fn delete_recurrence(&mut self, recurrence_id: u128) -> bool;

    ///
    fn feed_timebased_goal(&mut self, goal_id: u128, time_to_add_ms: u128) -> bool;

    ///
    fn uncheck_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode;

    ///
    fn check_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode;

    ///
    fn succeed_goal(&mut self, goal_id: u128) -> GoalDeathCode;

    ///
    fn fail_goal(&mut self, goal_id: u128) -> GoalDeathCode;

    /// Return a list of goals with an INCOMPLETE completion status whose end time comes before the provided timestamp.
    fn get_expired_goal_ids(&self, cur_time_unix_timestamp: u128) -> Vec<u128>;

    ///
    fn get_timebased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<TimebasedGoal>);

    ///
    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<TaskbasedGoal>);

    ///
    fn get_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<Goal>);

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<TimebasedGoal>;

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<TaskbasedGoal>;

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<Goal>;

    /// Include only recurrences whose bounds intersect with the provided start, end interval. 0 for both start and end will return all
    fn get_timebased_recurrences(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> Vec<TimebasedRecurrence>;

    /// Include only recurrences whose bounds intersect with the provided start, end interval. 0 for both start and end will return all
    fn get_taskbased_recurrences(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> Vec<TaskbasedRecurrence>;

    ///
    fn is_timebased_goal(&self, goal_id: u128) -> bool;

    ///
    fn is_timebased_recurrence(&self, rec_id: u128) -> bool;

    /// Generates goals from the recurrence up to and including the first goal whose start time would be after cur_time
    fn generate_goals_from_recurrence(&mut self, recurrence_id: u128, cur_time: u128) -> bool;

    /// Get subgoals of the given goal at a depth of one. This means subgoals of subgoals are not included
    /// Include only subgoals that have a completion status within filter. If filter is an empty array then include all
    fn get_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> Vec<Goal>;

    ///
    fn does_goal_exist(&self, goal_id: u128) -> bool;

    ///
    fn does_recurrence_exist(&self, rec_id: u128) -> bool;

    /// Get the number subgoals of the given goal at a depth of one. This means subgoals of subgoals are not included.
    /// Include only subgoals that have a completion status within filter. If filter is an empty array then include all
    fn get_num_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> usize;
}


pub struct InMemoryRepo {
    next_free_id: u128,
    timebased_recurrences: Vec<TimebasedRecurrence>,
    taskbased_recurrences: Vec<TaskbasedRecurrence>,
    timebased_goals: Vec<TimebasedGoal>,
    taskbased_goals: Vec<TaskbasedGoal>,
    version_number: u128
}

impl InMemoryRepo {
    pub fn new() -> InMemoryRepo {
        InMemoryRepo {
            next_free_id: 0,
            taskbased_recurrences: Vec::new(),
            timebased_recurrences: Vec::new(),
            timebased_goals: Vec::new(),
            taskbased_goals: Vec::new(),
            version_number: 0
        }
    }

    fn __get_next_free_id(&mut self) -> u128 {
        self.next_free_id += 1;
        self.next_free_id
    }

    fn __is_within_bounds(outer_start: u128, outer_end: u128, inner_start: u128, inner_end: u128) -> bool {
        inner_start >= outer_start && inner_start <= outer_end
    }

    fn __do_bounds_intersect(bound1_start: u128, bound1_end: u128, bound2_start: u128, bound2_end: u128) -> bool {
        (bound2_start <= bound1_start && bound1_start <= bound2_end) ||
            (bound2_start <= bound1_end && bound1_end <= bound2_end) ||
            (bound1_end >= bound2_end && bound1_start <= bound2_start)
    }

    fn __create_edit_helper(&self, goal_dat: &Goal) -> (bool, CreateEditGoalCode) {
        // Check parent bounds
        if goal_dat.parent_id != 0 {
            let (code, goal) = self.get_goal_by_id(goal_dat.parent_id);
            if !InMemoryRepo::__is_within_bounds(goal.as_ref().unwrap().start_unix_timestamp, goal.as_ref().unwrap().end_unix_timestamp,
                                                  goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp) {
                return (false, CreateEditGoalCode::FailureSubgoalOutsideParentTimebound);
            }
        }

        // Edit mode
        if goal_dat.goal_id != 0 {

            // Check that the goal exists
            if !self.does_goal_exist(goal_dat.goal_id) {
                return (false, CreateEditGoalCode::FailureEditingGoalDoesntExist);
            }

            // Check that new timebound doesn't put any subgoals outside
            let subgoals = self.get_immediate_subgoals(goal_dat.goal_id, &[GoalCompletionStatus::Incomplete]);
            for subgoal in subgoals {
                if !InMemoryRepo::__is_within_bounds(
                    goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp,
                    subgoal.start_unix_timestamp, subgoal.end_unix_timestamp
                ) {
                    return (false, CreateEditGoalCode::FailureNewTimeboundSmallerThanSubgoals);
                }
            }

            // Check that the correct goal type is being edited
            if !self.is_timebased_goal(goal_dat.goal_id) {
                return (false, CreateEditGoalCode::FailureEditingGoalOfIncorrectType);
            }

            return (true, CreateEditGoalCode::Success);
        }

        // Create mode
        else {
            return (false, CreateEditGoalCode::Success);
        }
    }

    /// Return if the given goal_id was found in the list of timebased goals or taskbased goals (depending on is_timebased).
    /// If it was found, return its index in that list
    fn __get_index_of_goal(&self, is_timebased: bool, target_id: u128) -> (bool, usize) {
        if is_timebased {
            for i in 0..self.timebased_goals.len() {
                if self.timebased_goals[i].goal.goal_id == target_id { return (true, i) }
            }
            return (false, 0)
        } else {
            for i in 0..self.taskbased_goals.len() {
                if self.taskbased_goals[i].goal.goal_id == target_id { return (true, i) }
            }
            return (false, 0)
        }
    }

    /// Return if the given recurrence was found in the list of timebased goals or taskbased recurrences (depending on is_timebased).
    /// If it was found, return its index in that list
    fn __get_index_of_recurrence(&self, is_timebased: bool, target_id: u128) -> (bool, usize) {
        if is_timebased {
            for i in 0..self.timebased_recurrences.len() {
                if self.timebased_recurrences[i].recurrence.recurrence_id == target_id { return (true, i) }
            }
            return (false, 0)
        } else {
            for i in 0..self.taskbased_recurrences.len() {
                if self.taskbased_recurrences[i].recurrence.recurrence_id == target_id { return (true, i) }
            }
            return (false, 0)
        }
    }

    fn __set_goal_status(&mut self, goal_id: u128, status: GoalCompletionStatus) {
        if self.is_timebased_goal(goal_id) {
            let (_, idx) = self.__get_index_of_goal(true, goal_id);
            self.timebased_goals[idx].goal.completion_status = status;
        } else {
            let (_, idx) = self.__get_index_of_goal(false, goal_id);
            self.taskbased_goals[idx].goal.completion_status = status;
        }
    }
}

impl IRepo for InMemoryRepo {
    fn get_state_version(&self) -> u128 { self.version_number }

    fn create_edit_timebased_goal(&mut self, goal_dat: TimebasedGoal) -> CreateEditGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if code == CreateEditGoalCode::Success {
            self.version_number += 1;

            if is_edit {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if self.timebased_goals[i].goal.goal_id == goal_dat.goal.goal_id {
                        self.timebased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.timebased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.timebased_goals[i].goal.goal_name = goal_dat.goal.goal_name.clone();
                        self.timebased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback.clone();
                        self.timebased_goals[i].goal.success_callback = goal_dat.goal.success_callback.clone();
                        self.timebased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback.clone();
                        self.timebased_goals[i].criteria.feed = goal_dat.criteria.feed;
                        self.timebased_goals[i].criteria.time_ms = goal_dat.criteria.time_ms;
                        self.timebased_goals[i].criteria.task = goal_dat.criteria.task.clone();
                        self.timebased_goals[i].criteria.link_id = goal_dat.criteria.link_id;
                        self.timebased_goals[i].criteria.dedicated_time_ms = goal_dat.criteria.dedicated_time_ms;
                        break;
                    }
                }
                return code;
            } else {
                self.timebased_goals.push(goal_dat);
                let last_idx = self.timebased_goals.len() - 1;
                self.timebased_goals[last_idx].goal.goal_id = self.__get_next_free_id();
                self.timebased_goals[last_idx].goal.recurrence_id = 0;
                self.timebased_goals[last_idx].goal.completion_status = GoalCompletionStatus::Incomplete;
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_taskbased_goal(&mut self, goal_dat: TaskbasedGoal) -> CreateEditGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if code == CreateEditGoalCode::Success {
            self.version_number += 1;

            if is_edit {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if self.taskbased_goals[i].goal.goal_id == goal_dat.goal.goal_id {
                        self.taskbased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.taskbased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.taskbased_goals[i].goal.goal_name = goal_dat.goal.goal_name.clone();
                        self.taskbased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback.clone();
                        self.taskbased_goals[i].goal.success_callback = goal_dat.goal.success_callback.clone();
                        self.taskbased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback.clone();
                        self.taskbased_goals[i].criteria = goal_dat.criteria.clone();
                    }
                }
                return code;
            } else {
                self.taskbased_goals.push(goal_dat);
                let last_idx = self.taskbased_goals.len() - 1;
                self.taskbased_goals[last_idx].goal.goal_id = self.__get_next_free_id();
                self.timebased_goals[last_idx].goal.recurrence_id = 0;
                self.taskbased_goals[last_idx].goal.completion_status = GoalCompletionStatus::Incomplete;
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_timebased_recurrence(&mut self, recurrence_dat: TimebasedRecurrence) -> CreateEditRecurrenceCode {
        // Create mode
        if recurrence_dat.recurrence.recurrence_id == 0 {
            self.version_number += 1;

            self.timebased_recurrences.push(recurrence_dat);
            let last_idx = self.timebased_recurrences.len() - 1;
            self.timebased_recurrences[last_idx].recurrence.recurrence_id = self.__get_next_free_id();
            self.timebased_recurrences[last_idx].recurrence.latest_spawned_start_time_ms = self.timebased_recurrences[last_idx].recurrence.start_unix_timestamp;
        }

        // Edit mode
        else {
            let (is_found_timebased, idx_time) = self.__get_index_of_recurrence(true, recurrence_dat.recurrence.recurrence_id);
            let (is_found_taskbased, idx_task) = self.__get_index_of_recurrence(false, recurrence_dat.recurrence.recurrence_id);

            let recorded_last_start_time;
            if !is_found_taskbased && !is_found_timebased { return CreateEditRecurrenceCode::FailureEditingRecurrenceDoesntExist; }
            if is_found_taskbased {
                recorded_last_start_time = self.taskbased_recurrences[idx_time].recurrence.latest_spawned_start_time_ms;
                self.taskbased_recurrences.remove(idx_task);
            }
            else {
                recorded_last_start_time = self.timebased_recurrences[idx_time].recurrence.latest_spawned_start_time_ms;
                self.timebased_recurrences.remove(idx_time);
            }

            self.timebased_recurrences.push(recurrence_dat);
            let last_idx = self.timebased_recurrences.len() - 1;
            self.timebased_recurrences[last_idx].recurrence.latest_spawned_start_time_ms = recorded_last_start_time;

            self.version_number += 1;
        }

        return CreateEditRecurrenceCode::Success;
    }

    fn create_edit_taskbased_recurrence(&mut self, recurrence_dat: TaskbasedRecurrence) -> CreateEditRecurrenceCode {
        // Create mode
        if recurrence_dat.recurrence.recurrence_id == 0 {
            self.taskbased_recurrences.push(recurrence_dat);
            let last_idx = self.taskbased_recurrences.len() - 1;
            self.taskbased_recurrences[last_idx].recurrence.recurrence_id = self.__get_next_free_id();
            self.taskbased_recurrences[last_idx].recurrence.latest_spawned_start_time_ms = self.taskbased_recurrences[last_idx].recurrence.start_unix_timestamp;
        }

        // Edit mode
        else {
            let (is_found_timebased, idx_time) = self.__get_index_of_recurrence(true, recurrence_dat.recurrence.recurrence_id);
            let (is_found_taskbased, idx_task) = self.__get_index_of_recurrence(false, recurrence_dat.recurrence.recurrence_id);

            let recorded_last_start_time;
            if !is_found_taskbased && !is_found_timebased { return CreateEditRecurrenceCode::FailureEditingRecurrenceDoesntExist; }
            if is_found_taskbased {
                recorded_last_start_time = self.taskbased_recurrences[idx_time].recurrence.latest_spawned_start_time_ms;
                self.taskbased_recurrences.remove(idx_task);
            }
            else {
                recorded_last_start_time = self.timebased_recurrences[idx_time].recurrence.latest_spawned_start_time_ms;
                self.timebased_recurrences.remove(idx_time);
            }

            self.taskbased_recurrences.push(recurrence_dat);
            let last_idx = self.taskbased_recurrences.len() - 1;
            self.taskbased_recurrences[last_idx].recurrence.latest_spawned_start_time_ms = recorded_last_start_time;

            self.version_number += 1;
        }

        return CreateEditRecurrenceCode::Success;
    }

    fn delete_goal(&mut self, goal_id: u128) -> GoalDeleteCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeleteCode::FailureGoalDoesntExist; }
        // Check that goal has no incomplete subgoals
        if self.get_num_immediate_subgoals(goal_id, &[GoalCompletionStatus::Incomplete]) != 0 { return GoalDeleteCode::FailureGoalHasSubgoals; }

        self.version_number += 1;

        self.__set_goal_status(goal_id, GoalCompletionStatus::Deleted);
        return GoalDeleteCode::Success;
    }

    fn delete_recurrence(&mut self, recurrence_id: u128) -> bool {
        let (is_found_timebased, idx_time) = self.__get_index_of_recurrence(true, recurrence_id);
        let (is_found_taskbased, idx_task) = self.__get_index_of_recurrence(false, recurrence_id);

        if !is_found_timebased && !is_found_taskbased { return false; }

        if is_found_timebased {
            self.timebased_recurrences.remove(idx_time);
        } else {
            self.taskbased_recurrences.remove(idx_task);
        }

        self.version_number += 1;
        return true;
    }

    fn feed_timebased_goal(&mut self, goal_id: u128, time_to_add_ms: u128) -> bool {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return false; }
        // Check that goal is timebased
        if !self.is_timebased_goal(goal_id) { return false; }

        // Log time
        let (_, idx) = self.__get_index_of_goal(true, goal_id);
        self.timebased_goals[idx].criteria.dedicated_time_ms += time_to_add_ms;

        self.version_number += 1;
        return true;
    }

    fn uncheck_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist; }
        // Check that goal is taskbased
        if self.is_timebased_goal(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalIsTimebased; }

        let (_, idx) = self.__get_index_of_goal(false, goal_id);

        // Check if criteria index is within bounds of criteria list
        if criteria_index >= self.taskbased_goals[idx].criteria.len() { return CheckUncheckTaskbasedCriteriaCode::FailureCriteriaIndexOutOfBounds; }

        self.version_number += 1;

        // Uncheck the criteria at given index
        if self.taskbased_goals[idx].criteria[criteria_index].is_checked {
            self.taskbased_goals[idx].criteria[criteria_index].is_checked = false;
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaToggled;
        } else {
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaAlreadyInThisState;
        }
    }

    fn check_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist; }
        // Check that goal is taskbased
        if self.is_timebased_goal(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalIsTimebased; }

        self.version_number += 1;

        // Check the criteria at given index
        let (_, idx) = self.__get_index_of_goal(false, goal_id);
        if self.taskbased_goals[idx].criteria[criteria_index].is_checked {
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaAlreadyInThisState;
        } else {
            self.taskbased_goals[idx].criteria[criteria_index].is_checked = true;
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaToggled;
        }
    }

    fn succeed_goal(&mut self, goal_id: u128) -> GoalDeathCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeathCode::FailureGoalDoesntExist; }
        // Check that all subgoals have been resolved
        if self.get_num_immediate_subgoals(goal_id, &[GoalCompletionStatus::Incomplete]) > 0 { return GoalDeathCode::FailureSubgoalsNotAllDead }

        self.version_number += 1;

        self.__set_goal_status(goal_id, GoalCompletionStatus::Succeeded);
        return GoalDeathCode::Success;
    }

    fn fail_goal(&mut self, goal_id: u128) -> GoalDeathCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeathCode::FailureGoalDoesntExist; }
        // Check that all subgoals have been resolved
        if self.get_num_immediate_subgoals(goal_id, &[GoalCompletionStatus::Incomplete]) > 0 { return GoalDeathCode::FailureSubgoalsNotAllDead }

        self.version_number += 1;

        self.__set_goal_status(goal_id, GoalCompletionStatus::Failed);
        return GoalDeathCode::Success;
    }

    fn get_expired_goal_ids(&self, cur_time_unix_timestamp: u128) -> Vec<u128> {
        let mut v = Vec::new();
        for goal in self.get_goals(0, 0, &[]) {
            if goal.completion_status == GoalCompletionStatus::Incomplete && goal.end_unix_timestamp < cur_time_unix_timestamp {
                v.push(goal.goal_id);
            }
        }

        return v;
    }

    fn get_timebased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<TimebasedGoal>) {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return (GetGoalCode::FailureGoalDoesntExist, None); }
        // Check that goal is timebased
        if !self.is_timebased_goal(goal_id) { return (GetGoalCode::FailureGoalIncorrectType, None); }

        let (_, idx) = self.__get_index_of_goal(true, goal_id);
        return (GetGoalCode::Success, Some(self.timebased_goals[idx].clone()));
    }

    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<TaskbasedGoal>) {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return (GetGoalCode::FailureGoalDoesntExist, None); }
        // Check that goal is taskbased
        if self.is_timebased_goal(goal_id) { return (GetGoalCode::FailureGoalIncorrectType, None); }

        let (_, idx) = self.__get_index_of_goal(false, goal_id);
        return (GetGoalCode::Success, Some(self.taskbased_goals[idx].clone()));
    }

    fn get_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, Option<Goal>) {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return (GetGoalCode::FailureGoalDoesntExist, None); }

        let (timebased_success, idx_time) = self.__get_index_of_goal(true, goal_id);
        let (taskbased_success, idx_task) = self.__get_index_of_goal(false, goal_id);

        if timebased_success {
            return (GetGoalCode::Success, Some(self.timebased_goals[idx_time].goal.clone()))
        } else {
            return (GetGoalCode::Success, Some(self.taskbased_goals[idx_time].goal.clone()))
        }
    }

    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<TimebasedGoal> {
        let mut goals = Vec::new();
        for g in &self.timebased_goals {
            if ((start_unix_timestamp == 0 && end_unix_timestamp == 0) || InMemoryRepo::__do_bounds_intersect(
                g.goal.start_unix_timestamp, g.goal.end_unix_timestamp, start_unix_timestamp, end_unix_timestamp)) &&
                (filter.contains(&g.goal.completion_status) || filter.len() == 0) {

                goals.push(g.clone());
            }
        }
        return goals;
    }

    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<TaskbasedGoal> {
        let mut goals = Vec::new();
        for g in &self.taskbased_goals {
            if ((start_unix_timestamp == 0 && end_unix_timestamp == 0) || InMemoryRepo::__do_bounds_intersect(
                g.goal.start_unix_timestamp, g.goal.end_unix_timestamp, start_unix_timestamp, end_unix_timestamp)) &&
                (filter.contains(&g.goal.completion_status) || filter.len() == 0) {

                goals.push(g.clone());
            }
        }
        return goals;
    }

    fn get_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> Vec<Goal> {
        let mut goals = Vec::new();
        for g in self.get_taskbased_goals(start_unix_timestamp, end_unix_timestamp, filter) {
            goals.push(g.goal);
        }
        for g in self.get_timebased_goals(start_unix_timestamp, end_unix_timestamp, filter) {
            goals.push(g.goal);
        }
        return goals;
    }

    fn get_timebased_recurrences(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> Vec<TimebasedRecurrence> {
        let mut recs = Vec::new();
        for r in &self.timebased_recurrences {
            if (start_unix_timestamp == 0 && end_unix_timestamp == 0) || InMemoryRepo::__do_bounds_intersect(
                r.recurrence.start_unix_timestamp, r.recurrence.end_unix_timestamp, start_unix_timestamp, end_unix_timestamp) {
                recs.push(r.clone());
            }
        }
        return recs;
    }

    fn get_taskbased_recurrences(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> Vec<TaskbasedRecurrence> {
        let mut recs = Vec::new();
        for r in &self.taskbased_recurrences {
            if (start_unix_timestamp == 0 && end_unix_timestamp == 0) || InMemoryRepo::__do_bounds_intersect(
                r.recurrence.start_unix_timestamp, r.recurrence.end_unix_timestamp, start_unix_timestamp, end_unix_timestamp) {
                recs.push(r.clone());
            }
        }
        return recs;
    }

    fn is_timebased_goal(&self, goal_id: u128) -> bool {
        let (is_in_timebased, _) = self.__get_index_of_goal(true, goal_id);
        return is_in_timebased;
    }

    fn is_timebased_recurrence(&self, rec_id: u128) -> bool {
        let (is_in_timebased, _) = self.__get_index_of_recurrence(true, rec_id);
        return is_in_timebased;
    }

    fn generate_goals_from_recurrence(&mut self, rec_id: u128, cur_time: u128) -> bool {
        // Fail if rec doesnt exist
        if !self.does_recurrence_exist(rec_id) { return false; }

        self.version_number += 1;

        // Acquire recurrence data and index data
        let (is_in_timebased, idx_time) = self.__get_index_of_recurrence(true, rec_id);
        let (is_in_taskbased, idx_task) = self.__get_index_of_recurrence(true, rec_id);
        let rec;
        if is_in_taskbased { rec = self.taskbased_recurrences[idx_time].recurrence.clone(); }
        else { rec = self.timebased_recurrences[idx_time].recurrence.clone(); }

        let mut next_spawn_start_time = max(rec.latest_spawned_start_time_ms, rec.start_unix_timestamp);

        loop {
            if next_spawn_start_time > rec.end_unix_timestamp { break; }  // Never spawn goals after rec end time

            // Generate goal and record ID within recurrence
            let next_id = self.__get_next_free_id();
            if is_in_timebased {
                self.timebased_goals.push(self.timebased_recurrences[idx_time].timebased_goal.clone());
                let last_idx = self.timebased_goals.len() - 1;
                self.timebased_goals[last_idx].goal.start_unix_timestamp = next_spawn_start_time;
                self.timebased_goals[last_idx].goal.end_unix_timestamp = next_spawn_start_time + self.timebased_recurrences[idx_time].recurrence.goal_duration_ms;
                self.timebased_goals[last_idx].goal.goal_id = next_id;
                self.timebased_goals[last_idx].goal.recurrence_id = rec_id;
                self.timebased_recurrences[idx_time].recurrence.latest_spawned_start_time_ms = next_spawn_start_time;
            } else {
                self.taskbased_goals.push(self.taskbased_recurrences[idx_task].taskbased_goal.clone());
                let last_idx = self.taskbased_goals.len() - 1;
                self.taskbased_goals[last_idx].goal.start_unix_timestamp = next_spawn_start_time;
                self.taskbased_goals[last_idx].goal.end_unix_timestamp = next_spawn_start_time + self.taskbased_recurrences[idx_time].recurrence.goal_duration_ms;
                self.taskbased_goals[last_idx].goal.goal_id = next_id;
                self.taskbased_goals[last_idx].goal.recurrence_id = rec_id;
                self.taskbased_recurrences[idx_task].recurrence.latest_spawned_start_time_ms = next_spawn_start_time;
            }

            if next_spawn_start_time > cur_time { break; }  // Spawn one goal after cur_time
            next_spawn_start_time += rec.spawn_interval_ms;
        }
        return true;
    }

    fn get_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> Vec<Goal> {
        let mut goals = Vec::new();
        for g in &self.timebased_goals {
            if g.goal.parent_id == goal_id && filter.contains(&g.goal.completion_status) { goals.push(g.goal.clone()); }
        }
        for g in &self.taskbased_goals {
            if g.goal.parent_id == goal_id && filter.contains(&g.goal.completion_status) { goals.push(g.goal.clone()); }
        }
        return goals;
    }

    fn does_goal_exist(&self, goal_id: u128) -> bool {
        let (is_in_timebased, _) = self.__get_index_of_goal(true, goal_id);
        let (is_in_taskbased, _) = self.__get_index_of_goal(false, goal_id);
        return is_in_taskbased || is_in_timebased;
    }

    fn does_recurrence_exist(&self, rec_id: u128) -> bool {
        let (is_in_timebased, _) = self.__get_index_of_recurrence(true, rec_id);
        let (is_in_taskbased, _) = self.__get_index_of_recurrence(true, rec_id);
        return is_in_taskbased || is_in_timebased;
    }

    fn get_num_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> usize {
        return self.get_immediate_subgoals(goal_id, filter).len();
    }
}
