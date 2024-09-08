enum CreateGoalCode {
    Success,
    FailureSubgoalOutsideParentTimebound,
    FailureNewTimeboundSmallerThanSubgoals,
    FailureEditingGoalOfIncorrectType
}

enum GoalDeathCode {
    Success,
    FailureSubgoalsNotAllDead,
    FailureGoalStillNotExpired
}

enum GetGoalCode {
    Success,
    FailureGoalIncorrectType,
    FailureGoalDoesntExist
}

enum GoalDeleteCode {
    Success,
    FailureGoalHasSubgoals
}

enum GoalCompletionStatus {
    Incomplete,
    Succeeded,
    Failed,
    Deleted
}

///
struct Goal {
    parent_id: u128,  // 0 for no parent
    goal_id: u128,  // 0 if not yet defined
    start_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    end_unix_timestamp: u128,  // 0 if undefined (e.g for recurrences)
    failure_callback: [str],
    success_callback: [str],
    finally_callback: [str],
    completion_status: GoalCompletionStatus
}

///
struct TimebasedCriteria {
    time_ms: u128,
    link_id: u128,  // 0 if custom task is present
    task: String,  // Only relevant if link_id is 0
    feed: bool  // Only relevant is link_id is NOT 0
}

///
struct TimebasedGoal {
    goal: Goal,
    criteria: TimebasedCriteria
}

///
struct TaskbasedCriteriaItem {
    description: str,
    link_id: u128  // 0 if unlinked
}

///
struct TaskbasedGoal {
    goal: Goal,
    criteria: [TaskbasedCriteriaItem]
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
struct TimebasedRecurrence {
    timebased_goal: TimebasedGoal,
    recurrence: Recurrence
}

///
struct TaskbasedRecurrence {
    taskbased_goal: TimebasedGoal,
    recurrence: Recurrence
}

///
trait IRepo {
    ///
    fn create_edit_timebased_goal(&self, goal_dat: &TimebasedGoal) -> CreateGoalCode;

    ///
    fn create_edit_taskbased_goal(&self, goal_dat: &TaskbasedGoal) -> CreateGoalCode;

    ///
    fn create_edit_timebased_recurrence(&self, recurrence_dat: TimebasedRecurrence) -> bool;

    ///
    fn create_edit_taskbased_recurrence(&self, recurrence_dat: TaskbasedRecurrence) -> bool;

    ///
    fn delete_goal(&self, goal_id: u128) -> GoalDeleteCode;

    ///
    fn feed_timebased_goal(&self, goal_id: u128, time_to_add_ms: u128) -> bool;

    ///
    fn uncheck_taskbased_criteria(&self, goal_id: u128, criteria_index: usize) -> bool;

    ///
    fn check_taskbased_criteria(&self, goal_id: u128, criteria_index: usize) -> bool;

    ///
    fn succeed_goal(&self, goal_id: u128) -> GoalDeathCode;

    ///
    fn fail_goal(&self, goal_id: u128) -> GoalDeathCode;

    ///
    fn get_expired_goal_ids(&self, cur_time_unix_timestamp: u128) -> &'static[u128];

    ///
    fn get_timebased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, TimebasedGoal);

    ///
    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> &'static(GetGoalCode, TaskbasedGoal);

    ///
    fn get_goal_by_id(&self, goal_id: u128) -> &'static(GetGoalCode, Goal);

    ///
    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> &'static[TimebasedGoal];

    ///
    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> &'static[TaskbasedGoal];

    ///
    fn get_goals(&self) -> &'static[Goal];

    ///
    fn is_timebased(&self, goal_id: u128) -> bool;

    /// Generates goals from the recurrence up to and including the first goal whose start time would be after cur_time
    fn generate_goals_from_recurrence(&self, recurrence_id: u128, cur_time: u128) -> bool;

    /// Get subgoals of the given goal at a depth of one. This means subgoals of subgoals are not included
    fn get_immediate_subgoals(&self, goal_id: u128) -> &'static[Goal];
}


struct InMemoryRepo {
    next_free_id: u128,
    timebased_recurrences: Vec<TimebasedRecurrence>,
    taskbased_recurrences: Vec<TaskbasedRecurrence>,
    timebased_goals: Vec<TimebasedGoal>,
    taskbased_goals: Vec<TaskbasedGoal>
}

impl InMemoryRepo {
    fn get_next_free_id(&mut self) -> u128 {
        self.next_free_id += 1;
        self.next_free_id
    }

    fn is_within_bounds(outer_start: u128, outer_end: u128, inner_start: u128, inner_end: u128) -> bool {
        inner_start >= outer_start && inner_start <= outer_end
    }

    fn __create_edit_helper(&self, goal_dat: &Goal) -> (bool, CreateGoalCode) {
        // Check parent bounds
        if (goal_dat.parent_id != 0) {
            let (code, goal) = self.get_goal_by_id(goal_dat.parent_id);
            if (!InMemoryRepo::is_within_bounds(goal.start_unix_timestamp, goal.end_unix_timestamp,
                                                goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp)) {
                return (false, CreateGoalCode::FailureSubgoalOutsideParentTimebound);
            }
        }

        // Edit mode
        if (goal_dat.goal_id != 0) {

            // Check that new timebound doesn't put any subgoals outside
            let subgoals = self.get_immediate_subgoals(goal_dat.goal_id);
            for subgoal in subgoals {
                if (!InMemoryRepo::is_within_bounds(
                    goal_dat.start_unix_timestamp, goal_dat.end_unix_timestamp,
                    subgoal.start_unix_timestamp, subgoal.end_unix_timestamp
                )) {
                    return (false, CreateGoalCode::FailureNewTimeboundSmallerThanSubgoals);
                }
            }

            // Check that the correct goal type is being edited
            if (!self.is_timebased(goal_dat.goal.goal_id)) {
                return (false, CreateGoalCode::FailureEditingGoalOfIncorrectType);
            }

            return (true, CreateGoalCode::Success);
        }

        // Create mode
        else {
            return (false, CreateGoalCode::Success);
        }
    }
}

