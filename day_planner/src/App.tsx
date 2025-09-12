import {useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

    const [state, setState] = useState([]);

    useEffect(() => {

    }, []);

    return (
    <div className="container">
        <h1> TEST </h1>
    </div>
    );
}

export default App;
