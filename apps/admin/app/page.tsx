import { getServerSession } from "next-auth"
import { redirect } from "next/navigation"

import { authOptions } from "./api/auth/[...nextauth]/options"

const Home = async () => {
  const session = await getServerSession(authOptions)
  if (!session) {
    redirect("/api/auth/signin")
  }

  return <div>Home</div>
}

export default Home
