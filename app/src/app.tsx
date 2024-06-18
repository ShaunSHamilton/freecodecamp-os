import { createRef, useEffect, useState } from "react";
import "./index.css";
import { invoke } from "@tauri-apps/api/core";

export function App() {
  const [btnText, setBtnText] = useState("Start Server");
  const btnRef = createRef<HTMLButtonElement>();

  async function handleClick() {
    const btn = btnRef.current;
    if (btn) {
      btn.disabled = true;
    }
    setBtnText("...");
    const serverStatus = await invoke("check_server");
    if (serverStatus) {
      const res = await invoke("stop_server");
      console.log(res);
      setBtnText("Start Server");
    } else {
      const res = await invoke("start_server");
      console.log(res);
      setBtnText("Stop Server");
    }
    if (btn) {
      btn.disabled = false;
    }
  }

  useEffect(() => {
    // Check if server is already running
    (async () => {
      const res = await invoke("check_server");
      console.log(res);
      if (!res) {
        setBtnText("Stop Server");
      }
    })();
  }, []);

  return (
    <div className="container">
      <button ref={btnRef} onKeyDown={handleClick}>
        {btnText}
      </button>
    </div>
  );
}
