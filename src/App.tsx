import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react";
import "./App.css";
import Connect from "./components/Connect";

type WebSocketEvent = {
    incoming: boolean;
    message: string;
}

function App() {
    const [events, setEvents] = useState([] as WebSocketEvent[])
    const [message, setMessage] = useState("")

    const clearEvents = () => {
        setEvents([])
    }

    const messageChangeHandler = (event: any) => {
        setMessage(event.target.value)
    }

    const sendMessage = () => {
        setEvents((prevState: WebSocketEvent[]) => {
            const newEvent: WebSocketEvent = {
                incoming: false,
                message: message
            }
            return [...prevState, newEvent]
        })
        invoke("send_message", { message: message })
    }

    useEffect(() => {
        const unlisten = listen("message_read", (event: any) => {
            const message = event.payload.inner;
            setEvents((prevState: WebSocketEvent[]) => {
                const newEvent: WebSocketEvent = {
                    incoming: true,
                    message: message
                }
                return [...prevState, newEvent]
            })
        })

        return () => {
            unlisten.then(f => f())
        }
    }, [])

    const onConnect = () => {
        setEvents((prevState: WebSocketEvent[]) => {
            const newEvent: WebSocketEvent = {
                incoming: false,
                message: "Connected"
            }
            return [...prevState, newEvent]
        })

    }

    const onDisconnect = () => {
        setEvents((prevState: WebSocketEvent[]) => {
            const newEvent: WebSocketEvent = {
                incoming: false,
                message: "Disconnected"
            }
            return [...prevState, newEvent]
        })
    }


    const isIncoming = (event: WebSocketEvent) => {

        return event.incoming ? "incoming" : ""
    }

    return (
        <div className="container">
            <Connect onConnect={onConnect} onDisconnect={onDisconnect} />
            <div>
                <label>Message:</label>
                <input type="text" value={message} onChange={messageChangeHandler} />
                <button onClick={sendMessage}>Send</button>
            </div>
            {events.map((event: WebSocketEvent) => (<p className={`${isIncoming(event)}`}>{event.message}</p>))}
        </div>
    );
}

export default App;
