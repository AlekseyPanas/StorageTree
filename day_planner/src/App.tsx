import {useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {AppState, IStateLayerAdapter} from "./state_layers/state_layer_interface";

type AppProps = {
    stateLayer: IStateLayerAdapter
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

    return (
    <div className="day-view-container" style={{display: "flex", width: '100vw', height: '100vh', margin: 0, padding: 0}}>
        <div className="makeup-col" style={{width:'20%', height:'100%', margin: 0, padding: 0, backgroundColor: 'rgb(232,221,221)'}}>

        </div>

        <div className="main-grid" style={{width:'80%', height:'100%', margin: 0, padding: 0}}>

            <div className="main-grid-top-half" style={{display: "flex", height: '70%'}}>

                <div className="main-grid-left-bar" style={{width: '10%', height: '100%', margin: 0, padding: 0, backgroundColor: 'rgb(179,179,179)'}}>
                    <div className="main-grid-day-title-container" style={{height: "5%", outline: "1px solid"}}>

                    </div>
                    <div className="main-grid-day-time-container" style={{position: "relative", height:'75%', outline: "1px solid"}}>
                        {Array.from(Array(hourRange[1] - hourRange[0] + 1).keys()).map((i) => {
                            const total = hourRange[1] - hourRange[0];
                            return <p key={i} style={{width:'100%', position: 'absolute',
                                top: ((i / total) * 100).toString() + "%", left: 0, transform: "translateY(-50%)", color: "black", padding: 0, margin: 0, textAlign: 'right'}}>
                                {i + hourRange[0] + ":00"}
                            </p>;
                        })}
                    </div>
                    <div className="main-grid-day-overflow-container" style={{height: "20%", outline: "1px solid"}}>

                    </div>
                </div>

                <div className="main-grid-days-container" style={{display: "flex", width: "90%", justifyContent: "space-between"}}>
                    <div className="main-grid-day-col" style={{
                        flex: 1,
                        position: "relative",
                        height: '100%',
                        margin: 0,
                        padding: 0,
                        backgroundColor: 'rgb(232,232,232)'
                    }}>
                        <div className="main-grid-day-title-container" style={{
                            height: "5%",
                            outline: "1px solid",
                            textAlign: "center",
                            fontFamily: "sans-serif"
                        }}>
                            <h2 style={{padding: 0, margin: 0}}> Sun</h2>
                            <h3 style={{padding: 0, margin: 0}}> 9/12</h3>
                        </div>
                        <div className="main-grid-day-time-container" style={{position: "relative", height: '75%', outline: "1px solid"}}>

                            {Array.from(Array(hourRange[1] - hourRange[0] + 1).keys()).map((i) => {
                                const total = hourRange[1] - hourRange[0];
                                return <div key={i} style={{width:'100%', height:'1px', position: 'absolute',
                                    top: ((i / total) * 100).toString() + "%", left: 0, backgroundColor: "rgb(184,184,184)"}}></div>;
                            })}

                            <div className="goal-block" style={{
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
                                left: '50%',
                                transform: 'translateX(-50%)'
                            }}>
                                Goal Name
                            </div>


                        </div>
                        <div className="main-grid-day-overflow-container" style={{position: "relative", height: "20%", outline: "1px solid"}}>

                        </div>

                    </div>
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
