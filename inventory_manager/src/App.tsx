import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Timeline from "./components/Timeline";

function App() {

  return (
    <div className="container">
      <Timeline> </Timeline>
    </div>
  );
}

export default App;
