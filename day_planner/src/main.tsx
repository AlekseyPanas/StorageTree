import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {TestStateLayer} from "./state_layers/test_state_layer";
import {Test_auth_layer} from "./auth_layers/test_auth_layer";

const AUTH_LAYER = new Test_auth_layer();
const STATE_LAYER = new TestStateLayer(AUTH_LAYER);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App stateLayer={STATE_LAYER} />
  </React.StrictMode>
);
