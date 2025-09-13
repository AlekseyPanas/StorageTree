import {Component, useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {AppState, Goal, IStateLayerAdapter} from "./state_layers/state_layer_interface";
import {DayColumn} from "./components/DayColumn";

type DayViewGoalStruct = {
    visibleDayMap: Map<Date, Goal[]>;
    visibleSpanningGoals: Goal[];
}

type AppProps = {
    stateLayer: IStateLayerAdapter
}

function generate_view_struct(start_date: Date, days_shown: number, hour_range: [number, number], app_state: AppState | null): DayViewGoalStruct {
    if (app_state === null)
        return {visibleDayMap: new Map(), visibleSpanningGoals: []}
    return {visibleDayMap: new Map(), visibleSpanningGoals: []}
}





function App({ stateLayer }: AppProps) {

    // TODO: Add logic to react to errors of not being logged in, license expiry, unauthorized feature
    // TODO: The app will not track different users. If a user was logged in and the app quits, it overrides data in sqlite for the latest user
    //      and if gone offline, it checks the existence of a token and assumes the user is the latest one logged in

    // Grab initial core app state
    const [appDataState, setAppDataState] = useState<AppState | null>(stateLayer.get_current_state().data);

    // Subscribe to receive state updates
    useEffect(() => {
        stateLayer.on_state_change((newState: AppState) => { setAppDataState(newState); })
    }, []);

    // State related only to the view
    const [hourRange, setHourRange] = useState<[number, number]>([7, 24]);
    const [startDate, setStartDate] = useState<Date>(new Date(2025, 8, 12));
    const [daysShown, setDaysShown] = useState<number>(3);
    const [dayViewStruct, setDayViewStruct] = useState<DayViewGoalStruct>(generate_view_struct(startDate, daysShown, hourRange, appDataState));

    useEffect(() => {
        setDayViewStruct(generate_view_struct(startDate, daysShown, hourRange, appDataState));
    }, [daysShown, startDate, hourRange]);

    return (
    <div className="day-view-container"
         style={{display: "flex", width: '100vw', height: '100vh', margin: 0, padding: 0}}>

        <div className="makeup-col"
             style={{width: '20%', height: '100%', margin: 0, padding: 0, backgroundColor: 'rgb(232,221,221)'}}>

        </div>

        <div className="main-grid" style={{width: '80%', height: '100%', margin: 0, padding: 0}}>

            <div className="main-grid-top-half" style={{display: "flex", height: '70%'}}>

                <div className="main-grid-left-bar"
                     style={{width: '10%', height: '100%', margin: 0, padding: 0, backgroundColor: 'rgb(179,179,179)'}}>
                    <div className="main-grid-day-title-container" style={{height: "5%", outline: "1px solid"}}>

                    </div>
                    <div className="main-grid-day-time-container"
                         style={{position: "relative", height: '75%', outline: "1px solid"}}>
                        {Array.from(Array(hourRange[1] - hourRange[0] + 1).keys()).map((i) => {
                            const total = hourRange[1] - hourRange[0];
                            return <p key={i} style={{
                                width: '100%',
                                position: 'absolute',
                                top: ((i / total) * 100).toString() + "%",
                                left: 0,
                                transform: "translateY(-50%)",
                                color: "black",
                                padding: 0,
                                margin: 0,
                                textAlign: 'right'
                            }}>
                                {i + hourRange[0] + ":00"}
                            </p>;
                        })}
                    </div>
                    <div className="main-grid-day-overflow-container" style={{height: "20%", outline: "1px solid"}}>

                    </div>
                </div>

                <div className="main-grid-days-container"
                     style={{display: "flex", width: "90%", justifyContent: "space-between"}}>

                    {
                        Array.from(Array(daysShown).keys()).map((i) => {
                            return <DayColumn hourRange={hourRange} goals={[]} className={"main-grid-day-col"} style={{flex: 1, height: '100%'}} />
                        })
                    }

                </div>

            </div>

            <div className="main-grid-bottom-half"
                 style={{width: '100%', height: '30%', margin: 0, padding: 0, backgroundColor: 'rgb(210,218,218)'}}>
                <div className="main-grid-left-bar">

                </div>

                <div className="main-grid-spanning-container">

                </div>

            </div>
        </div>
    </div>
    );
}

export default App;
