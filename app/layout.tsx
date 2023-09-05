import './globals.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'

const inter = Inter({ subsets: ['latin'] })

// export const metadata: Metadata = {
//   // title: 'Create Next App',
//   // description: 'Generated by create next app',
// }

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="flex min-h-screen flex-col items-center content-center p-24">
          <div className="table grow justify-center">
          <div className="table-cell align-middle">

            {children}

          </div>
          </div>
        </div>
      </body>
    </html>
  )
}