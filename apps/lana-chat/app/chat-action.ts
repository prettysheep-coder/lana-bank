"use server";

import OpenAI from "openai";
import { tools, runFunction } from "./api/chat/functions";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
});

export async function sendChatMessage(messages: ChatCompletionMessageParam[]) {
  try {
    const completion = await openai.chat.completions.create({
      model: "gpt-4o-mini",
      messages,
      tools,
    });

    const choice = completion.choices[0];

    if (choice.message.tool_calls?.[0]) {
      const toolCall = choice.message.tool_calls[0];
      const args = JSON.parse(toolCall.function.arguments);
      const functionResult = await runFunction(toolCall.function.name, args);

      messages.push(choice.message);
      messages.push({
        role: "tool",
        tool_call_id: toolCall.id,
        content: JSON.stringify(functionResult),
      });

      const secondCompletion = await openai.chat.completions.create({
        model: "gpt-4o-mini",
        messages,
        tools,
      });

      return secondCompletion.choices[0].message;
    }

    return choice.message;
  } catch (error) {
    console.error("Error:", error);
    throw new Error("Failed to process chat message");
  }
}
