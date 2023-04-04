import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react";
import "./App.css";

type WebSocketEvent = {
    incoming: boolean;
    message: string;
}

function App() {
    const [isConnected, setIsConnected] = useState(false)
    const [enteredUrl, setEnteredUrl] = useState("")
    const [events, setEvents] = useState([] as WebSocketEvent[])

    const [message, setMessage] = useState("")

    const urlChangeHandler = (event: any) => {
        setEnteredUrl(event.target.value)
    }

    const connect = () => {
        console.log(enteredUrl)
        setEvents((prevStatus: WebSocketEvent[]) => {
            const newEvent: WebSocketEvent = {
                incoming: false,
                message: "Attempting to connect: " + enteredUrl
            }
            return [...prevStatus, newEvent]
        })
        invoke("connect", { url: enteredUrl })

        setIsConnected(true)
    }

    const disconnect = () => {
        setEvents((prevState: WebSocketEvent[]) => {
            const newEvent: WebSocketEvent = {
                incoming: false,
                message: "Disconnecting"
            }
            return [...prevState, newEvent]
        })
        setIsConnected(false)
    }

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

    const isIncoming = (event: WebSocketEvent) => {

        return event.incoming ? "incoming" : ""
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
