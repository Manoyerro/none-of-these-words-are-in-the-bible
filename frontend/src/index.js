import { createRoot } from "react-dom";
import App from "./js/App";

const container = document.getElementById("root")

const root = createRoot(container)
root.render(<App/>)