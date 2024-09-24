const { createStrategyForToken } = require('../utils/cli/commands/createStrategyForToken');
const { getTokenAddress } = require('../utils/cli/helpers/utils');
require('dotenv').config();

describe('COMMANDS', () => {
    describe('createStrategyForToken', () => {
        test('Successfully creates strategy for token', async () => {
            const chainId = 31337; // Local Anvil chain ID
            const tokenAddress = await getTokenAddress(chainId);
            const config = {
                rpcUrl: 'http://localhost:8545',
                privateKey: process.env.PRIVATE_KEY,
                tokenAddress: tokenAddress
            };

            const [error, result] = await createStrategyForToken(config);
            
            expect(error).toBeNull();
            expect(result).not.toBeNull();
            
            const zeroAddress = '0x0000000000000000000000000000000000000000';
            expect(result).not.toBe(zeroAddress);
        });
    });
});
