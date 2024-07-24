import "../styles/Timeline.css"
import GoalBlock from "./GoalBlock";
import {useEffect, useRef, useState} from "react";
import {i} from "@tauri-apps/api/fs-6ad2a328";

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
        title: "TestGoal",
        criteria: "Do this goal!",
        success: ["+3B", "Candy"],
        failure: ["-3B / hr, +2B / hr"],
        startTime: new Date(2024, 7, 15, 5, 30, 3),
        deadline: new Date(2024, 7, 18, 5, 30, 3),
        isDraft: false
    }];
}

function fetchRecurrenceData() {
    return [{
        title: "TestRecurrenceGoal",
        criteria: "Do this recurring thing!",
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
        secondsPerPixel: 2000,
        leftEdgeDate: new Date(2024, 7, 14, 2, 0, 0),
        rows: [
            [
                {
                    title: "TestGoal",
                    criteria: "Do this goal!",
                    success: ["+3B", "Candy"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 15, 5, 30, 3),
                    deadline: new Date(2024, 7, 18, 5, 30, 3),
                    isRecurrenceGhost: false
                },

                {
                    title: "TestGoal2",
                    criteria: "Do this goal NOW!",
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
                    criteria: "Do this goal!",
                    success: ["+3B", "Candy"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 15, 15, 30, 3),
                    deadline: new Date(2024, 7, 18, 15, 30, 3),
                    isRecurrenceGhost: false
                },

                {
                    title: "TestRecurrenceGhost",
                    criteria: "Do this goal NOW!",
                    success: ["+30B", "Candy2"],
                    failure: ["-3B / hr, +2B / hr"],
                    startTime: new Date(2024, 7, 19, 15, 30, 3),
                    deadline: new Date(2024, 7, 20, 15, 30, 3),
                    isRecurrenceGhost: true
                }
            ]
        ]
    });

    let parentRefs = timelineStruct.rows.map((goals, i) => (
        useRef(null)
    ));
    let childRefs = timelineStruct.rows.map((goals, i) => (
        goals.map((goal, j) => (
            useRef(null)
        ))
    ));

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
        setTimelineStruct({...timelineStruct, secondsPerPixel: timelineStruct.secondsPerPixel + event.deltaY});
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
            let newLeftEdge = new Date(0);
            newLeftEdge.setUTCMilliseconds(
                timelineStruct.leftEdgeDate.getTime() - (diffPx * timelineStruct.secondsPerPixel * 1000)
            );
            setTimelineStruct({...timelineStruct, leftEdgeDate: newLeftEdge})
            dragPosRecord.current = event.screenX;
        }
    }

    function mouseUpHandler(event) {
        event.preventDefault();
        isDragging.current = false;
    }

    useEffect(() => {
        updateHeight();
        window.addEventListener("resize", updateHeight);
        window.addEventListener("wheel", timeScale);
    });

    return (
        <div id="timelineContainer" onMouseDown={mouseDownHandler} onMouseUp={mouseUpHandler} onMouseMove={mouseMoveHandler}>
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
                                                goalCriteria={goal.criteria}
                                                goalSuccess={goal.success}
                                                goalFailure={goal.failure}
                                                leftOffsetPx={goalStartPx}
                                                widthPx={diffPx}
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
