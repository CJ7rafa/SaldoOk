import { mount } from 'svelte'
import './app.css'
import Menu from './Menu.svelte'

const app = mount(Menu, {
  target: document.getElementById('app')!,
})

export default app
