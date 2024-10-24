import Sidebar from "./sidebar"

import ApolloServerWrapper from "@/lib/apollo-client/server-wrapper"

const AppLayout: React.FC<React.PropsWithChildren> = async ({ children }) => {
  return (
    <ApolloServerWrapper>
      <div className="flex flex-col md:flex-row h-screen">
        <Sidebar />
        <div className="flex-1 overflow-auto p-2">{children}</div>
      </div>
    </ApolloServerWrapper>
  )
}

export default AppLayout
