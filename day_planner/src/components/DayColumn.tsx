import {Component} from "react";
import {GoalBlock} from "./GoalBlock";
import {Goal} from "../state_layers/state_layer_interface";

type DayColumnProps = {
    hourRange: [number, number];
    goals: Goal[];
    style: object;
    className: string;
}

export class DayColumn extends Component<DayColumnProps> {
    render() {
        return <div className={this.props.className} style={{...{
            position: "relative",
            margin: 0,
            padding: 0,
            backgroundColor: 'rgb(232,232,232)'
        }, ...this.props.style}}>
            <div className="day-column-title-container" style={{
                height: "5%",
                outline: "1px solid",
                textAlign: "center",
                fontFamily: "sans-serif"
            }}>
                <h2 style={{padding: 0, margin: 0}}> Sun</h2>
                <h3 style={{padding: 0, margin: 0}}> 9/12</h3>
            </div>
            <div className="day-column-time-container"
                 style={{position: "relative", height: '75%', outline: "1px solid"}}>

                {/* Creates the horizontal lines for each hour block */}
                {Array.from(Array(this.props.hourRange[1] - this.props.hourRange[0] + 1).keys()).map((i) => {
                    const total = this.props.hourRange[1] - this.props.hourRange[0];
                    return <div key={i} style={{
                        width: '100%', height: '1px', position: 'absolute',
                        top: ((i / total) * 100).toString() + "%", left: 0, backgroundColor: "rgb(184,184,184)"
                    }}></div>;
                })}

                {
                    this.props.goals.map((goal, i) => {
                        return <GoalBlock goal={goal}/>
                    })
                }

            </div>
            <div className="day-column-overflow-container"
                 style={{position: "relative", height: "20%", outline: "1px solid"}}>

            </div>

        </div>
    }
}
