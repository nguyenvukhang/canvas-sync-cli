import type { NextApiRequest, NextApiResponse } from 'next'
import axios from 'axios'

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const url = req.query['url']

  if (typeof url !== 'string') return res.send('{ "message": "Invalid Url" }')
  if (url.length === 0) return res.send('{ "message": "Invalid Url" }')
  console.log('canvassing url:', url)
  return axios
    .get(url, { headers: { Authorization: `Bearer ${req.query['token']}` } })
    .then((e) => {
      return res.send(e.data)
    })
    .catch(() => res.send('{ "message": "Invalid Response" }'))
}
