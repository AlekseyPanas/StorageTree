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
        console.log("func called");
        for (let i = 0; i < parentRefs.length; i++) {
            let height = 0;
            for (let j = 0; j < childRefs[i].length; j++) {
                console.log(childRefs[i][j].current.offsetHeight);
                if (childRefs[i][j].current.offsetHeight > height) {
                    height = childRefs[i][j].current.offsetHeight;
                    console.log("test");
                }
            }
            parentRefs[i].current.style.height = height + "px";
        }
    }

    useEffect(() => {
        updateHeight();
        console.log("here");
        window.addEventListener("resize", updateHeight);
    });

    return (
        <div id="timelineContainer">
            {
                timelineStruct.rows.map((goals, i) => (
                    (
                        <div className="timelineRow" ref={parentRefs[i]}>
                            {
                                goals.map((goal, j) => (
                                    <div className="goalBlockOuterContainer" ref={childRefs[i][j]}>
                                        <GoalBlock
                                            key={i+","+j}
                                            goalTitle={goal.title}
                                            goalCriteria={goal.criteria}
                                            goalSuccess={goal.success}
                                            goalFailure={goal.failure}
                                            leftOffsetPx={j * 250}
                                            widthPx={(j + 1) * 40}
                                        ></GoalBlock>
                                    </div>
                                ))
                            }
                        </div>
                    )
                ))
            }

        </div>
    );
}

export default Timeline;