impl IRepo for InMemoryRepo {
    fn create_edit_timebased_goal(&mut self, goal_dat: &TimebasedGoal) -> CreateGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if (code == CreateGoalCode::Success) {
            if (is_edit) {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if (self.timebased_goals[i].goal.goal_id == goal_dat.goal.goal_id) {
                        self.timebased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.timebased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.timebased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback.clone();
                        self.timebased_goals[i].goal.success_callback = goal_dat.goal.success_callback.clone();
                        self.timebased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback.clone();
                        self.timebased_goals[i].criteria.feed = goal_dat.criteria.feed;
                        self.timebased_goals[i].criteria.time_ms = goal_dat.criteria.time_ms;
                        self.timebased_goals[i].criteria.task = goal_dat.criteria.task.clone();
                        self.timebased_goals[i].criteria.link_id = goal_dat.criteria.link_id;
                        break;
                    }
                }
                return code;
            } else {
                self.timebased_goals.push(goal_dat);
                self.timebased_goals[-1].goal.goal_id = self.get_next_free_id();
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_taskbased_goal(&mut self, goal_dat: &TaskbasedGoal) -> CreateGoalCode {
        let (is_edit, code) = self.__create_edit_helper(&goal_dat.goal);

        if (code == CreateGoalCode::Success) {
            if (is_edit) {
                // Perform edit
                for i in 0..self.timebased_goals.len() {
                    if (self.taskbased_goals[i].goal.goal_id == goal_dat.goal.goal_id) {
                        self.taskbased_goals[i].goal.start_unix_timestamp = goal_dat.goal.start_unix_timestamp;
                        self.taskbased_goals[i].goal.end_unix_timestamp = goal_dat.goal.end_unix_timestamp;
                        self.taskbased_goals[i].goal.failure_callback = goal_dat.goal.failure_callback.clone();
                        self.taskbased_goals[i].goal.success_callback = goal_dat.goal.success_callback.clone();
                        self.taskbased_goals[i].goal.finally_callback = goal_dat.goal.finally_callback.clone();
                        self.taskbased_goals[i].criteria = goal_dat.criteria.clone();
                        break;
                    }
                }
                return code;
            } else {
                self.taskbased_goals.push(goal_dat);
                self.taskbased_goals[-1].goal.goal_id = self.get_next_free_id();
                return code;
            }
        } else {
            return code;
        }
    }

    fn create_edit_timebased_recurrence(&mut self, recurrence_dat: TimebasedRecurrence) -> bool {
        self.timebased_recurrences.push(recurrence_dat);
        return true;
    }

    fn create_edit_taskbased_recurrence(&mut self, recurrence_dat: TaskbasedRecurrence) -> bool {
        self.taskbased_recurrences.push(recurrence_dat);
        return true;
    }

    fn delete_goal(&self, goal_id: u128) -> GoalDeleteCode {
        todo!()
    }

    fn feed_timebased_goal(&self, goal_id: u128, time_to_add_ms: u128) -> bool {
        todo!()
    }

    fn uncheck_taskbased_criteria(&self, goal_id: u128, criteria_index: usize) -> bool {
        todo!()
    }

    fn check_taskbased_criteria(&self, goal_id: u128, criteria_index: usize) -> bool {
        todo!()
    }

    fn succeed_goal(&self, goal_id: u128) -> GoalDeathCode {
        todo!()
    }

    fn fail_goal(&self, goal_id: u128) -> GoalDeathCode {
        todo!()
    }

    fn get_expired_goal_ids(&self, cur_time_unix_timestamp: u128) -> &'static [u128] {
        todo!()
    }

    fn get_timebased_goal_by_id(&self, goal_id: u128) -> (GetGoalCode, TimebasedGoal) {
        todo!()
    }

    fn get_taskbased_goal_by_id(&self, goal_id: u128) -> &'static (GetGoalCode, TaskbasedGoal) {
        todo!()
    }

    fn get_goal_by_id(&self, goal_id: u128) -> &'static (GetGoalCode, Goal) {
        todo!()
    }

    fn get_timebased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> &'static [TimebasedGoal] {
        todo!()
    }

    fn get_taskbased_goals(&self, start_unix_timestamp: u128, end_unix_timestamp: u128) -> &'static [TaskbasedGoal] {
        todo!()
    }

    fn get_goals(&self) -> &'static [Goal] {
        todo!()
    }

    fn is_timebased(&self, goal_id: u128) -> bool {
        todo!()
    }

    fn generate_goals_from_recurrence(&self, recurrence_id: u128, cur_time: u128) -> bool {
        todo!()
    }

    fn get_immediate_subgoals(&self, goal_id: u128) -> &'static [Goal] {
        todo!()
    }
}
