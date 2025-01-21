"use client";
import {
  useRef,
  useEffect,
  useOptimistic,
  useTransition,
  useState,
} from "react";
import { Send, MessagesSquare, User } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { ChatProvider, useChat } from "@/app/chat-context";
import { sendChatMessage } from "@/app/chat-action";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";

import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeHighlight from "rehype-highlight";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { components } from "./markdown";

const ChatMessage = ({ message }: { message: ChatCompletionMessageParam }) => {
  const isUser = message.role === "user";
  const messageContent =
    typeof message.content === "string" ? message.content : "";

  if (isUser) {
    return (
      <div className="flex items-end justify-end mb-6 gap-3 text-sm">
        <Card className="bg-primary text-primary-foreground">
          <CardContent className="py-2 px-4">
            <p className="whitespace-pre-wrap">{messageContent}</p>
          </CardContent>
        </Card>
        <Avatar>
          <AvatarFallback>
            <User />
          </AvatarFallback>
        </Avatar>
      </div>
    );
  }

  return (
    <div className="flex items-start mb-6 gap-3 text-sm">
      <Avatar>
        <AvatarFallback>AI</AvatarFallback>
      </Avatar>
      <Card className="bg-secondary text-secondary-foreground">
        <CardContent className="py-2 px-4">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            rehypePlugins={[rehypeHighlight]}
            components={components}
          >
            {messageContent}
          </ReactMarkdown>
        </CardContent>
      </Card>
    </div>
  );
};

const ChatInput = () => {
  const { state, dispatch } = useChat();
  const [isPending, startTransition] = useTransition();
  const [input, setInput] = useState("");
  const [, addOptimisticMessage] = useOptimistic<
    ChatCompletionMessageParam[],
    ChatCompletionMessageParam
  >(state.messages, (state, newMessage) => [...state, newMessage]);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!input.trim() || isPending) return;

    const userMessage: ChatCompletionMessageParam = {
      role: "user",
      content: input.trim(),
    };

    setInput("");

    startTransition(async () => {
      addOptimisticMessage(userMessage);

      try {
        const response = await sendChatMessage([
          ...state.messages,
          userMessage,
        ]);
        dispatch({ type: "ADD_MESSAGE", payload: userMessage });
        dispatch({ type: "ADD_MESSAGE", payload: response });
      } catch {
        dispatch({ type: "SET_ERROR", payload: "Failed to send message" });
      }
    });
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="flex gap-2 px-4 py-3 border-t bg-background"
    >
      <Textarea
        value={input}
        autoFocus
        onChange={(e) => setInput(e.target.value)}
        placeholder="Type your message..."
        className="resize-none flex-grow shadow-lg h-20"
        rows={1}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            handleSubmit(e);
          }
        }}
      />
      <Button type="submit" size="icon" disabled={isPending}>
        <Send className={`h-4 w-4 ${isPending ? "animate-pulse" : ""}`} />
      </Button>
    </form>
  );
};

export const ChatContainer = () => {
  const { state } = useChat();
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [state.messages]);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-hidden">
        <ScrollArea className="h-full w-full">
          <div className="py-6">
            {state.messages.map((message, index) => (
              <ChatMessage key={index} message={message} />
            ))}
            <div ref={bottomRef} />
          </div>
        </ScrollArea>
      </div>
      {state.error && (
        <Alert variant="destructive" className="mx-6 mb-4">
          <AlertDescription>{state.error}</AlertDescription>
        </Alert>
      )}
      <div className="sticky bottom-0">
        <ChatInput />
      </div>
    </div>
  );
};

const ChatLayout = () => {
  const { dispatch } = useChat();

  const handleNewChat = () => {
    dispatch({ type: "CLEAR_CHAT" });
  };

  return (
    <div className="flex flex-col h-screen bg-background max-w-5xl mx-auto">
      <nav className="flex flex-row justify-between w-full border-b py-4 px-4">
        <div className="text-2xl font-medium">Chat-bot</div>
        <Button onClick={handleNewChat}>
          <MessagesSquare className="mr-2 h-4 w-4" /> New Chat
        </Button>
      </nav>
      <main className="flex-1 flex flex-col mx-auto w-full relative">
        <ChatContainer />
      </main>
    </div>
  );
};

export const Chat = () => {
  return (
    <ChatProvider>
      <ChatLayout />
    </ChatProvider>
  );
};
