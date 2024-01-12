import { useEffect, useState } from "react";
import { createInputManager } from "./io-managers/input";
import * as Slider from '@radix-ui/react-slider';
import "./main.css";

const Panel = ({ title, children }) => {
  return (
    <div className="bg-secondary p-4 rounded-lg shadow-md">
      {title && <h2 className="text-text font-semibold mb-2">{title}</h2>}
      {children}
    </div>
  );
};

const Setting = ({ label, children }) => {
  return (
    <div className="mb-4 last:mb-0">
      {label && <label className="text-textSecondary block mb-1">{label}</label>}
      {children}
    </div>
  );
};

const SliderDemo = ({ label, value, onChange }) => {
  return (
    <Setting label={label}>
      <Slider.Root
        className="relative flex items-center select-none touch-none w-[200px] h-5"
        defaultValue={[50]}
        max={100}
        step={1}
      >
        <Slider.Track className="bg-textSecondary relative grow rounded-full h-[3px]">
          <Slider.Range className="absolute bg-accent rounded-full h-full" />
        </Slider.Track>
        <Slider.Thumb
        className="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
        aria-label="Volume"
      />
      </Slider.Root>
    </Setting>
  );
};

const Toggle = ({ label, enabled, onToggle }) => {
  return (
    <Setting label={label}>
      <button
        onClick={onToggle}
        className={`w-12 h-6 rounded-full shadow-inner ${enabled ? 'bg-toggleOn' : 'bg-toggleOff'}`}
      >
        <span
          className={`block w-4 h-4 bg-white rounded-full shadow transform transition-transform ${
            enabled ? 'translate-x-6' : 'translate-x-0'
          }`}
        />
      </button>
    </Setting>
  );
};

const managerDestructors: {
  createInputManager?: () => void;
} = {};

function App() {
  const [contrast, setContrast] = useState(50);
  const [isRayModeEnabled, setIsRayModeEnabled] = useState(false);

  useEffect(() => {});

  return (
    <div className="container">
      <Panel title="Image Settings">
        <SliderDemo label="Contrast" value={contrast} onChange={(e) => setContrast(e.target.value)} />
        <Toggle label="Ray Mode" enabled={isRayModeEnabled} onToggle={() => setIsRayModeEnabled(!isRayModeEnabled)} />
      </Panel>
    </div>
  );
}

export default App;
