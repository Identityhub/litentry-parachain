//run:npx ts-node setup-enclave.ts $enclaveAccount $mrenclave $accountPassword
//example:npx ts-node setup-enclave.ts 2KWd4sEmYj2VW42L2WUDDRKA4JwnKg76uoQ2keUBUwFHU9Dx a552654d1733c4054a3c7e5e86adf26b5d65c072b57b2550fe763821ebac54c6 123456
const { Keyring } = require("@polkadot/api");
import { initApi } from "./initApis";
const { hexToU8a } = require("@polkadot/util");
import colors from "colors";


//100 token
const transferAmount = "100000000000000";
const enclaveAccount = process.argv[2];
const mrenclave = process.argv[3];
const block = process.argv[4];

async function transfer(api: any, Alice: any) {

    console.log(colors.green("transfer start..."));
    return new Promise(async (resolve, reject) => {
        await api.tx.balances
            .transfer(enclaveAccount, transferAmount)
            .signAndSend(Alice, ({ status, events, dispatchError }) => {
                if (status.isInBlock || status.isFinalized) {
                    events.forEach(({ phase, event: { data, method, section } }) => {
                        if (method === "Transfer" && section === "balances") {
                            console.log(colors.green("transfer completed"));
                            resolve("transfer done");
                            return;
                        }
                    });
                }
            });
    });
}
async function setTeerexAdmin(api: any, Alice: any) {
    return new Promise(async (resolve, reject) => {
        // Note: The hardcoded address is that of Alice on dev chain 
        await api.tx.sudo
            .sudo(api.tx.teerex.setAdmin("esqZdrqhgH8zy1wqYh1aLKoRyoRWLFbX9M62eKfaTAoK67pJ5"))
            .signAndSend(Alice, ({ status, events, dispatchError }) => {
                if (status.isInBlock || status.isFinalized) {
                    if (dispatchError) {
                        if (dispatchError.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(
                                dispatchError.asModule
                            );
                            const { docs, name, section } = decoded;

                            console.log(colors.red(`${section}.${name}: ${docs.join(" ")}`));
                            reject("updateScheduledEnclave failed");
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            console.log(dispatchError.toString());
                            reject("updateScheduledEnclave failed");
                        }
                    } else {
                        console.log(colors.green("updateScheduledEnclave completed"));
                        resolve("updateScheduledEnclave done");
                    }
                }
            });
    });
}
async function updateScheduledEnclave(api: any, Alice: any, block: any) {
    return new Promise(async (resolve, reject) => {
        await api.tx.teerex.updateScheduledEnclave(block, hexToU8a(`0x${mrenclave}`))
            .signAndSend(Alice, ({ status, events, dispatchError }) => {
                if (status.isInBlock || status.isFinalized) {
                    if (dispatchError) {
                        if (dispatchError.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(
                                dispatchError.asModule
                            );
                            const { docs, name, section } = decoded;

                            console.log(colors.red(`${section}.${name}: ${docs.join(" ")}`));
                            reject("updateScheduledEnclave failed");
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            console.log(dispatchError.toString());
                            reject("updateScheduledEnclave failed");
                        }
                    } else {
                        console.log(colors.green("updateScheduledEnclave completed"));
                        resolve("updateScheduledEnclave done");
                    }
                }
            });
    });
}

async function main() {

    // NOTE: If we are using development environment, we can use Alice instead of moving around with keys
    const keyring = new Keyring({ type: "sr25519" });
    // let signAccount = keyring.addFromJson(private_account_pair);

    console.log(colors.green("account unlock..."));

    //unlock account
    // signAccount.toJson(accountPassword);

    const { defaultAPI } = await initApi();
    const Alice = keyring.addFromUri("//Alice", { name: "Alice default" });

    await transfer(defaultAPI, Alice);
    await setTeerexAdmin(defaultAPI, Alice);
    await updateScheduledEnclave(defaultAPI, Alice, block);

    console.log(colors.green("done"));
    process.exit();
}

main();
