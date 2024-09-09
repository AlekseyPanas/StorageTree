#[derive(PartialEq, Eq)]
enum CreateGoalCode {
    Success,
    FailureSubgoalOutsideParentTimebound,
    FailureNewTimeboundSmallerThanSubgoals,
    FailureEditingGoalOfIncorrectType,
    FailureEditingGoalDoesntExist
}

#[derive(PartialEq, Eq)]
enum GoalDeathCode {
    Success,
    FailureSubgoalsNotAllDead,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
enum GetGoalCode {
    Success,
    FailureGoalIncorrectType,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
enum GoalDeleteCode {
    Success,
    FailureGoalHasSubgoals,
    FailureGoalDoesntExist
}

#[derive(PartialEq, Eq)]
enum CheckUncheckTaskbasedCriteriaCode {
    SuccessCriteriaToggled,
    SuccessCriteriaAlreadyInThisState,
    FailureCriteriaIndexOutOfBounds,
    FailureGoalDoesntExist,
    FailureGoalIsTimebased
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum GoalCompletionStatus {
    Incomplete,
    Succeeded,
    Failed,
    Deleted
}

///
struct Goal<'goal> {
    parent_id: u128,  // 0 for no parent
    goal_id: u128,  // 0 if not yet defined
    start_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    end_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    failure_callback: &'goal mut[String],
    success_callback: &'goal mut[String],
    finally_callback: &'goal mut[String],
    completion_status: GoalCompletionStatus
}

///
struct TimebasedCriteria {
    time_ms: u128,
    link_id: u128,  // 0 if custom task is present
    task: String,  // Only relevant if link_id is 0
    feed: bool,  // Only relevant is link_id is NOT 0
    dedicated_time_ms: u128  // time dedicated to this timebased goal so far
}

///
struct TimebasedGoal<'tibg> {
    goal: &'tibg mut Goal<'tibg>,
    criteria: &'tibg mut TimebasedCriteria
}

///
struct TaskbasedCriteriaItem {
    description: String,
    link_id: u128  // 0 if unlinked
}

///
struct TaskbasedGoal<'tbg> {
    goal: &'tbg mut Goal<'tbg>,
    criteria: &'tbg mut[&'tbg mut TaskbasedCriteriaItem],
    checked_indexes: Vec<usize>  // Indexes corresponding to criteria whose checkbox has been checked
}

///
struct Recurrence {
    recurrence_id: u128,
    start_unix_timestamp: u128,
    end_unix_timestamp: u128,  // 0 if indefinite
    spawn_interval_ms: u128,
    goal_duration_ms: u128
}

///
struct TimebasedRecurrence<'tibr> {
    timebased_goal: &'tibr mut TimebasedGoal<'tibr>,
    recurrence: Recurrence
}

///
struct TaskbasedRecurrence<'tbr> {
    taskbased_goal: &'tbr mut TimebasedGoal<'tbr>,
    recurrence: Recurrence
}

