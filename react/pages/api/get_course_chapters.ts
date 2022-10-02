// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

export type ChapterInfo = {
  heading: string;
  author: string;
  goals: string;
}

export type Data = [
  ChapterInfo
]

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const fs = require('fs');
  const data = JSON.parse(fs.readFileSync('../rust/chapter_infos.txt', 'utf8')) as Data;
  res.status(200).json(data)
}
