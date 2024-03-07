import { expect } from 'chai';
import { step } from 'mocha-steps';
import { AbiItem } from 'web3-utils';
import { signAndSend, describeLitentry, loadConfig, subscribeToEvents, sudoWrapper } from '../common/utils';
import Web3 from 'web3';
import precompileStakingContractAbi from '../common/abi/precompile/Staking.json';
import precompileBridgeContractAbi from '../common/abi/precompile/Bridge.json';
const BN = require('bn.js');
import { mnemonicGenerate, mnemonicToMiniSecret, evmToAddress } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';

const toBigNumber = (int: number) => int * 1e12;
const bn1e12 = new BN(10).pow(new BN(12)).mul(new BN(1));

describeLitentry('Test Parachain Precompile Contract', ``, (context) => {
    const config = loadConfig();

    const precompileStakingContractAddress = '0x000000000000000000000000000000000000502d';
    const precompileBridgeContractAddress = '0x000000000000000000000000000000000000503d';
    const evmAccountRaw = {
        privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
        address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
        mappedAddress: evmToAddress('0xaaafB3972B05630fCceE866eC69CdADd9baC2771', 31),
        publicKey: '0x93eac2793cb6d9e837b0f8da1a63dbc0db2ca848c05cbe66db139157922f78f9',
    };

    // candidate: collator address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    // transform to bytes32(public key) reference:https://polkadot.subscan.io/tools/format_transform?input=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY&type=All
    const collatorPublicKey = '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d';

    const web3 = new Web3(config.parachain_ws);

    const precompileStakingContract = new web3.eth.Contract(
        precompileStakingContractAbi as AbiItem[],
        precompileStakingContractAddress
    );
    const precompileBridgeContract = new web3.eth.Contract(
        precompileBridgeContractAbi as AbiItem[],
        precompileBridgeContractAddress
    );

    const executeTransaction = async (delegateTransaction: any, contractAddress: HexString, label = '') => {
        console.log(`=== Executing ${label} ===`);

        // estimate gas doesn't work
        // const gas = await delegateTransaction.estimateGas();
        // console.log("gas", gas);

        const transaction = await web3.eth.accounts.signTransaction(
            {
                to: contractAddress,
                data: delegateTransaction.encodeABI(),
                gas: 1000000,
            },
            evmAccountRaw.privateKey
        );

        return await web3.eth.sendSignedTransaction(transaction.rawTransaction!);
    };

    const printBalance = (label: string, bl: any) => {
        console.log(label, 'free', bl.free.toNumber() / 1e12, 'reserved', bl.reserved.toNumber() / 1e12);
    };

    // this function makes two transactions first one from token owner substrate account to owner evm mapped account
    // and then it transfers to recepient
    const transferTokens = async (from: KeyringPair, to: any) => {
        const aliceEVMMappedAccount = from.publicKey.slice(0, 20); // pretend to be evm
        console.log(`alice address: ${from.publicKey}`);
        console.log(`aliceEVMMappedAccount: ${aliceEVMMappedAccount}`);

        let aliceMappedSustrateAccount = evmToAddress(aliceEVMMappedAccount, 31);

        console.log('transfer from Alice to alice EMV');

        // Deposit money into substrate account's truncated EVM address's mapping substrate account
        const tx_init = context.api.tx.balances.transfer(aliceMappedSustrateAccount, 70 * 1e12);
        await signAndSend(tx_init, context.alice);

        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(
            aliceEVMMappedAccount, // evm like
            to.address, // evm like
            '0x',
            toBigNumber(65),
            1000000,
            25000,
            null,
            null,
            []
        );
        let block = await context.api.rpc.chain.getBlock();
        console.log(`evm call await before: ${block.block.header.number}`);
        await signAndSend(tx, from);
        let temp = await context.api.rpc.chain.getBlock();
        console.log(`evm call await end: ${temp.block.header.number}`);
    };

    const isPendingRequest = async () =>
        await precompileStakingContract.methods
            .delegationRequestIsPending(evmAccountRaw.publicKey, collatorPublicKey)
            .call();

    const collatorDetails = async () => {
        const response = await context.api.query.parachainStaking.autoCompoundingDelegations(collatorPublicKey);
        const collators = response.toJSON() as { address: string; value: number }[];
        return collators[0];
    };

    step('Address with not sufficient amount of tokens', async function () {
        // Create valid Substrate-compatible seed from mnemonic
        const randomSeed = mnemonicToMiniSecret(mnemonicGenerate());
        const secretKey = Buffer.from(randomSeed).toString('hex');

        const delegateWithAutoCompound = precompileStakingContract.methods.delegateWithAutoCompound(
            collatorPublicKey,
            toBigNumber(60),
            1
        );

        try {
            await web3.eth.accounts.signTransaction(
                {
                    to: precompileStakingContractAddress,
                    data: delegateWithAutoCompound.encodeABI(),
                    gas: await delegateWithAutoCompound.estimateGas(),
                },
                secretKey
            );
            expect(true).to.eq(false); // test should fail here
        } catch (e) {
            expect(e).to.be.instanceof(Error);
        }
    });

    // To see full params types for the interfaces, check notion page: https://web3builders.notion.site/Parachain-Precompile-Contract-0c34929e5f16408084446dcf3dd36006
    step('Test precompile staking contract', async function () {
        console.time('Test precompile staking contract');
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = await sudoWrapper(context.api, context.api.tx.extrinsicFilter.setMode('Test'));
            let temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await Before: ${temp.block.header.number}`);
            await signAndSend(extrinsic, context.alice);
            temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await end: ${temp.block.header.number}`);
        }
        let balance = (await context.api.query.system.account(evmAccountRaw.mappedAddress)).data;
        printBalance('initial balance', balance);

        // top up LITs if not sufficient amount for staking or they are not reserved (require: 50 LITs minimum)
        if (balance.free.toNumber() < toBigNumber(60) && balance.reserved.toNumber() === 0) {
            console.log('transferring more tokens');

            await transferTokens(context.alice, evmAccountRaw);

            balance = (await context.api.query.system.account(evmAccountRaw.mappedAddress)).data;
            printBalance('balance after transferring', balance);
        }

        //// TESTS
        const autoCompoundPercent = 20;

        // delegateWithAutoCompound(collator, amount, percent)
        const delegateWithAutoCompound = precompileStakingContract.methods.delegateWithAutoCompound(
            collatorPublicKey,
            toBigNumber(60),
            autoCompoundPercent
        );

        let afterDelegateBalance = balance;
        // skip test if already delegated
        if (balance.reserved.toNumber() === 0) {
            await executeTransaction(
                delegateWithAutoCompound,
                precompileStakingContractAddress,
                'delegateWithAutoCompound'
            );
            afterDelegateBalance = (await context.api.query.system.account(evmAccountRaw.mappedAddress)).data;

            expect(balance.free.toNumber() - toBigNumber(60)).to.closeTo(
                afterDelegateBalance.free.toNumber(),
                toBigNumber(1)
            );
            expect(afterDelegateBalance.reserved.toNumber()).to.eq(toBigNumber(60));
            const collator = await collatorDetails();
            expect(collator.value).to.eq(autoCompoundPercent);
        }

        // delegatorBondMore(collator, amount)
        const delegatorBondMore = precompileStakingContract.methods.delegatorBondMore(
            collatorPublicKey,
            toBigNumber(1)
        );
        await executeTransaction(delegatorBondMore, precompileStakingContractAddress, 'delegatorBondMore');

        const { data: balanceAfterBondMore } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        expect(balanceAfterBondMore.free.toNumber()).to.closeTo(
            balanceAfterBondMore.free.toNumber() - toBigNumber(1),
            toBigNumber(1)
        );
        expect(balanceAfterBondMore.reserved.toNumber()).to.eq(
            afterDelegateBalance.reserved.toNumber() + toBigNumber(1)
        );

        // setAutoCompound(collator, percent);
        const setAutoCompound = precompileStakingContract.methods.setAutoCompound(
            collatorPublicKey,
            autoCompoundPercent + 5
        );
        await executeTransaction(setAutoCompound, precompileStakingContractAddress, 'setAutoCompound');
        const collatorAfterCompound = await collatorDetails();
        expect(collatorAfterCompound.value).to.eq(autoCompoundPercent + 5);

        // scheduleDelegatorBondLess(collator, amount)
        expect(await isPendingRequest()).to.be.false;
        const scheduleDelegatorBondLess = precompileStakingContract.methods.scheduleDelegatorBondLess(
            collatorPublicKey,
            toBigNumber(5)
        );
        await executeTransaction(
            scheduleDelegatorBondLess,
            precompileStakingContractAddress,
            'scheduleDelegatorBondLess'
        );
        expect(await isPendingRequest()).to.be.true;

        // cancelDelegationRequest(collator)
        const cancelDelegationRequest = precompileStakingContract.methods.cancelDelegationRequest(collatorPublicKey);
        expect(await isPendingRequest()).to.be.true;
        await executeTransaction(cancelDelegationRequest, precompileStakingContractAddress, 'cancelDelegationRequest');
        expect(await isPendingRequest()).to.be.false;

        // testing bond less + execution
        await executeTransaction(
            scheduleDelegatorBondLess,
            precompileStakingContractAddress,
            'scheduleDelegatorBondLess again to test execution'
        );
        expect(await isPendingRequest()).to.be.true;

        console.log('Waiting 2 blocks before execute delegation request');
        await context.api.rpc.chain.getBlock();
        await context.api.rpc.chain.getBlock();

        // executeDelegationRequest(delegator, collator);
        const executeDelegationRequest = precompileStakingContract.methods.executeDelegationRequest(
            evmAccountRaw.publicKey,
            collatorPublicKey
        );
        await executeTransaction(
            executeDelegationRequest,
            precompileStakingContractAddress,
            'executeDelegationRequest'
        );
        const { data: balanceAfterBondLess } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        expect(balanceAfterBondLess.free.toNumber()).to.closeTo(
            balanceAfterBondMore.free.toNumber() + toBigNumber(5),
            toBigNumber(1)
        );
        expect(balanceAfterBondLess.reserved.toNumber()).to.eq(
            balanceAfterBondMore.reserved.toNumber() - toBigNumber(5)
        );

        // testing revoke delegation + execute
        // scheduleRevokeDelegation(collator);
        const scheduleRevokeDelegation = precompileStakingContract.methods.scheduleRevokeDelegation(collatorPublicKey);
        await executeTransaction(
            scheduleRevokeDelegation,
            precompileStakingContractAddress,
            'scheduleRevokeDelegation'
        );

        console.log('Waiting 2 blocks before execute delegation request');
        await context.api.rpc.chain.getBlock();
        await context.api.rpc.chain.getBlock();

        await executeTransaction(
            executeDelegationRequest,
            precompileStakingContractAddress,
            'executeDelegationRequest'
        );
        const { data: balanceAfterRevoke } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        expect(balanceAfterRevoke.free.toNumber()).to.closeTo(balance.free.toNumber(), toBigNumber(1));
        expect(balanceAfterRevoke.reserved.toNumber()).to.eq(0);

        // delegate(collator, amount);
        const delegate = precompileStakingContract.methods.delegate(collatorPublicKey, toBigNumber(57));
        await executeTransaction(delegate, precompileStakingContractAddress, 'delegate');
        const { data: balanceAfterDelegate } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        expect(balanceAfterDelegate.reserved.toNumber()).to.eq(toBigNumber(57));

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = await sudoWrapper(context.api, context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);

        console.timeEnd('Test precompile staking contract');
    });
    step('Test precompile bridge contract', async function () {
        console.time('Test precompile bridge contract');
        const dest_address = '0xaaafb3972b05630fccee866ec69cdadd9bac2772'; // random address
        let balance = (await context.api.query.system.account(evmAccountRaw.mappedAddress)).data;
        if (balance.free.toNumber() < toBigNumber(0.01)) {
            await transferTokens(context.alice, evmAccountRaw);
            expect(balance.free.toNumber()).to.gt(toBigNumber(0.01));
        }

        // update chain bridge fee
        const updateFeeTx = await sudoWrapper(context.api, context.api.tx.chainBridge.updateFee(0, bn1e12 / 1000));
        await signAndSend(updateFeeTx, context.alice);

        const bridge_fee = await context.api.query.chainBridge.bridgeFee(0);
        expect(bridge_fee.toString()).to.eq((bn1e12 / 1000).toString());

        // set chainId to whitelist
        const whitelistChainTx = await sudoWrapper(context.api, context.api.tx.chainBridge.whitelistChain(0));
        await signAndSend(whitelistChainTx, context.alice);

        // The above two steps are necessary, otherwise the contract transaction will be reverted.
        // transfer native token
        const transferNativeTx = precompileBridgeContract.methods.transferNative(
            bn1e12 / 100, // 0.01 LIT
            dest_address,
            0
        );

        const res = await executeTransaction(transferNativeTx, precompileBridgeContractAddress, 'transferNative');
        expect(res.status).to.eq(true);

        const eventsPromise = subscribeToEvents('chainBridge', 'FungibleTransfer', context.api);
        const events = (await eventsPromise).map(({ event }) => event);

        expect(events.length).to.eq(1);
        const event_data = events[0].toHuman().data! as Array<string>;

        // FungibleTransfer(BridgeChainId, DepositNonce, ResourceId, u128, Vec<u8>)
        expect(event_data[0]).to.eq('0');
        const destResourceId = context.api.consts.bridgeTransfer.nativeTokenResourceId.toHex();
        expect(event_data[2]).to.eq(destResourceId);
        expect(event_data[3]).to.eq((bn1e12 / 100 - bn1e12 / 1000).toLocaleString());
        expect(event_data[4]).to.eq(dest_address);

        console.timeEnd('Test precompile bridge contract');
    });
});
