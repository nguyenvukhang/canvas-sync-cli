import { Dispatch, SetStateAction, useEffect, useState } from 'react'

/**
 * O(1) delete from unsorted array
 *
 * @param {T[]} arr
 * @param {number} index
 * @returns {T}
 */
function quickpop<T>(arr: T[], index: number): T {
  if (arr.length === 0) throw new Error('Tried to quickpop an empty array')
  if (index >= arr.length || index < 0) throw new Error('Out of bounds')
  const res = arr[index]
  const elem = arr.pop()
  if (!elem) throw new Error('Quickpop somehow popped an undefined element')
  if (arr.length !== index) arr[index] = elem
  return res
}

type SetState<T> = Dispatch<SetStateAction<T>>

/**
 * Parses a canvas course folders url into course id and remote path.
 */
const parseUrl = (url: string) => {
  if (url.startsWith('https://')) url = url.slice(8)
  if (url.startsWith('canvas.nus.edu.sg/courses/')) url = url.slice(26)
  if (url.length === 0) return [NaN, '']
  // at this point, the url is left with either
  // 1. <courseId>/files
  // 2. <courseId>/files/folder/...
  let [courseId, remotePath] = url.split('/files', 2)
  if (remotePath && remotePath.startsWith('/folder/')) {
    remotePath = remotePath.slice(8)
  }
  return [courseId, remotePath]
}

const EnterUrl = (props: { setUrl: SetState<string>; done: () => void }) => {
  return (
    <div className="flex flex-row">
      <input
        className="flex-1"
        defaultValue="https://canvas.nus.edu.sg/courses/36732/files/folder/Lecture%20Notes"
        placeholder="canvas folder url"
        onChange={(e) => props.setUrl(e.target.value)}
      />
      <button onClick={props.done}>close</button>
    </div>
  )
}

const TrackedCanvasFolder = (props: { token: string }) => {
  const [url, setUrl] = useState('')
  const [edit, setEdit] = useState(false)
  const [response, setResponse] = useState<any>({})
  useEffect(() => {
    const [courseId, remotePath] = parseUrl(url)
    const url = `https://canvas.nus.edu.sg/api/v1/courses/${courseId}`
    fetch(`/api/canvas/any?url=${url}`)
      .then((e) => e.json())
      .then((e) => setResponse(e))
  }, [url])
  return (
    <div className="text-sm w-full border flex flex-col">
      <div className="flex flex-row">
        {!edit ? <button onClick={() => setEdit(true)}>edit url</button> : null}
      </div>
      {edit ? <EnterUrl setUrl={setUrl} done={() => setEdit(false)} /> : null}
    </div>
  )
}

function Input(
  props: JSX.IntrinsicElements['input'] & {
    setValue: SetState<string>
  }
) {
  const css = 'bg-gray-50 border border-gray-300 py-0.5 px-1 rounded-md'
  let { setValue, className, ...rest } = props
  className = className ? `${css} ${className}` : css
  return (
    <input
      {...rest}
      className={className}
      onChange={(e) => props.setValue(e.target.value)}
    />
  )
}

type Course = {
  id: number
  name: string
}

type CourseData = {
  folders: Folder[]
  files: File[]
}

type Folder = {
  id: number
  name: string
}

type File = {
  id: number
  filename: string
}

