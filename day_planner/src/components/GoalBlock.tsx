import "../styles/GoalBlock.css"

function GoalBlock({goalTitle, goalCriteria, goalSuccess, goalFailure,
                       leftOffsetPx, widthPx, goalType, goalColor, isGhost}) {
    return (
        <div className="goalBlock" style={
            (() => {
                let borderStyle;
                let col = goalColor;
                let opacity = "1";
                if (goalType == "event") {
                    borderStyle = "solid";
                    col = "rgba(255, 255, 0, 1)"
                } else if (goalType == "timebased") {
                    borderStyle = "dashed";
                } else if (goalType == "taskbased") {
                    borderStyle = "dotted";
                }
                if (isGhost) { opacity = "0.35"; }
                return {
                    width: widthPx,
                    left: leftOffsetPx,
                    backgroundColor: col,
                    borderStyle: borderStyle,
                    borderColor: "black",
                    borderWidth: "3px",
                    opacity: opacity
                }
            })()

        }>
            <h4 className="goalBlockTitle"> {goalTitle} </h4>
            <div className="goalBlockFlex">
                <div className="goalBlockFlexItem goalBlockCriteriaContainer">
                    <p> {goalCriteria} </p>
                </div>
                <div className="goalBlockFlexItem goalBlockSuccessContainer">
                    <ul>
                        {goalSuccess.map((successItem, index) => (
                            <li> {successItem} </li>
                        ))}
                    </ul>
                </div>
                <div className="goalBlockFlexItem goalBlockFailureContainer">
                    <ul>
                        {goalFailure.map((failItem, index) => (
                            <li> {failItem} </li>
                        ))}
                    </ul>
                </div>
            </div>
        </div>
    );
}

export default GoalBlock;
