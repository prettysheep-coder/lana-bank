import OpenAI from "openai";
import { tools, runFunction } from "./functions";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
});

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const messages = body.messages as ChatCompletionMessageParam[];

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

      return Response.json(secondCompletion.choices[0].message);
    }

    return Response.json(choice.message);
  } catch (error) {
    console.error("Error:", error);
    return new Response("Error processing request", { status: 500 });
  }
}
