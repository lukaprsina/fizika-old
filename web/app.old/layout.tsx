import { PrismaClient } from "@prisma/client"
import Head from "next/head"
import EmotionRootStyleRegistry from "./EmotionRootStyleRegistry";

async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html>
      <Head>
        <title>Layout shiz</title>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>
      <body>
        <EmotionRootStyleRegistry>
          {children}
        </EmotionRootStyleRegistry>
      </body>
    </html>
  )
}

export default RootLayout;