///
trait IRepo {
    ///
    fn create_edit_timebased_goal(&mut self, goal_dat: &'static mut TimebasedGoal) -> CreateGoalCode;

    ///
    fn create_edit_taskbased_goal(&mut self, goal_dat: &'static mut TaskbasedGoal) -> CreateGoalCode;

    ///
    fn create_edit_timebased_recurrence(&mut self, recurrence_dat: &'static mut TimebasedRecurrence) -> bool;

    ///
    fn create_edit_taskbased_recurrence(&mut self, recurrence_dat: &'static mut TaskbasedRecurrence) -> bool;

    ///
    fn delete_goal(&mut self, goal_id: u128) -> GoalDeleteCode;

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
    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> &(GetGoalCode, Option<TaskbasedGoal>);

    ///
    fn get_goal_by_id(&self, goal_id: u128) -> &(GetGoalCode, Option<Goal>);

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[TimebasedGoal];

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[TaskbasedGoal];

    /// Include only goals that have a completion status within filter. If filter is an empty array then include all
    /// Include only goals which intersect with the provided start, end interval. 0 for both start and end will return all goals
    fn get_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[Goal];

    ///
    fn is_timebased(&self, goal_id: u128) -> bool;

    /// Generates goals from the recurrence up to and including the first goal whose start time would be after cur_time
    fn generate_goals_from_recurrence(&self, recurrence_id: u128, cur_time: u128) -> bool;

    /// Get subgoals of the given goal at a depth of one. This means subgoals of subgoals are not included
    /// Include only subgoals that have a completion status within filter. If filter is an empty array then include all
    fn get_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> &[Goal];

    ///
    fn does_goal_exist(&self, goal_id: u128) -> bool;

    /// Get the number subgoals of the given goal at a depth of one. This means subgoals of subgoals are not included.
    /// Include only subgoals that have a completion status within filter. If filter is an empty array then include all
    fn get_num_immediate_subgoals(&self, filter: &[GoalCompletionStatus]) -> usize;
}


struct InMemoryRepo<'imr> {
    next_free_id: u128,
    timebased_recurrences: Vec<&'imr mut TimebasedRecurrence<'imr>>,
    taskbased_recurrences: Vec<&'imr mut TaskbasedRecurrence<'imr>>,
    timebased_goals: Vec<&'imr mut TimebasedGoal<'imr>>,
    taskbased_goals: Vec<&'imr mut TaskbasedGoal<'imr>>
}

impl InMemoryRepo<'static> {
    fn __get_next_free_id(&mut self) -> u128 {
        self.next_free_id += 1;
        self.next_free_id
    }

    fn __is_within_bounds(outer_start: u128, outer_end: u128, inner_start: u128, inner_end: u128) -> bool {
        inner_start >= outer_start && inner_start <= outer_end
    }

    fn __create_edit_helper(&self, goal_dat: &Goal) -> (bool, CreateGoalCode) {
        // Check parent bounds
        if goal_dat.parent_id != 0 {
            let (code, goal) = self.get_goal_by_id(goal_dat.parent_id);
            if !InMemoryRepo::__is_within_bounds(goal.unwrap().start_unix_timestamp, goal.unwrap().end_unix_timestamp,
                                                  goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp) {
                return (false, CreateGoalCode::FailureSubgoalOutsideParentTimebound);
            }
        }

        // Edit mode
        if goal_dat.goal_id != 0 {

            // Check that the goal exists
            if !self.does_goal_exist(goal_dat.goal_id) {
                return (false, CreateGoalCode::FailureEditingGoalDoesntExist);
            }

            // Check that new timebound doesn't put any subgoals outside
            let subgoals = self.get_immediate_subgoals(goal_dat.goal_id, &[GoalCompletionStatus::Incomplete]);
            for subgoal in subgoals {
                if !InMemoryRepo::__is_within_bounds(
                    goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp,
                    subgoal.start_unix_timestamp, subgoal.end_unix_timestamp
                ) {
                    return (false, CreateGoalCode::FailureNewTimeboundSmallerThanSubgoals);
                }
            }

            // Check that the correct goal type is being edited
            if !self.is_timebased(goal_dat.goal_id) {
                return (false, CreateGoalCode::FailureEditingGoalOfIncorrectType);
            }

            return (true, CreateGoalCode::Success);
        }

        // Create mode
        else {
            return (false, CreateGoalCode::Success);
        }
    }

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

    fn __set_goal_status(&mut self, goal_id: u128, status: GoalCompletionStatus) {
        if self.is_timebased(goal_id) {
            let (_, idx) = self.__get_index_of_goal(true, goal_id);
            self.timebased_goals[idx].goal.completion_status = status;
        } else {
            let (_, idx) = self.__get_index_of_goal(false, goal_id);
            self.taskbased_goals[idx].goal.completion_status = status;
        }
    }
}

impl IRepo for InMemoryRepo<'static> {
    fn create_edit_timebased_goal(&mut self, goal_dat: &'static mut TimebasedGoal) -> CreateGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if code == CreateGoalCode::Success {
            if is_edit {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if self.timebased_goals[i].goal.goal_id == goal_dat.goal.goal_id {
                        self.timebased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.timebased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.timebased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback;
                        self.timebased_goals[i].goal.success_callback = goal_dat.goal.success_callback;
                        self.timebased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback;
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
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_taskbased_goal(&mut self, goal_dat: &'static mut TaskbasedGoal) -> CreateGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if code == CreateGoalCode::Success {
            if is_edit {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if self.taskbased_goals[i].goal.goal_id == goal_dat.goal.goal_id {
                        self.taskbased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.taskbased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.taskbased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback;
                        self.taskbased_goals[i].goal.success_callback = goal_dat.goal.success_callback;
                        self.taskbased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback;
                        self.taskbased_goals[i].criteria = goal_dat.criteria;
                        self.taskbased_goals[i].checked_indexes = self.taskbased_goals[i].checked_indexes.
                            iter().
                            filter(|&&x| x < self.taskbased_goals[i].criteria.len())
                            .cloned()
                            .collect();
                        break;
                    }
                }
                return code;
            } else {
                self.taskbased_goals.push(goal_dat);
                let last_idx = self.taskbased_goals.len() - 1;
                self.taskbased_goals[last_idx].goal.goal_id = self.__get_next_free_id();
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_timebased_recurrence(&mut self, recurrence_dat: &'static mut TimebasedRecurrence) -> bool {
        self.timebased_recurrences.push(recurrence_dat);
        return true;
    }

    fn create_edit_taskbased_recurrence(&mut self, recurrence_dat: &'static mut TaskbasedRecurrence) -> bool {
        self.taskbased_recurrences.push(recurrence_dat);
        return true;
    }

    fn delete_goal(&mut self, goal_id: u128) -> GoalDeleteCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeleteCode::FailureGoalDoesntExist; }
        // Check that goal has no incomplete subgoals
        if self.get_num_immediate_subgoals(&[GoalCompletionStatus::Incomplete]) != 0 { return GoalDeleteCode::FailureGoalHasSubgoals; }

        self.__set_goal_status(goal_id, GoalCompletionStatus::Deleted);
        return GoalDeleteCode::Success;
    }

    fn feed_timebased_goal(&mut self, goal_id: u128, time_to_add_ms: u128) -> bool {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return false; }
        // Check that goal is timebased
        if !self.is_timebased(goal_id) { return false; }

        // Log time
        let (_, idx) = self.__get_index_of_goal(true, goal_id);
        self.timebased_goals[idx].criteria.dedicated_time_ms += time_to_add_ms;

        return true;
    }

    fn uncheck_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist; }
        // Check that goal is taskbased
        if self.is_timebased(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalIsTimebased; }

        let (_, idx) = self.__get_index_of_goal(false, goal_id);

        // Check if criteria index is within bounds of criteria list
        if criteria_index >= self.taskbased_goals[idx].criteria.len() { return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist; }

        // Uncheck the criteria at given index
        let index_of_criteria = self.taskbased_goals[idx].checked_indexes.iter().position(|&x| x == criteria_index);
        if index_of_criteria.is_some() {
            self.taskbased_goals[idx].checked_indexes.remove(index_of_criteria.unwrap());
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaToggled;
        } else {
            return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist;
        }
    }

