from datetime import datetime, time, timedelta
import uuid


class Manager:
    def __init__(self):
        self.goals = []




class Goal:
    def __init__(self, criteria: str,
                 start_date: datetime, deadline: datetime,
                 success: str, failure: str, on_death: str,
                 is_time_based=False, criteria_time=None):
        """
        :param criteria: NotNull string of criteria for goal completion
        :param start_date: NotNull start date of goal activation
        :param deadline: Nullable deadline. If null, goal goes in special list of deadline-less goals
        :param success: String describing what to do on success
        :param failure: String describing what to do on failure
        :param on_death: String describing what to do on death
        :param is_time_based: Is the criteria based on time (e.g work on X for X hours)
        :param criteria_time: If criteria is time based, for how long must you work on criteria to succeed
        """
        self.__goal_id = str(uuid.uuid4())
        self.__criteria = criteria
        self.__is_time_based = is_time_based
        self.__criteria_time = criteria_time
        self.__start_date = start_date,
        self.__deadline = deadline
        self.__success = success
        self.__failure = failure
        self.__on_death = on_death


"""
Notes:

Horizontal timetable

- A goal has a starting date and a deadline date.
- A goal can be single or recurring.
- A goal has the following optional callbacks:

    success {
        What to do if criterion was fulfilled within time bound
    } failure {
        What to do if criterion wasn't fulfilled within time bound
    } finally (death) {
        What to do when the time bound is over, regardless of success or failure
    }

- A goal can be time-based or task-based.
    - Time-based: Criterion is based on time, such as working on something for X amount of time
    - Task-based: Criterion is based on completing a task
- Dates in a goal can either be set by you or be set by another party outside your control (e.g exam)

- A goal comes with a "strategy", which is essentially a list of subgoals.

    - All task based goals break down into time-based subgoals. This is because to accomplish any task, you need to
    dedicate time to it. This is fundamentally a goal. Therefore, the leaves of any subgoal tree are always time-based.
    However, below examples, especially the 3rd one, illustrate how some timebased subgoals are just direct replicas
    of the parent goal. This formality is so that taskbased goals can be planned in the calendar

    - Subgoals can only exist within the timebound of the parent goal. Therefore, subgoals will always expire before
    the parent goal. Recurring subgoals recur up to the last possible recurrance without exceeding parent timebound

    Examples:
    - Get P3 (taskbased) { fulfill requirements (taskbased) { go fly {timebased} }, take exam (event), do SIV (event), get signed off (event) }
    - Do assignment (taskbased) { work on assignment (timebased), submit assignment (taskbased) { work on submitting {timebased} } }
    - Take a photo of Ascutney (taskbased) { work on taking a photo of ascutney (timebased) }

- A goal is "tightly bounded" if its start and end date difference are exactly equal to the time criterion. 
A goal must be time-based for this to work, since a task-based goal may take longer or shorter than the given interval.

- Example: An "Event", such as a lecture, is a time-based goal whose start and end dates are tightly bounded
 to the time-based criterion and are not controlled by you. For this reason, "Event" will be a shorthand format
 for this type of goal; a goal which is both tightly bounded AND the dates are not controlled by you

- Goal resolution:
    - If a goal's timebound expires, the goal gets added to a queue of unresolved goals
        - The queue must be cleared each time the app is opened before other actions can be performed
        - If a subgoal's expiration is tied with the parent's, the subgoal must be resolved first
        - Resolving means checking success (as in the goal was completed) or failure (means the goal expired uncompleted)
        - During resolution, the goal can be edited in case plans changed but the app wasn't opened earlier. After editing
        is finished, the goal will either reappear in the queue or be removed (typically editing involves extending deadline)
        - During resolution, the goal can also be deleted
    - A goal can always be checked off as a success anytime
        - Since subgoals are resolved first in timebound expiration, all subgoals will be dead prior to parent goal
        UNLESS its a recurring subgoal OR the goal was checked off as an early success
    - A success goal death prompts the execution of the success callback immediately. Same with failure goal death
        - A failure which involves deadline extension will automatically revive any recurring subgoals
        - A goal death will automatically kill all subgoals (e.g early success). Only the "finally" callback will execute
        on those subgoals

- Common failure callbacks:
    - Extend deadline with optional penalty or increasing penalty per extension
    - New goal to makeup the missed goal with optional penalty

- Recurring goals:
    - A recurrence is a function which spawns goals by some set of time-based rules. Therefore, a "recurrence" and a goal
    resulting from a recurrence are separate things
    - A recurring goal's time bound can overlap or have gaps
    - A single instance of a recurring goal is active at a time
    - A recurrence can be configured to:
        - End at a certain date: it will recur until the next recurrence would exceed this date (thus there may be a gap)
        - Recur a specific number of times after which it stops
        - Omit/cancel a specific recurrence by date or count
    - Editing a goal spawned as part of the recurrence also has a section to edit the recurrence itself
    - Goals as part of a recurrence technically spawn immediately upon the goal start date of the next recurrence. Once
    the goal is spawned, it is now unaffected by recurrence edits

- Goal display
    - Goals will all be shown in the horizontal timeline stacked vertically
    - Recurrences will show "ghost" goals which have yet to be spawned
    - Subgoals will always take rows immediately below the parent
    - Recurrences take up a whole row of their own. Otherwise, non-overlapping goals may use the same row
    - A time-based goal whose timebound is greater than the criteria appears differently than those that are tightly bounded
        - It is typical for a non-tightly-bounded time-based goal to have subgoals which are tightly bounded
        - e.g Work on Paxos for 10 hours in the week, but then you schedule tight work goals throughout the week
    - Events have an extra highlight to them

- Goal Progress
    - Nested timebased: It is often useful to break down long term time-based goals into smaller time-based goals. To put
    in 500 hours of reading in a year, you might read for 2-3 hours a day. Such nesting can be directly configured to feed
    into the criteria progress for the parent goal
    - Record time: A time-based goal has a nice UI to record time you've committed to the goal so far
    - Checklist: Criteria can have an embedded checklist which saves state of checked items
    - Checklist goal link: A checklist item can link to subgoal and will automatically get checked off if that subgoal
    has succeeded (or show a red X if the goal has failed)

- Draft goals: It is often useful to have a goal be partially created but not yet active. For example, you know you'll
go on a hike in the next year, but you have no clue when. You dont want the goal to be evaluated for failure or success
yet. Such goals are left as "drafts".
    - If your draft has a date set on it (as a tentative date), then it will show up on the timeline with a
    special "ghost" appearance.
    - The app will prompt you to edit the goal if the tentative start date is approaching within a configured interval
    - Active goals can be changed to draft goals
    - Deadlineless goals are effectively "Draft" goals since you are not actively pursuing their completion yet

- Window Goals: This special type of goal is a maintenance goal. Rather than having a criteria which must be completed,
the criteria must be maintained. e.g Maintain 5 hours of bike riding per 7 days.
    - Has a time window within which the criteria must be satisfied.
    - For the above example, as long as you biked for at least 5 hours in the past 7 days, you're maintaining it.
    - If you biked 5 hours in the past 7 days but 3 of those were done exactly 7 days ago, the next day over you will
    only have biked 2 hours in the past 7 days since the window moved. You must now bike again to maintain the goal
    - success/failure for such a goal must be configured to either be interval-based or continuous. For the duration
    that the goal is maintained, the "success" will be evaluated. Otherwise failure
        - e.g continuous: +3 coins / 1 hour. Then for any unit of time that the goal is maintained, you gain an exact
        amount of coin
        - e.g interval: 10 pushups / 1 day of no maintenance. If less than a day, then you dont get your punishment/reward.
        It must be that the interval has passed exactly.goal
    - The criteria for this goal must be numerically quantifiable. This is required and built into the app
    - To log completion of this goal, you enter a quantity along with a log time
        - e.g for the bike example, you might log 30 min at 4:15 PM, Monday.
    - If you open the app maintenance goals will prompt you
        - An unmaintained goal will first ask you to log anything you may have not logged in hindsight.
        - Then if the success/failure is interval based, it will prompt you to execute the callback for how many ever
        intervals have passed in maintained or unmaintained states based on what you logged
            - If continuous, it will prompt you once for the whole duration


- Dead goals are kept on record for a while until they are deleted (maybe month or more)

- At any time for any reason, you have the power to modify goals, move them, etc

- Automated callbacks: Some callback tasks on success or failure can be preset automations. You will still be prompted
to execute this automation when resolving a goal, but you wont need to do manual labor. Automations include
    - Extend deadline of this goal by X
    - TODO: Add more. e.g creating new goals, modifying values, etc. Automatic tasks will be choosable from a list with
    a custom set of parameters for each, and add-able to a goal's blocks.

- Evening Checkin Summary:
    - [2] Complete quick-tasks, which often involve creating goals
    - [3] Plan next day, and plan the week ahead

- Morning Checkin Summary:
    - [1] Check of yesterday's goal completion
"""

