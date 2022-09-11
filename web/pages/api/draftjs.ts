// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { writeFile } from 'fs/promises'

type Data = {
  message: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  try {
    const controller = new AbortController();
    const { signal } = controller;
    const data = new Uint8Array(Buffer.from(JSON.stringify(req.body, null, 4)));
    const promise = writeFile('message.json', data, { signal });

    await promise;
    res.status(200).json({ message: "Ok" })
  } catch (err) {
    // When a request is aborted - err is an AbortError
    console.error(err);
    res.status(500)
  }
}
