// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "./ui/table";
import Link from "next/link";
import { useState } from "react";
import { Components } from "react-markdown";

export const components: Partial<Components> = {
  code: CodeBlock,
  pre: ({ children }) => <>{children}</>,
  ol: ({ node, children, ...props }) => {
    return (
      <ol className="list-decimal list-outside ml-4" {...props}>
        {children}
      </ol>
    );
  },
  li: ({ node, children, ...props }) => {
    return (
      <li className="py-1" {...props}>
        {children}
      </li>
    );
  },
  ul: ({ node, children, ...props }) => {
    return (
      <ul className="list-decimal list-outside ml-4" {...props}>
        {children}
      </ul>
    );
  },
  strong: ({ node, children, ...props }) => {
    return (
      <span className="font-semibold" {...props}>
        {children}
      </span>
    );
  },
  a: ({ node, children, ...props }) => {
    return (
      <Link
        className="text-blue-500 hover:underline"
        target="_blank"
        rel="noreferrer"
        {...props}
      >
        {children}
      </Link>
    );
  },
  h1: ({ node, children, ...props }) => {
    return (
      <h1 className="text-3xl font-semibold mt-6 mb-2" {...props}>
        {children}
      </h1>
    );
  },
  h2: ({ node, children, ...props }) => {
    return (
      <h2 className="text-2xl font-semibold mt-6 mb-2" {...props}>
        {children}
      </h2>
    );
  },
  h3: ({ node, children, ...props }) => {
    return (
      <h3 className="text-xl font-semibold mt-6 mb-2" {...props}>
        {children}
      </h3>
    );
  },
  h4: ({ node, children, ...props }) => {
    return (
      <h4 className="text-lg font-semibold mt-6 mb-2" {...props}>
        {children}
      </h4>
    );
  },
  h5: ({ node, children, ...props }) => {
    return (
      <h5 className="text-base font-semibold mt-6 mb-2" {...props}>
        {children}
      </h5>
    );
  },
  h6: ({ node, children, ...props }) => {
    return (
      <h6 className="text-sm font-semibold mt-6 mb-2" {...props}>
        {children}
      </h6>
    );
  },
  table: ({ children }) => {
    return (
      <div className="my-4 w-full">
        <Table>{children}</Table>
      </div>
    );
  },
  thead: ({ children }) => {
    return <TableHeader>{children}</TableHeader>;
  },
  tbody: ({ children }) => {
    return <TableBody>{children}</TableBody>;
  },
  tr: ({ children }) => {
    return <TableRow>{children}</TableRow>;
  },
  th: ({ children }) => {
    return <TableHead className="font-medium">{children}</TableHead>;
  },
  td: ({ children }) => {
    return <TableCell>{children}</TableCell>;
  },
};

interface CodeBlockProps {
  node: any;
  inline: boolean;
  className: string;
  children: any;
}

function CodeBlock({
  node,
  inline,
  className,
  children,
  ...props
}: CodeBlockProps) {
  const [output, setOutput] = useState<string | null>(null);
  const [pyodide, setPyodide] = useState<any>(null);
  const match = /language-(\w+)/.exec(className || "");
  const isPython = match && match[1] === "python";
  const codeContent = String(children).replace(/\n$/, "");
  const [tab, setTab] = useState<"code" | "run">("code");

  if (!inline) {
    return (
      <div className="not-prose flex flex-col">
        {tab === "code" && (
          <pre {...props} className={`text-sm w-full overflow-x-auto `}>
            <code className="whitespace-pre-wrap break-words">{children}</code>
          </pre>
        )}
        {tab === "run" && output && (
          <div className="text-sm w-full overflow-x-auto ">
            <code>{output}</code>
          </div>
        )}
      </div>
    );
  } else {
    return (
      <code className={`${className} text-sm  rounded-md`} {...props}>
        {children}
      </code>
    );
  }
}
