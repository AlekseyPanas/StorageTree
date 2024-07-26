import "../styles/Timeline.css"
import GoalBlock from "./GoalBlock";
import {useEffect, useRef, useState} from "react";
import {i} from "@tauri-apps/api/fs-6ad2a328";
import {d} from "@tauri-apps/api/http-43c39402";

// fn fetch_goal_data() -> Goal[]
// - Fetch all goals from DB
//
// fn fetch_recurrences() -> Recurrence[]
// - Fetch all recurrences from DB
//
// fn build_timeline_structure(Goal[], Recurrence[]) -> GoalBlock[]
// - Converts goals and recurrences into the timetable structure
// - More precisely, determines the row each goal and recurrence lives on
// - Saves this data to timeline_structure

function fetchGoalData() {
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

function fetchRecurrenceData() {
    return [{
        recurrenceId: 0,
        title: "TestRecurrenceGoal",
        criteria: {
            type: "taskbased",
            dat: { desc: "Do this recurring thing!" }
        },
        success: ["+5B", "Cake"],
        failure: ["-6B / hr, +1B / hr"],
        startTime: new Date(2024, 7, 15, 5, 30, 3),
        goalLengthSeconds: 108000,
        goalSpawnInterval: 201600,
        isDraft: false
    }];
}

function Timeline() {
    const [timelineStruct, setTimelineStruct] = useState({
        secondsPerPixel: 20,
        leftEdgeDate: new Date(2024, 7, 14, 2, 0, 0),
        rows: [
            [
                {
                    title: "TestGoal",
                    criteria: {
                        type: "event",
                        desc: "Attend this event"
                    },
                    success: ["+3B", "Candy"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 15, 5, 30, 3),
                    deadline: new Date(2024, 7, 18, 5, 30, 3),
                    isRecurrenceGhost: false
                },

                {
                    title: "TestGoal2",
                    criteria: {
                        type: "timebased",
                        desc: "Work on Paxos Research for 5 hours"
                    },
                    success: ["+30B", "Candy2"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 19, 5, 30, 3),
                    deadline: new Date(2024, 7, 20, 5, 30, 3),
                    isRecurrenceGhost: false
                }
            ],

            [
                {
                    title: "TestRecurrenceGoal",
                    criteria: {
                        type: "taskbased",
                        desc: "Do this goal!"
                    },
                    success: ["+3B", "Candy"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 15, 15, 30, 3),
                    deadline: new Date(2024, 7, 18, 15, 30, 3),
                    isRecurrenceGhost: false
                },

                {
                    title: "TestRecurrenceGhost",
                    criteria: {
                        type: "taskbased",
                        desc: "Do this goal NOW!"
                    },
                    success: ["+30B", "Candy2"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 19, 15, 30, 3),
                    deadline: new Date(2024, 7, 20, 15, 30, 3),
                    isRecurrenceGhost: true
                }
            ]
        ]
    });

    function fetchAndComputeTimeline() {
        let goals = fetchGoalData();
        let recurrences = fetchRecurrenceData();

        for (let rec in recurrences) {

        }
    }

    let parentRefs = timelineStruct.rows.map((goals, i) => (
        useRef(null)
    ));
    let childRefs = timelineStruct.rows.map((goals, i) => (
        goals.map((goal, j) => (
            useRef(null)
        ))
    ));

    let containerRef = useRef(null);
    let [containerWidth, setContainerWidth] = useState(0);

    function updateHeight() {
        for (let i = 0; i < parentRefs.length; i++) {
            let height = 0;
            for (let j = 0; j < childRefs[i].length; j++) {
                if (childRefs[i][j].current.offsetHeight > height) {
                    height = childRefs[i][j].current.offsetHeight;
                }
            }
            parentRefs[i].current.style.height = height + "px";
        }
    }

    function timeScale(event) {
        setTimelineStruct({...timelineStruct, secondsPerPixel: timelineStruct.secondsPerPixel * Math.pow(1.003, event.deltaY)});
    }

    let dragPosRecord = useRef(0);
    let isDragging = useRef(false);

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
        updateHeight();
        setContainerWidth(containerRef.current.offsetWidth);
    }

    useEffect(() => {
        resizeHandler();
        window.addEventListener("resize", resizeHandler);
        window.addEventListener("wheel", timeScale);
    });

    return (
        <div ref={containerRef} id="timelineContainer" onMouseDown={mouseDownHandler} onMouseUp={mouseUpHandler}
             onMouseMove={mouseMoveHandler}>
            {
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
                timelineStruct.rows.map((goals, i) => (
                    (
                        <div className="timelineRow" ref={parentRefs[i]}>
                            {
                                goals.map((goal, j) => {
                                    let goalStartMs = goal.startTime.getTime();
                                    let diffMs = goal.deadline.getTime() - goalStartMs;
                                    let leftEdgeMs = timelineStruct.leftEdgeDate.getTime();
                                    let diffPx = diffMs / (timelineStruct.secondsPerPixel * 1000);
                                    let goalStartPx = (goalStartMs - leftEdgeMs) / (timelineStruct.secondsPerPixel * 1000);
                                    return (
                                        <div className="goalBlockOuterContainer" ref={childRefs[i][j]}>
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
