import axios from 'axios';
import 'dotenv/config';

const GROQ_API_KEY = process.env.GROQ_API_KEY || '';

export async function askGemma(prompt: string): Promise<string> {
  try {
    const res = await axios.post(
      'https://api.groq.com/openai/v1/chat/completions',
      {
        model: 'gemma-7b-it',
        messages: [
          {
            role: 'system',
            content: 'Eres un verificador lógico que responde sí o no según la validez del caso.',
          },
          {
            role: 'user',
            content: prompt,
          },
        ],
        temperature: 0.2,
        max_tokens: 50,
      },
      {
        headers: {
          Authorization: `Bearer ${GROQ_API_KEY}`,
          'Content-Type': 'application/json',
        },
      }
    );

    return res.data.choices[0].message.content.trim();
  } catch (error: any) {
    console.error('❌ Error al consultar Gemma:', error?.response?.data || error.message);
    return 'error';
  }
}
