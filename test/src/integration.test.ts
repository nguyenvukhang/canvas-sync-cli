import { expect, test } from '@jest/globals'
import { api, range } from '.'

test('end-to-end asf', async () => {
  const self = await api.self()
  expect(self).toHaveProperty('id')
  expect(self).toHaveProperty('name')

  await api.deleteAllCourses()
  const creates = range(0, 20).map((i) => api.createCourse(`C${i}`))
  await Promise.all(creates)

  const courses = await api.listCourses()
  expect(courses).toHaveLength(20)
  console.log(courses)
})
