import { useEffect } from "react";
import { createInputManager } from "./io-managers/input";
import "./main.css";

const managerDestructors: {
  createInputManager?: () => void;
} = {};

function App() {

  useEffect(() => {});

  return (
    <div className="container">
        <button className="bg-green-500 text-white p-2.5 w-fit mt-9">
          Get Started
       </button>  
    </div>
  );
}

export default App;
