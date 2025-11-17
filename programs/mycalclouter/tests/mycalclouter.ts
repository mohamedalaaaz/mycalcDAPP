const assert = require('assert');
const mycalclouter = require('@project-serum/anchor');

const {SystemProgram} = anchor.web3;
describe('mycalclouter', () => {
    const provider = anchor.provider.local()
    anchor.setProvider(provider);
    const calculator = anchor.web3.Keypair.generate();

    const program = anchor.workspace.Mycalclouter;

    it('Creates a calclouter', async() => {
        await program.rpc.create("hello solana!", {
            accounts: {
                calclouter: calculator.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [calculator],
        });
        const account = await program.account.calclouter.fetch(calculator.publicKey);
        assert.ok(account.greeting === "hello solana!");
        
    });
    it('add two numbers', async() => {
        await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
            accounts: {
                calclouter: calculator.publicKey,
            },
        });
        const account = await program.account.calclouter.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(5)));

    });
    it(' subtract two number ',async() => {
        await program.rpc.subtract(new anchor.BN(7), new anchor.BN(5), {
            accounts: {
                calclouter: calculator.publicKey,
            },
        });
        const account = await program.account.calclouter.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(2)));     
        
    });
    it(' multible ',async() => {
        await program.rpc.multiply(new anchor.BN(3), new anchor.BN(4), {
            accounts: {
                calclouter: calculator.publicKey,       
            },
        });
        const account = await program.account.calclouter.fetch(calculator.publicKey);
        assert.ok(account.result.eq(new anchor.BN(12)));
        
    });

});

