import type { NextApiRequest, NextApiResponse } from 'next'
import axios from 'axios'

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  return axios
    .get('https://canvas.nus.edu.sg/api/v1/users/self/profile', {
      headers: { Authorization: `Bearer ${req.query['token']}` },
    })
    .then((e) => res.send(e.data))
    .catch(() => res.send('{ "message": "Invalid Response" }'))
}
