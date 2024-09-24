const { createStrategyForToken } = require('../utils/cli/commands/createStrategyForToken');
require('dotenv').config();

describe('commands', () => {
    describe('createStrategyForToken', () => {
        test('Successfully creates strategy for token', async () => {
            const chainId = 31337; 
            const config = {
                rpcUrl: 'http://localhost:8545',
                privateKey: process.env.PRIVATE_KEY,
                tokenAddress: ""
            };

            const [error, result] = await createStrategyForToken(config);
            
            expect(error).toBeNull();
            expect(result).not.toBeNull();
            
            const zeroAddress = '0x0000000000000000000000000000000000000000';
            expect(result).not.toBe(zeroAddress);
        });
    });
});
