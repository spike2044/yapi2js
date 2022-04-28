import { isNil } from 'lodash-es'
import { request } from 'something'
import compilePath from './compilePath'

// 封装请求
type FirstLowerCase<T> = T extends string
  ? T extends `${infer U}${infer V}`
    ? `${Lowercase<U>}${V}`
    : T
  : T

export const api = {
  get: (path: string) =>
    function <T extends Record<string, any>>(params: T) {
      if (!isNil(params.spaceId)) {
        Object.assign(params, { space_id: params.spaceId })
        delete params.spaceId
      }
      const [uri] = compilePath(path, params)
      return request({
        uri,
        method: 'GET',
      })
    },
  post:
    (path: string) =>
    <T extends Record<string, any>>(params: T) => {
      if (!isNil(params.spaceId)) {
        Object.assign(params, { space_id: params.spaceId })
        delete params.spaceId
      }
      const [uri, rest ] = compilePath(path, params)
      return request({
        uri,
        body: rest,
        method: 'POST',
      })
    },
}

const firstLowerCase = (str: string) =>
  str.slice(0, 1).toLocaleLowerCase() + str.slice(1)

export type ApiWithKey<T> = T extends Record<string, string[]>
  ? Record<
      FirstLowerCase<keyof T>,
      (d: Record<string, any>) => PromiseLike<any>
    >
  : never

export function apiConfig<T extends Record<string, string[]>>(
  config: T
): ApiWithKey<T> {
  return Object.entries(config).reduce((acc, [key, [method, path]]) => {
    if (api[method as 'get']) {
      acc[firstLowerCase(key)] = api[method as 'get'](path)
    }
    return acc
  }, {} as any)
}
