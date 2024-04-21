import { ApiPromise, WsProvider } from "@polkadot/api";
import { u32 } from "@polkadot/types";
import { EventRecord } from "@polkadot/types/interfaces";

const WEB_SOCKET = "ws://127.0.0.1:9944";

const connect = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}

// 订阅template pallet中的something的更新
const subscribeSomething = async (api: ApiPromise) => {
    api.query.templateModule.something((result: u32) => {
        console.log(`something is updated: ${result.toHuman()}`);
    })
}

// 订阅template pallet中的事件SomethingStored的更新
const subscribeSomethingStored = async (api: ApiPromise) => {
    await api.query.system.events((events: EventRecord[]) => {
        events.forEach(function (record) {
            const { event } = record;
            if (event.section == "templateModule" && event.method == "SomethingStored") {
                console.log(`Event SomethingStored is updated: ${event.data}`);
            }
        });
    })
}

const main = async () => {
    const api = await connect();

    await subscribeSomething(api);
    await subscribeSomethingStored(api);

    await new Promise((resolve) => setTimeout(resolve, 60000));
    api.disconnect();
}

main().then(() => {
    console.log("exits with success.");
    process.exit(0);
}).catch(err => {
    console.log(`error is ${err}`);
    process.exit(1);
})