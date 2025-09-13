import {Component} from "react";
import {Goal} from "../state_layers/state_layer_interface";

type GoalBlockProps = {
    goal: Goal;
}

export class GoalBlock extends Component<GoalBlockProps> {
    render() {
        return <div className="goal-block" style={{
            backgroundColor: "rgb(168,244,208)",
            paddingLeft: "5px",
            paddingRight: "5px",
            borderRadius: "5px",
            fontFamily: "sans-serif",
            height: "25px",
            overflow: "hidden",
            borderTop: "solid 2px",
            borderBottom: "dashed 1px",
            borderRight: "dashed 1px",
            borderLeft: "dashed 1px",
            position: "absolute",
            top: "5%",
            left: "50%",
            transform: "translateX(-50%)"
        }}>
            Goal Name
        </div>;
    }
}
