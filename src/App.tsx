import { invoke } from "@tauri-apps/api";
import { connected } from "process";
import { useState } from "react";
import "./App.css";

function App() {
    const [isConnected, setIsConnected] = useState(false)
    const [enteredUrl, setEnteredUrl] = useState("")
    const [events, setEvents] = useState([] as string[])

    const urlChangeHandler = (event: any) => {
        setEnteredUrl(event.target.value)
    }

    const connect = () => {
        console.log(enteredUrl)
        setEvents((prevStatus: string[]) => {
            return [...prevStatus, "Attempting to connect: ws://" + enteredUrl]
        })
        invoke("connect", { url: enteredUrl })

        setIsConnected(true)
    }

    const disconnect = () => {
        setEvents((prevState: string[]) => {
            return [...prevState, "Disconnected"]
        })
        setIsConnected(false)
    }

    const clearEvents = () => {
        setEvents([])
    }

    return (
        <div className="container">
            <div>
                <label>URL:</label>
                <input type="text" value={enteredUrl} onChange={urlChangeHandler} />
                {
                    isConnected &&
                    (<button onClick={disconnect}>Disconnect</button>) ||
                    (<button onClick={connect}>Connect</button>)
                }
                <button onClick={clearEvents}>Clear Events</button>
            </div>
            {events.map((event: string) => (<p>{event}</p>))}
        </div>
    );
}

export default App;
