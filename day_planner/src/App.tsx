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
    const [appDataState, setAppDataState] = useState(stateLayer.get_current_state().data);

    // Subscribe to receive state updates
    useEffect(() => {
        stateLayer.on_state_change((newState: AppState) => { setAppDataState(newState); })
    }, []);

    return (
    <div className="day-view-container" style={{display: "flex", width: '100vw', height: '100vh', margin: 0, padding: 0}}>
        <div className="makeup-col" style={{width:'20%', height:'100%', margin: 0, padding: 0, backgroundColor: 'rgb(232,221,221)'}}>

        </div>

        <div className="main-grid" style={{width:'80%', height:'100%', margin: 0, padding: 0}}>
            <div className="main-grid-top-half" style={{display: "flex", height: '70%'}}>
                <div className="main-grid-left-bar" style={{width:'10%', height:'100%', margin: 0, padding: 0, backgroundColor: 'rgb(179,179,179)'}}>

                </div>
                <div className="main-grid-day-col" style={{height: '100%', margin: 0, padding: 0, backgroundColor: 'rgb(232,232,232)'}}>
                    <div className="goal-block">
                        Goal Name
                    </div>
                </div>
            </div>

            <div className="main-grid-bottom-half" style={{width: '100%', height: '30%', margin: 0, padding: 0, backgroundColor: 'rgb(210,218,218)'}}>
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
