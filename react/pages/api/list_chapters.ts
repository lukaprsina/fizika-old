// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import fs from 'fs'

export type CourseInfo = {
  subheading: string;
}

export type Data = [
  CourseInfo
]

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {

  if (typeof req.query.course != 'string') {
    res.status(400);
    return;
  }

  let course_number = parseInt(req.query.course)
  if (isNaN(course_number)) {
    res.status(400);
    return;
  }

  let i = 0;
  let arr: CourseInfo[] = [];

  while (true) {
    const file_path = `./courses/${req.query.course}/pages/page_${i}/config.json`

    if (fs.existsSync(file_path)) {
      let file = fs.readFileSync(file_path, 'utf8');
      const data = JSON.parse(file) as CourseInfo;
      arr.push(data);
    } else {
      break;
    }

    i++;
  }

  res.status(200).json(arr as Data)
}
