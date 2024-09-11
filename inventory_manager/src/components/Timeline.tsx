import "../styles/Timeline.css"
import GoalBlock from "./GoalBlock";
import { do_bounds_intersect } from "../util";
import {createRef, useEffect, useRef, useState} from "react";

enum CompletionStatus {
    INCOMPLETE,
    SUCCEEDED,
    FAILED,
    DELETED
}

interface TimebasedCriteria {
    timeMs: number,
    linkId: number,
    task: string,
    feed: boolean,
    dedicatedTimeMs: number
}

interface TaskbasedCriteriaItem {
    description: string,
    linkId: number,
    isChecked: boolean
}

interface GoalDat {
    parentId: number,
    sourceRecurrence: number,  // -1 if None
    goalId: number,
    startUnixTimestamp: number,
    endUnixTimestamp: number
    goalName: string,
    criteria: TimebasedCriteria | TaskbasedCriteriaItem[],
    isTimebased: boolean,
    success: string[],
    failure: string[],
    final: string[],
    completionStatus: CompletionStatus,
    isRecurrenceGhost: boolean
}

interface RecurrenceDat {
    recurrenceId: number,
    startUnixTimestamp: number,
    endUnixTimestamp: number,
    spawnIntervalMs: number,
    GoalDurationMs: number,
    latestSpawnedTimeMs: number,
    goal: GoalDat
}

/**
 * Return a list of goal objects fetched from backend repository
 */
function fetchGoalData(): GoalDat[] {
    return [{
        goalId: 0,
        title: "TestGoal",
        criteria: {
            type: "taskbased",  // event, taskbased, timebased
            dat: { desc: "Do this goal" }  // {desc: str}, {desc: str}, {task: str, timehours: int}
        },
        success: ["+3B", "Candy"],
        failure: ["-3B / hr, +2B / hr"],
        startTime: new Date(2024, 7, 15, 5, 30, 3),
        deadline: new Date(2024, 7, 18, 5, 30, 3),
        isDraft: false,
        sourceRecurrenceId: -1
    }];
}

/**
 * Return a list of recurrence objects fetched from backend repository
 */
function fetchRecurrenceData(): RecurrenceDat[] {
    return [{
        recurrenceId: 0,
        title: "TestRecurrenceGoal",
        criteria: {
            type: "taskbased",
            dat: { desc: "Do this recurring thing!" }
        },
        success: ["+5B", "Cake"],
        failure: ["-6B / hr, +1B / hr"],
        startTime: new Date(2024, 7, 12, 5, 30, 3),
        goalLengthSeconds: 108000 / 2,  // seconds
        goalSpawnInterval: 201600 / 2,  // seconds
        isDraft: false
    }];
}

/**
 * Given the start date epoch of the latest goal generated from the recurrence rec,
 * return a list of parsed ghost goals ready for timeline use ending before the given date.
 * if latestGeneratedEpoch = 0, then assume recurrence has not yet spawned any goals
 */
function generateGhostRecurrenceGoals(latestGeneratedEpoch: number, before: Date, rec: RecurrenceDat): GoalDat[] {
    let ghostGoals = [];
    let start = latestGeneratedEpoch === 0 ? rec.startTime.getTime() : latestGeneratedEpoch + (rec.goalSpawnInterval * 1000);
    for (let i = start; i <= before.getTime(); i += rec.goalSpawnInterval * 1000) {
        ghostGoals.push({
            title: rec.title,
            criteria: getParsedCriteria(rec.criteria),
            success: rec.success,
            failure: rec.failure,
            startTime: new Date(i),
            deadline: new Date(i + (rec.goalLengthSeconds * 1000)),
            isRecurrenceGhost: true
        });
    }
    return ghostGoals;
}

/**
 * Generate ghost recurrence goals and arrange all goals (including ghosts) into timeline rows such that
 * children are immediately below parents and recurrences share a row
 */
function generate_timeline(goals: GoalDat[], recurrences: RecurrenceDat[],
                           start_unix_ms: number, end_unix_ms: number): GoalDat[][] {
    /*
    * arrangement:
	- Get all relevant goals and ghost recurrence goals
	- Track goals that have been "placed"
	- loop goals
	- Add goal to next available row + all goals that share the same recurrence
	- loop goal's children
	- Add goals to next available row + all goals that share the same recurrence
	- Repeat until no more children
	- Continue looping goals until all goals "placed"
    * */

    // Collect goals which intersect bounds
    let finalGoals = goals.filter((goal) => do_bounds_intersect(goal.startUnixTimestamp, goal.endUnixTimestamp,
        start_unix_ms, end_unix_ms));

    // Create and append recurrence ghosts from all recurrences
    

    let indexes = Array.from({ length: goals.length }, (_, index) => index);
    let rows: GoalDat[][] = new Array<GoalDat[]>();

}


