import { expect } from 'chai';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';

describeLitentry('Test Base Filter', ``, (context) => {
    console.log(`Test Base Filter`);

    step('Transfer 1000 unit from Eve to Bob', async function () {
        // Get the initial balance of Eve and Bob
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        const tx = context.api.tx.balances.transfer(context.bob.address, 1000);
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        // the balance transfer should work for litmus|rococo but not for litentry
        if (context.parachain === 'litentry') {
            expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt());
        } else if (context.parachain === 'litmus' || context.parachain === 'rococo') {
            expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt() + BigInt(1000));
        } else {
            assert.fail('unsupported parachain type');
        }
    });

    step('Transfer 1000 unit from Eve to Bob with Sudo', async function () {
        // Get the initial balance of Alice and Bob
        const { nonce: aliceInitNonce, data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        // Force transfer 1000 unit from Eve to Bob, signed by Alice(sudo)
        const tx = context.api.tx.sudo.sudo(
            context.api.tx.balances.forceTransfer(context.eve.address, context.bob.address, 1000)
        );
        await signAndSend(tx, context.alice);

        const { nonce: aliceCurrentNonce, data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

        // The transfer should always succeed
        expect(aliceCurrentNonce.toNumber()).to.equal(aliceInitNonce.toNumber() + 1);
        expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt() + BigInt(1000));
    });
});
