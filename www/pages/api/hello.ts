// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import ffi from "ffi"
import path from 'path';

export type Data = {
  num: number
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const lib = ffi.Library(path.join(__dirname, '../target/release/math-eval.dll'), {
    sum_two: ['int', 'int', ['int']]
  });

  const num = lib.sum_two(2, 5);

  res.status(200).json({ num })
}
