import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import "@polkadot/api-augment";
import type { FrameSystemAccountInfo } from "@polkadot/types/lookup";
import { KeyringPair } from "@polkadot/keyring/types";

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));
const WEB_SOCKET = "ws://127.0.0.1:9944";

const con = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}

const getFreeBalance = async (api: ApiPromise, addr: string) => {
    const { data: { free, }, }: FrameSystemAccountInfo = await api.query.system.account(addr);
    return free;
}

const transfer = async (api: ApiPromise, alice: KeyringPair, bob: KeyringPair, amount: number) => {
    await api.tx.balances.transfer(bob.address, amount).signAndSend(alice, res => {
        console.log(`tx status: ${res.status}`);
    });
}

const main = async () => {
    const api = await con();
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    const old_balance = await getFreeBalance(api, bob.address);
    console.log(`bob balance is: ${old_balance.toHuman()}`);

    await transfer(api, alice, bob, 5);
    console.log('transfering...');
    await sleep(10000);

    const new_balance = await getFreeBalance(api, bob.address);
    console.log(`bob balance is: ${new_balance.toHuman()}`);
}

main().then(() => {
    console.log("exits with success.");
    process.exit(0);
}).catch(err => {
    console.log(`error is ${err}`);
    process.exit(1);
})