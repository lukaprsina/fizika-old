// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import fs from 'fs'

export type CourseInfo = {
  heading: string;
  author: string;
  goals: string;
}

export type Data = [
  CourseInfo
]

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const data = JSON.parse(fs.readFileSync('./chapter_infos.json', 'utf8')) as Data;
  res.status(200).json(data)
}
