import { createMemoryHistory, createRouter } from 'vue-router'
import Index from './pages/Index.vue'
import Board from "./pages/Board.vue";
import Main from "./pages/Main.vue";
import Multiplayer from './pages/Multiplayer.vue';
import Singleplayer from './pages/Singleplayer.vue';
import Settings from './pages/Settings.vue';
import ProfilePage from './pages/ProfilePage.vue';

const routes = [
    { path: '/', component: Index },
    { path: '/main', component: Main },
    { path: '/multiplayer', component: Multiplayer },
    { path: '/singleplayer', component: Singleplayer },
    { path: '/settings', component: Settings },
    { path: '/profile', component: ProfilePage },
    { path: '/classic', component: Board },
    { path: '/lines', component: Board },
    { path: '/blitz', component: Board },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

