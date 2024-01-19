import { writable } from 'svelte/store';
import { type ToolType } from "/src/communication/bindings";

export function createToolState() {
    interface ToolState {
      selectedTool: ToolType | null;
    };
  
    const initialToolState: ToolState = {
      selectedTool: null
    };
  
    const toolStore = writable<ToolState>(initialToolState);

    const tools = [
        { name: 'Tool1', icon: '/icons/tool1.svg' },
        { name: 'Tool2', icon: '/icons/tool2.svg' },
        { name: 'Tool3', icon: '/icons/tool3.svg' },
        { name: 'Tool4', icon: '/icons/tool4.svg' },
        { name: 'Tool5', icon: '/icons/tool5.svg' },
        { name: 'Tool6', icon: '/icons/tool6.svg' },
        { name: 'Tool7', icon: '/icons/tool7.svg' },
        { name: 'Tool8', icon: '/icons/tool8.svg' },
        { name: 'Tool9', icon: '/icons/tool9.svg' },
        { name: 'Tool10', icon: '/icons/tool10.svg' },
        { name: 'Tool11', icon: '/icons/tool11.svg' },
        { name: 'Tool12', icon: '/icons/tool12.svg' },
        { name: 'Tool13', icon: '/icons/tool13.svg' },
        { name: 'Tool14', icon: '/icons/tool14.svg' },
        { name: 'Tool15', icon: '/icons/tool15.svg' },
        { name: 'Tool16', icon: '/icons/tool16.svg' }
    ];
}