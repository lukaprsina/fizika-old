import { PrismaClient } from "@prisma/client"
import Head from "next/head"

async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const prisma = new PrismaClient();
  const users = await prisma.user.findMany()

  console.log(users)
  return (
    <html>
      <Head>
        <title>Layout shiz</title>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>
      <body>
        <div>
          {users.map(user => <p key={user.id}>{user.name}</p>)}
        </div>
        {children}
      </body>
    </html>
  )
}

export default RootLayout;