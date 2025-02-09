import { writable } from 'svelte/store'

// Create stores for reactive state
export const connected = writable(false)
export const address = writable('')
