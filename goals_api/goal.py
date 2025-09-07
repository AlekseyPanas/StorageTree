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