function Timeline() {
    const [goalData, setGoalData] = useState<GoalDat[]>(fetchGoalData());
    const [recurrenceData, setRecurrenceData] = useState<RecurrenceDat[]>(fetchRecurrenceData());
    const [timelineStruct, setTimelineStruct] = useState({
        secondsPerPixel: 20,
        leftEdgeDate: new Date(2024, 7, 14, 2, 0, 0),
    });
    const [timelineRows, setTimelineRows] = useState<ParsedGoal[][]>([]);

    let parentRefs = useRef([]);
    let childRefs = useRef([]);
    updateChildAndParentRefArrays();

    let containerRef = useRef(null);
    let [containerWidth, setContainerWidth] = useState(0);

    let dragPosRecord = useRef(0);
    let isDragging = useRef(false);

    function updateChildAndParentRefArrays() {
        parentRefs.current = timelineRows.map((goals, i) => null);
        childRefs.current = timelineRows.map((goals, i) => (
            goals.map((goal, j) => null)
        ));
        //console.log(childRefs, parentRefs);
        console.log("1");
    }

    /**
     * Fetches data from backend and sets it to state variables
     */
    function fetchData() {
        setGoalData(fetchGoalData());
        setRecurrenceData(fetchRecurrenceData());
    }

    /**
     * Criteria can be slightly different depending on type. This returns the criteria
     * as an object with a description parsed into a string
     */
    function getParsedCriteria(criteria: object): ParsedCriteria {
        let desc;
        if (criteria.type === "timebased") {
            desc = "Work on " + criteria.dat.task + " for " + criteria.dat.timehours + " hours";
        } else {
            desc = criteria.dat.desc;
        }
        return {type: criteria.type, desc: desc};
    }

    /**
     * Using timeline data computes the date at the right edge of the timeline window
     */
    function getRightEdgeDate() {
        let totalMs = timelineStruct.secondsPerPixel * containerWidth * 1000;
        return new Date(timelineStruct.leftEdgeDate.getTime() + totalMs);
    }

    /**
     * Using stored fetched goal and recurrence data, update the 'rows' element of
     * the timeline struct
     * */
    function getUpdatedTimelineRows(): ParsedGoal[][] {
        let rows = [];

        for (let rec of recurrenceData) {
             let row = [];
             let latestEpoch = 0;
             for (let goal of goalData) {
                 if (goal.sourceRecurrenceId === rec.recurrenceId) {
                    row.push({
                        title: goal.title,
                        criteria: getParsedCriteria(goal.criteria),
                        success: goal.success,
                        failure: goal.failure,
                        startTime: goal.startTime,
                        deadline: goal.deadline,
                        isRecurrenceGhost: false
                    });
                    if (goal.deadline.getTime() > latestEpoch) {
                        latestEpoch = goal.deadline.getTime();
                    }
                 }
             }
             row = row.concat(generateGhostRecurrenceGoals(latestEpoch, getRightEdgeDate(), rec));
             rows.push(row);
        }

        return rows;
    }

    /**
     * Update height of rows based on goals within them
     */
    function updateHeight() {
        for (let i = 0; i < parentRefs.current.length; i++) {
            let height = 0;
            for (let j = 0; j < childRefs.current[i].length; j++) {
                if (childRefs.current[i][j] !== null && childRefs.current[i][j].offsetHeight > height) {
                    height = childRefs.current[i][j].offsetHeight;
                }
            }
            parentRefs.current[i].style.height = height + "px";
        }
        console.log("height update");
    }

    function timeScale(event) {
        setTimelineStruct({...timelineStruct,
            secondsPerPixel: timelineStruct.secondsPerPixel * Math.pow(1.003, event.deltaY)});
    }

    function mouseDownHandler(event) {
        event.preventDefault();
        isDragging.current = true;
        dragPosRecord.current = event.screenX;
    }

    function mouseMoveHandler(event) {
        event.preventDefault();
        if (isDragging.current) {
            let diffPx = event.screenX - dragPosRecord.current;
            let newLeftEdge = new Date(timelineStruct.leftEdgeDate.getTime() - (diffPx * timelineStruct.secondsPerPixel * 1000));
            setTimelineStruct({...timelineStruct, leftEdgeDate: newLeftEdge})
            dragPosRecord.current = event.screenX;
        }
    }

    function mouseUpHandler(event) {
        event.preventDefault();
        isDragging.current = false;
    }

    function resizeHandler() {
        setContainerWidth(containerRef.current.offsetWidth);
    }

    useEffect(() => {
        resizeHandler();
        fetchData();
        setTimelineRows(getUpdatedTimelineRows());
    }, []);

    useEffect(() => {
        window.addEventListener("resize", resizeHandler);
        window.addEventListener("wheel", timeScale);
    }, [resizeHandler, timeScale]);

    useEffect(() => {
        setTimelineRows(getUpdatedTimelineRows());
    }, [timelineStruct]);

    useEffect(() => {
        updateHeight();
    }, [timelineRows]);

    useEffect(() => {
        //console.log(childRefs, parentRefs);
        console.log("3");
        console.log("");
    });

    return (
        <div ref={containerRef} id="timelineContainer" onMouseDown={mouseDownHandler} onMouseUp={mouseUpHandler}
             onMouseMove={mouseMoveHandler}>

            {
                // Creates time ticks above the timeline
                (() => {
                    let totalMs = timelineStruct.secondsPerPixel * containerWidth * 1000;
                    let rightEdgeDate = new Date(timelineStruct.leftEdgeDate.getTime() + totalMs);

                    let intervalMs;
                    let getStringFunc;
                    let firstDate = new Date(timelineStruct.leftEdgeDate.getTime());
                    if (totalMs > (7.884e+9 / 2)) {  // 2 months
                        intervalMs = 2.628e+9;
                        getStringFunc = (d: Date) => { return d.toLocaleString('default', { month: 'short' }); };
                        firstDate.setDate(0);
                        firstDate.setHours(0);
                        firstDate.setMinutes(0);
                        firstDate.setSeconds(0);
                        firstDate.setMilliseconds(0);
                    } else if (totalMs > 2.592e+8) {  // 3 days
                        intervalMs = 8.64e+7;
                        getStringFunc = (d: Date) => { return d.getMonth() + "/" + d.getDate() + "/" + d.getFullYear() };
                        firstDate.setHours(0);
                        firstDate.setMinutes(0);
                        firstDate.setSeconds(0);
                        firstDate.setMilliseconds(0);
                    } else {
                        intervalMs = 3.6e+6;
                        getStringFunc = (d: Date) => { return d.getHours().toString() + ":00" };
                        firstDate.setMinutes(0);
                        firstDate.setSeconds(0);
                        firstDate.setMilliseconds(0);
                    }
                    firstDate = new Date(firstDate.getTime() + intervalMs);

                    let ticks = []
                    for (let ms = firstDate.getTime(); ms < rightEdgeDate.getTime(); ms += intervalMs) {
                        let curDat = new Date(ms);

                        let leftPx = ((ms - timelineStruct.leftEdgeDate.getTime()) / 1000) / timelineStruct.secondsPerPixel;
                        let text = getStringFunc(curDat);

                        ticks.push({leftPx: leftPx, text: text});
                    }

                    return ticks.map((tick, i) => (
                        <div className="timelineVerticalLine" style={{left: tick.leftPx}}></div>
                    )).concat(ticks.map((tick, i) => (
                        <p className="timelineTimeText" style={{left: tick.leftPx}}> {tick.text} </p>)))
                })()
            }

            {
                // Goal block rows
                timelineRows.map((goals, i) => (
                    (
                        <div className="timelineRow" ref={(el) => {console.log("2");parentRefs.current[i] = el;}}>
                            {
                                goals.map((goal, j) => {
                                    let goalStartMs = goal.startTime.getTime();
                                    let diffMs = goal.deadline.getTime() - goalStartMs;
                                    let leftEdgeMs = timelineStruct.leftEdgeDate.getTime();
                                    let diffPx = diffMs / (timelineStruct.secondsPerPixel * 1000);
                                    let goalStartPx = (goalStartMs - leftEdgeMs) / (timelineStruct.secondsPerPixel * 1000);
                                    return (
                                        <div className="goalBlockOuterContainer" ref={(el) => {childRefs.current[i][j] = el;}}>
                                            <GoalBlock
                                                key={i + "," + j}
                                                goalTitle={goal.title}
                                                goalCriteria={goal.criteria.desc}
                                                goalSuccess={goal.success}
                                                goalFailure={goal.failure}
                                                leftOffsetPx={goalStartPx}
                                                widthPx={diffPx}
                                                goalType={goal.criteria.type}
                                                goalColor={"darksalmon"}
                                                isGhost={goal.isRecurrenceGhost}
                                            ></GoalBlock>
                                        </div>
                                    )
                                })
                            }
                        </div>
                    )
                ))
            }

        </div>
    );
}

export default Timeline;
