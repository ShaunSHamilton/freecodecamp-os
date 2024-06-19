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
    try {
      await invoke("check_server");
      await invoke("stop_server");
      setBtnText("Start Server");
    } catch (e) {
      await invoke("start_server");
      setBtnText("Stop Server");
    }
    if (btn) {
      btn.disabled = false;
    }
  }

  useEffect(() => {
    // Check if server is already running
    (async () => {
      try {
        await invoke("check_server");
        setBtnText("Stop Server");
      } catch (e) {
        setBtnText("Start Server");
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
