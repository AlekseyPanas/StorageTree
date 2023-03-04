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
- Dates in a goal can either be set by you or be set by another party outside your control

- A goal comes with a "strategy", which is essentially a list of subgoals. Each goal should always have many subgoals
instead of cramming a long criterion or complex time bound. If a goal's time bound ends, the goal dies along with
all of its sub goals

- A goal is "tightly bounded" if its start and end date difference are exactly equal to the time criterion. 
A goal must be time-based for this to work, since a task-based goal may take longer or shorter than the given interval.

- Example: An "Event", such as a lecture, is a time-based goal whose start and end dates are tightly bounded
 to the time-based criterion and are not controlled by you. For this reason, "Event" will be a shorthand format
 for this type of goal; a goal which is both tightly bounded AND the dates are not controlled by you

- A Calendar always shows all tightly bounded goals as anchored to their time since there is no customization to their
movement. Furthermore, "Events" will be shown in a highlighted manner indicating that the dates are not controlled by
you

- Goals that aren't tightly bounded will float around freely within their time bound. You can drag them around times
and dates but not outside the bound. As part of habitica evening check, you should always plan your next day concretely,
but also you should move around and plan days ahead as you may have new information

- [1] Morning checkins will involve checking off what you did yesterday from your planned day goal list, and what you didn't,
as well as executing success{} and finally{} blocks. If a goal's time-bound exceeded and you didn't do it, also execute
failure{}. Otherwise, the goal you didn't complete gets moved back into planning if it has more time until it's time-end bound

- [2] Habitica should still be checked mornings and evenings. If you need to quickly scribble down a goal, but don't
have access to this app, add it to the to do habitica list. In the evening procedure, it should be mandatory
to go through the to do and do all the "quick tasks" before bed, or turn them into goals otherwise

- At any time for any reason, you have the power to modify goals, move them, etc

- Evening Checkin Summary:
    - [2] Complete quick-tasks, which often involve creating goals
    - [3] Plan next day, and plan the week ahead

- Morning Checkin Summary:
    - [1] Check of yesterday's goal completion

- Dealing with late work: Sometimes, a goal's end date reaches and you now must execute the fail + finally actions
if the goal wasn't complete. Often, you might want to create a second goal which is a "catch up on this thing"-type 
goal. Maybe a second late penalty deadline gets enacted, or maybe you have a weekend where you catch up on missed work.
Whatever the case, it is convenient to simply create a second goal as the failure event of the first one. This should
be an easy UI option. Furthermore, this second goal creation is an automatic task, so it shouldnt require manual work

- The types of callbacks you can specify in the failure{} success{} finally{} blocks can be manual or automatic.
    - Manual are tasks written in English that cannot be done by the app but must be done by you
    - Automatic are tasks that the app can do automatically, such as creating new goals, modifying values, etc.
    Automatic tasks will be choosable from a list with a custom set of parameters for each, and add-able to a goal's
    blocks.

- Something to note is that not all goals appear in the timetable! Consider the goal "Do assignment". For this goal to
appear, you need time-based subgoals such as "Work on assignment". Since it is so common to create and constantly move
these "Work on assignment"-type subgoals, they should be easily creatable. The "Do Assignment" and any other non-time-based
goals will appear in a goal list outside, with their start and end date bounds visible on the calendar. You can easily
create the work subgoals by a simple UI interface. You can name rename them if you'd like. Maybe you want some
"Submit assignment" goal for 30 min near the deadline.
    
- Whenever you open the app, any goals whose deadline has passed BUT have not been dealt with will appear highlighted.
These goals may either be on the timetable (if time-based) or in the separate list if task-based. Each highlighted goal
can be "finalized". Finalizing involves marking it as complete or "failed", and then executing the corresponding event
tasks in the blocks. If all the events are automatic for the given outcome (fail/success), then a simple button click
will execute them. Otherwise you will be shown the manual tasks you must do and then a button to confirm you've done
them can close the goal. completed/failed goals will be in their own list, never deleted. You can delete a goal manually
anytime.

- Sometimes you want to keep track of goal information throughout the period. Add an optional text area to enter info

- Some goals have no deadline. These goals linger in a separate list and might acquire a deadline later.
"""

