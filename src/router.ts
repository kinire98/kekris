import { createMemoryHistory, createRouter } from 'vue-router'
import Index from './pages/Index.vue'
import Board from "./pages/Board.vue";
import Main from "./pages/Main.vue";

const routes = [
    { path: '/', component: Index },
    { path: '/board', component: Board },
    { path: '/main', component: Main },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

