import 'external'
import { foo } from './static.js'

export const result = [foo, import('./dynamic.js')]

export * from './static.js'