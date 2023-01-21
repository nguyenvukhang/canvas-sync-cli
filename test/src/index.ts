import type { AxiosInstance } from 'axios'
import axios from 'axios'
import { Course } from './types'

import { config } from 'dotenv'
config({ path: '../.env' })

const TOKEN = process.env['ADMIN_TOKEN'] || ''
const BASE_URL = 'https://canvas.instructure.com/api/v1/'
const ACCOUNT_ID = 81259

/**
 * Get a range of values from `s` (inclusive) to `e` (exclusive)
 */
export const range = (s: number, e: number): number[] =>
  e > s ? Array.apply(null, Array(e - s)).map((_, i) => s + i) : []

class Api {
  core: AxiosInstance

  constructor(core: AxiosInstance) {
    this.core = core
  }

  async self() {
    return this.core.get('/users/self').then((e) => e.data)
  }

  async profile() {
    return this.core.get('/users/self/profile').then((e) => e.data)
  }

  async accounts() {
    return this.core.get('/accounts').then((e) => e.data)
  }

  async listCourses(): Promise<Course[]> {
    return this.core
      .get('/courses', {
        params: { per_page: 200 },
      })
      .then((e) => e.data.map((e: any) => ({ id: e.id, name: e.name })))
  }

  async deleteCourse(courseId: number) {
    return this.core
      .delete(`/courses/${courseId}`, { params: { event: 'delete' } })
      .then((e) => ({ delete: e.data.delete as boolean } as const))
  }

  async deleteAllCourses() {
    return this.listCourses().then((courses) =>
      Promise.all(courses.map((course) => this.deleteCourse(course.id)))
    )
  }

  async createCourse(name: string) {
    const params = {
      course: { name, default_view: 'modules' },
      enroll_me: true,
      offer: true,
    }
    return this.core
      .post(`/accounts/${ACCOUNT_ID}/courses`, {}, { params })
      .then((e) => e.data)
  }

  async get(url: string) {
    return this.core
      .get(url)
      .then((e) => e.data)
      .catch(() => ({ error: 'bad request' }))
  }
}

export const api = new Api( axios.create({
    baseURL: BASE_URL,
    headers: { Authorization: `Bearer ${TOKEN}` },
  })
)

export async function experiments() {
  // api.createCourse("CS1010 Version 1").then(console.log)
  const creates = range(0, 10).map((i) =>
    api.createCourse(`CS1010 Version ${i}`)
  )
  const result = await Promise.all([creates])
  console.log(result)
}
