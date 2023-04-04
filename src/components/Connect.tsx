import { useEffect, useState } from "react"
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api"

type ConnectProps = {
    onConnect: () => void;
    onDisconnect: () => void;
}

const Connect = (props: ConnectProps) => {
    const [enteredUrl, setEnteredUrl] = useState("")
    const [connected, setConnected] = useState(false)

    const urlChangeHandler = (event: any) => {
        setEnteredUrl(event.target.value)
    }

    const connect = () => {
        invoke("connect", { url: enteredUrl })
    }

    const disconnect = () => {
        invoke("disconnect", {})
    }

    useEffect(() => {
        const unlisten = listen("connected", () => {
            console.log("connected")
            setConnected(true)
            props.onConnect()
        })

        const unlistenDisconnect = listen("disconnected", () => {
            setConnected(false)
            props.onDisconnect()
        })

        return function cleanup() {
            unlisten.then(f => f())
            unlistenDisconnect.then(f => f())
        }
    }, [])

    return (<>
        <div>
            <label>URL:</label>
            <input type="text" value={enteredUrl} onChange={urlChangeHandler} />
            {
                connected &&
                (<button onClick={disconnect}>Disconnect</button>) ||
                (<button onClick={connect}>Connect</button>)
            }
        </div>
    </>)
}

export default Connect
