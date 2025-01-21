import {
  createContext,
  useContext,
  useReducer,
  ReactNode,
  useEffect,
} from "react";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";

interface ChatState {
  messages: ChatCompletionMessageParam[];
  error: string | null;
}

type ChatAction =
  | { type: "ADD_MESSAGE"; payload: ChatCompletionMessageParam }
  | { type: "SET_ERROR"; payload: string | null }
  | { type: "CLEAR_CHAT" }
  | { type: "LOAD_MESSAGES"; payload: ChatCompletionMessageParam[] };

const STORAGE_KEY = "chat_messages";

const loadStoredMessages = (): ChatCompletionMessageParam[] => {
  if (typeof window === "undefined") return [];
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    return saved ? JSON.parse(saved) : [];
  } catch (error) {
    console.error("Error loading messages:", error);
    return [];
  }
};

const initialState: ChatState = {
  messages: [],
  error: null,
};

const chatReducer = (state: ChatState, action: ChatAction): ChatState => {
  switch (action.type) {
    case "ADD_MESSAGE": {
      const newMessages = [...state.messages, action.payload];
      if (typeof window !== "undefined") {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(newMessages));
      }
      return {
        ...state,
        messages: newMessages,
        error: null,
      };
    }
    case "SET_ERROR":
      return {
        ...state,
        error: action.payload,
      };
    case "CLEAR_CHAT": {
      if (typeof window !== "undefined") {
        localStorage.removeItem(STORAGE_KEY);
      }
      return initialState;
    }
    case "LOAD_MESSAGES":
      return {
        ...state,
        messages: action.payload,
      };
    default:
      return state;
  }
};

const ChatContext = createContext<{
  state: ChatState;
  dispatch: React.Dispatch<ChatAction>;
} | null>(null);

export const ChatProvider = ({ children }: { children: ReactNode }) => {
  const [state, dispatch] = useReducer(chatReducer, initialState);

  useEffect(() => {
    const savedMessages = loadStoredMessages();
    if (savedMessages.length > 0) {
      dispatch({ type: "LOAD_MESSAGES", payload: savedMessages });
    }
  }, []);

  return (
    <ChatContext.Provider value={{ state, dispatch }}>
      {children}
    </ChatContext.Provider>
  );
};

export const useChat = () => {
  const context = useContext(ChatContext);
  if (!context) {
    throw new Error("useChat must be used within a ChatProvider");
  }
  return context;
};
