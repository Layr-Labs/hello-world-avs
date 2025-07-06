import 'dotenv/config';
import { handler } from '../avs/handler';

const main = async () => {
  const input = 'What is the meaning of life?'; 
  const result = await handler(input);
  console.log('✅ Handler result:');
  console.log(result);
};

main();
