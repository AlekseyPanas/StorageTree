import "../styles/GoalBlock.css"

function GoalBlock({goalTitle, goalCriteria, goalSuccess, goalFailure, leftOffsetPx, widthPx}) {
    return (
        <div className="goalBlock" style={{width: widthPx, left: leftOffsetPx}}>
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
