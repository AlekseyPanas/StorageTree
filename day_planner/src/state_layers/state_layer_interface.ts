export enum ContributorType {
    SUBGOAL,
    STRATEGY,
    SCHEDULING
}

// Different functions of this type take a point in time and generate a relative actual time based on some potentially additional input
export interface RelativeTimeGenerator {
    get_relative_unix_time(relative_base_unix_time: number): number;
}

// Gets the idx-th timestamp in a pattern starting from a given lower bound, or NULL if out of bounds by upper bound
export interface RecurrencePattern {
    get_ith_timestamp(lower_bound_unix: number, upper_bound_unix: number, idx: number): number | null;
}

// Goal data which doesn't change across recurrence spawns
export interface StaticGoalDat {
    name: string;
    desc: string | null;
    timebased_criteria_seconds: number | null;
    ect_seconds: number | null;
    contributes_to: Set<number> | null;
    contributor_type: ContributorType | null;
}

// Existing goal instance
export interface Goal {
    id: number;
    static_data: StaticGoalDat;
    timebased_progress_seconds: number | null;
    starttime_unix: number;
    deadline_unix: number;
    subgoal_set: Set<number> | null;
    is_subgoal_set_completing: boolean | null;
    strategy_set: Set<number> | null;
    scheduling_set: Set<number> | null;
}

// Template for generating a recurrence relative to a window
export interface RecurrenceTemplate {
    static_data: StaticGoalDat;

    subgoal_set: Set<GoalTemplate | RecurrenceTemplate>
    strategy_set: Set<GoalTemplate | RecurrenceTemplate>

    rec_lower_bound_unix: number;  // These bounds should be set to parent goal time range if one is present
    rec_upper_bound_unix: number | null;

    rec_pattern: RecurrencePattern;
    starttime_generator: RelativeTimeGenerator;
    deadline_generator: RelativeTimeGenerator;
}

// Template for generating a goal relative to a time
export interface GoalTemplate {
    static_data: StaticGoalDat;

    subgoal_set: Set<GoalTemplate | RecurrenceTemplate>
    strategy_set: Set<GoalTemplate | RecurrenceTemplate>

    starttime_rel: RelativeTimeGenerator;
    deadline_rel: RelativeTimeGenerator;
}

// Existing recurrence instance
export interface Recurrence {
    id: number;
    template: RecurrenceTemplate;
    is_generated(idx: number): boolean;
    link_goal(idx: number, goal_id: number): void;
    get_linked_goal_id(idx: number): number | null;
    unlink_goal(idx: number): void;
    set_generated(idx: number, is_generate: boolean): void;
}

export interface AppState {
    goals: Goal[];
    recurrences: Recurrence[];
}

export interface Maybe<T> {
    data: T | null;
    not_logged_in_error: boolean;
    not_authorized_error: boolean;
    license_token_expired_error: boolean;
}

export interface StateLayerAdapter {
    // Acquiring and reacting to state changes
    get_current_state(): Maybe<AppState>;
    on_state_change(fn: (state: AppState) => void): void;

    // Submitting state change operations

    // TODO: This list is growing
}
