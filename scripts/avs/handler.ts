import axios from 'axios';

export const handler = async (input: string) => {
  try {
    const response = await axios.post(
      'https://api.groq.com/openai/v1/chat/completions',
      {
        model: 'llama3-8b-8192',
        messages: [
          { role: 'system', content: 'You are a helpful AVS inference model.' },
          { role: 'user', content: input }
        ]
      },
      {
        headers: {
          Authorization: `Bearer ${process.env.GROQ_API_KEY}`,
          'Content-Type': 'application/json'
        }
      }
    );

    const output = response.data.choices[0].message.content;

    return {
      status: 'success',
      result: output
    };
  } catch (err) {
    return {
      status: 'error',
      message: 'Groq API call failed',
      error: err instanceof Error ? err.message : err
    };
  }
};
