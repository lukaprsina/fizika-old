// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import fs from 'fs'

export type Data = {
  file: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  if (typeof req.query.course != 'string' || typeof req.query.page != 'string') {
    res.status(400);
    return;
  }

  const file_path = `./courses/${req.query.course}/pages/page_${req.query.page}/index.html`;
  if (fs.existsSync(file_path)) {
    const data = fs.readFileSync(file_path, 'utf8');
    res.status(200).json({ file: data })
  } else {
    res.status(400);
    return;
  }
}
