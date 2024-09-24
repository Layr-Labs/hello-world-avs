const { createStrategyForToken } = require('../utils/cli/commands/createStrategyForToken');

describe('COMMANDS', () => {
    describe('createStrategyForToken', () => {
        test('Successfully creates strategy for token', async () => {
            const [error, result] = await createStrategyForToken();
            expect(error).toBeNull();
            expect(result).not.toBeNull();
            
            const zeroAddress = '0x0000000000000000000000000000000000000000';
            expect(result).not.toBe(zeroAddress);
        });
    });

});
