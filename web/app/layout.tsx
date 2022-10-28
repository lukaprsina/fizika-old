export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html>
      <head><title>Layout shiz</title></head>
      <body>{children}</body>
    </html>
  )
}