    fn check_taskbased_criteria(&mut self, goal_id: u128, criteria_index: usize) -> CheckUncheckTaskbasedCriteriaCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalDoesntExist; }
        // Check that goal is taskbased
        if self.is_timebased(goal_id) { return CheckUncheckTaskbasedCriteriaCode::FailureGoalIsTimebased; }

        let (_, idx) = self.__get_index_of_goal(false, goal_id);
        if self.taskbased_goals[idx].checked_indexes.contains(&criteria_index) {
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaAlreadyInThisState;
        } else {
            self.taskbased_goals[idx].checked_indexes.push(criteria_index);
            return CheckUncheckTaskbasedCriteriaCode::SuccessCriteriaToggled;
        }
    }

    fn succeed_goal(&mut self, goal_id: u128) -> GoalDeathCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeathCode::FailureGoalDoesntExist; }
        // Check that all subgoals have been resolved
        if self.get_num_immediate_subgoals(&[GoalCompletionStatus::Incomplete]) > 0 { return GoalDeathCode::FailureSubgoalsNotAllDead }

        self.__set_goal_status(goal_id, GoalCompletionStatus::Succeeded);
        return GoalDeathCode::Success;
    }

    fn fail_goal(&mut self, goal_id: u128) -> GoalDeathCode {
        // Check that goal exists
        if !self.does_goal_exist(goal_id) { return GoalDeathCode::FailureGoalDoesntExist; }
        // Check that all subgoals have been resolved
        if self.get_num_immediate_subgoals(&[GoalCompletionStatus::Incomplete]) > 0 { return GoalDeathCode::FailureSubgoalsNotAllDead }

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
        if !self.is_timebased(goal_id) { return (GetGoalCode::FailureGoalIncorrectType, None); }

        let (_, idx) = self.__get_index_of_goal(true, goal_id);

        let g = self.timebased_goals[idx];
        return (GetGoalCode::Success, Some(TimebasedGoal {
            goal: &mut Goal {
                parent_id: g.goal.parent_id,
                goal_id: g.goal.goal_id,
                start_unix_timestamp: g.goal.start_unix_timestamp,
                end_unix_timestamp: g.goal.end_unix_timestamp,
                failure_callback: g.goal.failure_callback,
                success_callback: g.goal.success_callback,
                finally_callback: g.goal.finally_callback,
                completion_status: g.goal.completion_status
            },
            criteria: &mut TimebasedCriteria {
                time_ms: g.criteria.time_ms,
                link_id: g.criteria.link_id,
                task: g.criteria.task.clone(),
                feed: g.criteria.feed,
                dedicated_time_ms: g.criteria.dedicated_time_ms
            }
        }))
    }

    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> &(GetGoalCode, Option<TaskbasedGoal>) {
        todo!()
    }

    fn get_goal_by_id(&self, goal_id: u128) -> &(GetGoalCode, Option<Goal>) {
        todo!()
    }

    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[TimebasedGoal] {
        todo!()
    }

    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[TaskbasedGoal] {
        todo!()
    }

    fn get_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128, filter: &[GoalCompletionStatus]) -> &[Goal] {
        todo!()
    }

    fn is_timebased(&self, goal_id: u128) -> bool {
        todo!()
    }

    fn generate_goals_from_recurrence(&self, recurrence_id: u128, cur_time: u128) -> bool {
        todo!()
    }

    fn get_immediate_subgoals(&self, goal_id: u128, filter: &[GoalCompletionStatus]) -> &[Goal] {
        todo!()
    }

    fn does_goal_exist(&self, goal_id: u128) -> bool {
        todo!()
    }

    fn get_num_immediate_subgoals(&self, filter: &[GoalCompletionStatus]) -> usize {
        todo!()
    }
}