const Courses = (props: { token: string }) => {
  const [response, setResponse] = useState<any[]>([])
  const [courseData, setCourseData] = useState<Record<number, CourseData>>({})
  const [selectedCourses, setSelectedCourses] = useState<number[]>([])

  // fetch courses for the first time
  useEffect(() => {
    const url = 'https://canvas.nus.edu.sg/api/v1/courses?per_page=420'
    fetch(`/api/canvas/any?url=${url}&token=${props.token}`)
      .then((e) => e.json())
      .then((e) => setResponse(e))
  }, [])

  // fetch new course if not fetched yet
  useEffect(() => {
    selectedCourses.forEach((courseId) => {
      if (courseData[courseId]) return
      const url = (t: string) =>
        `https://canvas.nus.edu.sg/api/v1/courses/${courseId}/${t}`
      const a = fetch(
        `/api/canvas/any?url=${url('folders')}&token=${props.token}`
      ).then((e) => e.json())
      const b = fetch(
        `/api/canvas/any?url=${url('files')}&token=${props.token}`
      ).then((e) => e.json())
      Promise.all([a, b]).then(([folders, files]) => {
        const fresh = {
          ...courseData,
          [courseId]: {
            files,
            folders,
          },
        }
        setCourseData(fresh)
      })
    })
  }, [selectedCourses])
  const pretty: Course[] = response
    .map((v) => ({ id: parseInt(v['id']), name: v['name'] }))
    .filter((v) => Boolean(v['name']))

  const addCourse = (id: number) => {
    const tmp = [...selectedCourses]
    const idx = selectedCourses.indexOf(id)
    if (idx === -1) {
      tmp.push(id)
      setSelectedCourses(tmp)
    } else {
      quickpop(tmp, idx)
      setSelectedCourses(tmp)
    }
  }
  const removeCourse = (id: number) => {
    const idx = selectedCourses.indexOf(id)
    console.log('found', idx, selectedCourses)
    if (idx === -1) return
    const tmp = [...selectedCourses]
    quickpop(tmp, idx)
    setSelectedCourses(tmp)
  }

  const hasFiles = (courseId: number) =>
    courseData[courseId] && courseData[courseId].files.length > 0
  const hasFolders = (courseId: number) =>
    courseData[courseId] && courseData[courseId].folders.length > 0

  return (
    <div>
      {pretty.map((course, i) => {
        return (
          <div key={i} className="flex flex-col">
            <div className="flex flex-row">
              <input
                onChange={(e) => {
                  const on = e.target.checked
                  console.log(on)
                  if (on) {
                    console.log('add:', course.name)
                    addCourse(course.id)
                  } else {
                    console.log('remove:', course.name)
                    removeCourse(course.id)
                  }
                }}
                className="mr-2"
                type="checkbox"
              />
              <div className="truncate">{course.name}</div>
            </div>
            {hasFolders(course.id) && selectedCourses.includes(course.id)
              ? courseData[course.id].folders.map((f) => {
                  return <div>{f.name}</div>
                })
              : null}
            {hasFiles(course.id) && selectedCourses.includes(course.id)
              ? courseData[course.id].files.map((f) => {
                  return <div>{f.filename}</div>
                })
              : null}
          </div>
        )
      })}
    </div>
  )
}

const TokenDialog = (props: {
  setAuth: SetState<boolean>
  tokenState: [string, SetState<string>]
}) => {
  const [token, setToken] = props.tokenState
  const [response, setResponse] = useState<any>({})
  useEffect(() => {
    fetch(`/api/canvas/profile?token=${token}`)
      .then((e) => e.json())
      .then((e) => {
        if (e['id'] && e['name']) props.setAuth(true)
        else setResponse(e)
      })
  }, [token])
  return (
    <div>
      Enter your canvas token: <Input setValue={setToken} />
      <div>token: {token}</div>
      <div>JSON{JSON.stringify(response)}...</div>
    </div>
  )
}

/**
 * Centered, responsive layout
 */
const MainLayout = (props: JSX.IntrinsicElements['div']) => (
  <div className="flex justify-center w-full">
    <div className="flex flex-col w-full max-w-full sm:max-w-3xl px-8 sm:px-12 md:px-14">
      <div className="overflow-x-auto border">{props.children}</div>
    </div>
  </div>
)

export default function Home() {
  const [auth, setAuth] = useState(false)
  const [token, setToken] = useState('')
  return (
    <MainLayout>
      {auth ? (
        <div className="px-2 py-10">
          <Courses token={token} />
          {/* <TrackedCanvasFolder token={token} /> */}
        </div>
      ) : (
        <TokenDialog setAuth={setAuth} tokenState={[token, setToken]} />
      )}
    </MainLayout>
  )
}
